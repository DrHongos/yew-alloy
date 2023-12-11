use alloy_primitives::{keccak256, Address};
use elliptic_curve::sec1::ToEncodedPoint;
use k256::{
    ecdsa::{SigningKey, VerifyingKey},
    AffinePoint,
};
use wasm_bindgen::prelude::*;
// HELPERS
use ruint::{
    Uint,
    aliases::U256
};

pub fn format_gas(val: U256) -> usize {
    match val.checked_div(Uint::from(1000000000)) {
        Some(v) => v.try_into().unwrap(),
        None => 0
    }
}

pub fn format_eth(val: U256) -> f64 {
    let v: f64 = val.into();
    v/1000000000000000000f64        // parses to 18 decimals
}

/// Applies [EIP-155](https://eips.ethereum.org/EIPS/eip-155).
#[inline]
pub const fn to_eip155_v(v: u8, chain_id: u64) -> u64 {
    (v as u64) + 35 + chain_id * 2
}

/// Converts an ECDSA public key to its corresponding Ethereum address.
#[inline]
pub fn public_key_to_address(pubkey: &VerifyingKey) -> Address {
    let affine: &AffinePoint = pubkey.as_ref();
    let encoded = affine.to_encoded_point(false);
    raw_public_key_to_address(&encoded.as_bytes()[1..])
}

#[inline]
#[track_caller]
pub fn raw_public_key_to_address(pubkey: &[u8]) -> Address {
    assert_eq!(pubkey.len(), 64, "raw public key must be 64 bytes");
    let digest = keccak256(pubkey);
    Address::from_slice(&digest[12..])
}

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace=["console"])]
    pub fn log(value: &str);    
}