use std::time::SystemTime;

mod sign;

pub enum Authorization {
    Sercret(Vec<u8>, u64),
}

impl Authorization {

    pub fn verify(&self, message: Vec<u8>, authorization_code: Vec<u8>) -> bool {
        match self {
            Authorization::Sercret(code, exp_time) => {
                if SystemTime::now().as_secs() > *exp_time {
                    return false;
                }

                if !sign::verify_signature(code, &message, &authorization_code).unwrap_or(false) {
                    return false;
                }
            }
        }
        true
    }

    pub fn auth_encode(&self, message: Vec<u8>) -> Vec<u8> {
        match self {
            Authorization::Sercret(code, exp_time) => {
                sign::sign_message(code, &code).unwrap_or(vec![])
            }
        }
    }
    
}