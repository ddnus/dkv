use std::time::SystemTime;

use crate::{auth::Authorization, error::Error, space::Space};


pub struct IdentityID {
}

pub struct Identity {
    identity_id: IdentityID,
}

/// Account storage
pub struct Account {
    pub identity: Identity,
    /// account name
    pub name: Vec<u8>,
    /// account pub key
    pub pubkey: Vec<u8>,
    /// login state
    pub login_state: bool,
    /// latest login time
    pub login_time: u64,
    /// account nonce
    pub nonce: u64,
    /// account version
    pub version: u64,
    /// login auth code
    pub auth: Authorization,
    /// balance
    pub balance: u64,
    /// balance freeze
    pub freeze_out_balance: u64,

    pub freeze_in_balance: u64,

    pub spaces: Vec<Space>,
}

impl Account {

    fn check_authorization(&self, authorization_code: Vec<u8>) -> Result<bool, Error> {
        if SystemTime::now().as_secs() > self.auth_exp_time {
            return Err(Error::SpaceAuthorization("authorization code expiration".to_string()))
        }

        if !authorization_code.eq(&self.auth_code) {
            return Err(Error::SpaceAuthorization("authorization code invalid".to_string()))
        }
        
        Ok(true)
    }
}
