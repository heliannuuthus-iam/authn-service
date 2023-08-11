use derive_builder::Builder;
use num_bigint::BigInt;
use redis::FromRedisValue;
use ring::digest::{self, SHA256};
use tracing::info;

use super::param::SrpParams;

#[derive(Builder)]
pub struct SrpClient {
    pub credential: String,
    pub proof: String,
}

impl SrpClient {
    pub fn getX(&mut self, salt: &[u8]) -> Vec<u8> {
        let mut salt = salt.to_vec();
        let right: String = format!("{}:{}", self.credential, self.proof);
        let right_hash = digest::digest(&SHA256, right.as_bytes());
        salt.extend_from_slice(right_hash.as_ref());
        digest::digest(&SHA256, &salt).as_ref().to_vec()
    }
    pub fn verfier(&mut self, salt: &[u8], params: &SrpParams) -> Vec<u8> {
        let x_hex = hex::encode(self.getX(salt));
        let bigintx = BigInt::parse_bytes(x_hex.as_bytes(), 16).unwrap();
        params.padding(params.g.modpow(&bigintx, &params.n))
    }
}
