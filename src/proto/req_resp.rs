use std::convert::TryFrom;

use prost::Message;

use crate::error::Error;

use super::*;

impl Request {
    /// Build a new request to get the account.
    pub fn new_account_req(account_name: Vec<u8>) -> Self {
        Self {
            method: Method::GetAccount as i32,
            body: Some(request::Body::GetAccountReq(GetAccountReq { account_name } )),
        }
    }
}

impl Response {
    /// Build a new response to get the account info.
    pub fn new_get_account_resp(block_height: u64) -> Self {
        Self {
            method: Method::GetAccount as i32,
            body: Some(response::Body::GetAccountResp(GetAccountResp {
            })),
        }
    }
}


impl TryFrom<Vec<u8>> for Request {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::decode(value.as_slice())?)
    }
}

impl From<Request> for Vec<u8> {
    fn from(value: Request) -> Self {
        value.encode_to_vec()
    }
}

impl TryFrom<Vec<u8>> for Response {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::decode(value.as_slice())?)
    }
}

impl From<Response> for Vec<u8> {
    fn from(value: Response) -> Self {
        value.encode_to_vec()
    }
}

impl From<Response> for GetAccountResp {
    fn from(value: Response) -> Self {
        match value.body.unwrap() {
            response::Body::GetAccountResp(resp) => resp,
            _ => GetAccountResp {  },
        }
    }
}

impl From<Response> for BlocksResp {
    fn from(value: Response) -> Self {
        match value.body.unwrap() {
            response::Body::BlocksResp(resp) => resp,
            _ => BlocksResp { blocks: vec![] },
        }
    }
}