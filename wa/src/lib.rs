#![deny(unsafe_code)]

use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use thiserror::Error;

mod binary;
mod constants;
#[rustfmt::skip]
mod proto;

pub use binary::node::{Node, NodeContent};
pub use proto::WebMessageInfo;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    InvalidKeyIvLength(#[from] block_modes::InvalidKeyIvLength),
    #[error(transparent)]
    BlockModeError(#[from] block_modes::BlockModeError),
    #[error(transparent)]
    MacError(#[from] crypto_mac::MacError),
    #[error(transparent)]
    InvalidKeyLength(#[from] crypto_mac::InvalidKeyLength),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

// General Result type for all functions
type Result<T> = std::result::Result<T, Error>;
// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;
// Create an alias for convenience
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn verify_and_decrypt_message(
    enc: &[u8],
    mac: &[u8],
    message_encrypted: &[u8],
) -> Result<Vec<u8>> {
    let mut h = HmacSha256::new_varkey(mac)?;
    debug_assert!(message_encrypted.len() > 48, "too short message");
    h.update(&message_encrypted[32..]);
    h.verify(&&message_encrypted[..32])?;
    aes_decrypt(enc, &message_encrypted[32..48], &message_encrypted[48..])
}

fn aes_decrypt(key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Cbc::new_var(&key, &iv)?;
    Ok(cipher.decrypt_vec(input)?)
}
