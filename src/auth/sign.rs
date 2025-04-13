use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::error::Error;

// 创建一个类型别名方便使用
type HmacSha256 = Hmac<Sha256>;

/// 使用 HMAC-SHA256 对消息进行签名
pub fn sign_message(secret: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    // 使用 secret 初始化 HMAC
    let mut mac = HmacSha256::new_from_slice(secret)?;
    
    // 输入消息
    mac.update(message);
    
    // 获取签名结果
    let result = mac.finalize();
    
    // 返回签名的字节数组
    Ok(result.into_bytes().to_vec())
}

/// 验证消息签名是否有效
pub fn verify_signature(secret: &[u8], message: &[u8], signature: &[u8]) -> Result<bool, Box<dyn Error>> {
    // 使用相同的 secret 和 message 生成签名
    let expected_signature = sign_message(secret, message)?;
    
    // 比较生成的签名和提供的签名
    Ok(expected_signature == signature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify() {
        let secret = b"my_super_secret_key";
        let message = b"important_message";
        
        // 签名
        let signature = sign_message(secret, message).unwrap();
        
        // 验证有效签名
        assert!(verify_signature(secret, message, &signature).unwrap());
        
        // 验证无效签名
        assert!(!verify_signature(secret, message, b"fake_signature").unwrap());
        
        // 验证使用不同 secret 的签名
        let wrong_secret = b"wrong_secret_key";
        assert!(!verify_signature(wrong_secret, message, &signature).unwrap());
        
        // 验证篡改后的消息
        let tampered_message = b"tampered_message";
        assert!(!verify_signature(secret, tampered_message, &signature).unwrap());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let secret = b"my_application_secret";
    let message = b"user_id=123&action=delete";
    
    // 签名
    let signature = sign_message(secret, message)?;
    println!("Signature (hex): {}", hex::encode(&signature));
    
    // 验证
    let is_valid = verify_signature(secret, message, &signature)?;
    println!("Signature valid: {}", is_valid);
    
    Ok(())
}