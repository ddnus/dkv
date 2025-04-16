use std::{path::Path, sync::Arc};
use serde::{Serialize, Deserialize};

use crate::{auth::Authorization, error::Error};

use super::{kv::KV, Space};


pub struct IdentityID {
}

pub struct Identity {
    identity_id: IdentityID,
}

/// Account storage
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Account {
    pub identity: Identity,
    /// account name
    pub name: Vec<u8>,
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

    fn check_authorization(&self, message: Vec<u8>, authorization_code: Vec<u8>) -> Result<bool, Error> {
        Ok(self.auth.verify(message, authorization_code))
    }

    fn encode(&self) -> Result<Vec<u8>, Error> {
        bincode::encode_to_vec(self, bincode::config::standard()).map_err(|e| Error::Serialization(format!("Failed to encode account: {}", e)))
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error> {
        let (decoded, _len) = bincode::decode_from_slice(bytes, bincode::config::standard()).unwrap();
        decoded
    }
}

pub struct Accounter {
    pub kv: Arc<KV>,
}

impl Accounter {
    
    pub fn new(path: Path) -> Accounter {
        Accounter {
            kv: Arc::new(KV::new(path)),
        }
    }

    pub fn get_account(&self, name: Vec<u8>) -> Result<Account, Error> {
        let account = self.kv.get(name)?;
        Account::decode(&account)
    }
    pub fn create_account(&self, name: Vec<u8>, account: Account) -> Result<(), Error> {
        let account = account.encode()?;
        self.kv.put(name, account)
    }
    pub fn delete_account(&self, name: Vec<u8>) -> Result<(), Error> {
        self.kv.delete(name)
    }
    pub fn update_account(&self, name: Vec<u8>, account: Account) -> Result<(), Error> {
        let account = account.encode()?;
        self.kv.put(name, account)
    }
    pub fn get_all_accounts(&self) -> Result<Vec<Account>, Error> {
        let mut accounts = Vec::new();
        for (key, value) in self.kv.range() {
            let account = Account::decode(&value)?;
            accounts.push(account);
        }
        Ok(accounts)
    }
    pub fn get_account_by_prefix(&self, prefix: &[u8]) -> Result<Vec<Account>, Error> {
        let mut accounts = Vec::new();
        for (key, value) in self.kv.prefix_search(prefix) {
            let account = Account::decode(&value)?;
            accounts.push(account);
        }
        Ok(accounts)
    }
    pub fn get_account_by_range(&self, start: &[u8], end: &[u8]) -> Result<Vec<Account>, Error> {
        let mut accounts = Vec::new();
        for (key, value) in self.kv.range_search(start, end) {
            let account = Account::decode(&value)?;
            accounts.push(account);
        }
        Ok(accounts)
    }
    pub fn get_account_by_key(&self, key: &[u8]) -> Result<Account, Error> {
        let value = self.kv.get(key)?;
        Account::decode(&value)
    }
    pub fn get_account_by_key_like(&self, pattern: &str) -> Result<Vec<Account>, Error> {
        let mut accounts = Vec::new();
        for (key, value) in self.kv.key_like(pattern) {
            let account = Account::decode(&value)?;
            accounts.push(account);
        }
        Ok(accounts)
    }
    pub fn get_account_by_key_range(&self, start: &[u8], end: &[u8]) -> Result<Vec<Account>, Error> {
        let mut accounts = Vec::new();
        for (key, value) in self.kv.range_search(start, end) {
            let account = Account::decode(&value)?;
            accounts.push(account);
        }
        Ok(accounts)
    }
    pub fn get_account_by_key_prefix(&self, prefix: &[u8]) -> Result<Vec<Account>, Error> {
        let mut accounts = Vec::new();
        for (key, value) in self.kv.prefix_search(prefix) {
            let account = Account::decode(&value)?;
            accounts.push(account);
        }
        Ok(accounts)
    }

    pub fn get_account_by_key_like_prefix(&self, pattern: &str, prefix: &[u8]) -> Result<Vec<Account>, Error> {
        let mut accounts = Vec::new();
        for (key, value) in self.kv.key_like(pattern) {
            if key.starts_with(prefix) {
                let account = Account::decode(&value)?;
                accounts.push(account);
            }
        }
        Ok(accounts)
    }
}

pub struct AccountManger {
}

impl AccountManger {

    pub fn new() -> AccountManger {
        AccountManger {}
    }
    
}