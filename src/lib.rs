extern crate console_error_panic_hook;
extern crate wasm_bindgen;
extern crate schnorrkel;
extern crate js_sys;
extern crate hex;

use wasm_bindgen::prelude::*;

use schnorrkel::{MiniSecretKey, signing_context};
use std::ops::Add;
use hex::FromHex;

#[wasm_bindgen]
pub enum SignError {
    InvalidCouponSecret = "InvalidCouponSecret",
    InvalidAddress = "InvalidAddress",
    InvalidHex = "InvalidHex",
}

/// accept addresses in hex format & coupon secret key in hex, and generate signature for coupon
#[wasm_bindgen]
pub fn get_coupon_signature(contract_address_hex: &str, receiver_address_hex: &str, coupon_secret_hex: &str) -> Result<String, JsValue> {
    console_error_panic_hook::set_once();

    // Parse addresses
    let contract_hex = contract_address_hex.strip_prefix("0x").ok_or(SignError::InvalidHex)?;
    let contract_bytes = <[u8; 32]>::from_hex(&*contract_hex).or(Err(SignError::InvalidHex))?;

    let receiver_hex = receiver_address_hex.strip_prefix("0x").ok_or(SignError::InvalidHex)?;
    let receiver_bytes = <[u8; 32]>::from_hex(&*receiver_hex).or(Err(SignError::InvalidHex))?;

    // Coupon private key
    let coupon_hex = coupon_secret_hex.strip_prefix("0x").ok_or(SignError::InvalidHex)?;
    let coupon_secret_bytes = <[u8; 32]>::from_hex(&*coupon_hex).or(Err(SignError::InvalidHex))?;
    let coupon = MiniSecretKey::from_bytes(&coupon_secret_bytes).or(Err(SignError::InvalidCouponSecret))?;

    // // Make signature
    let coupon_secret_key = coupon.expand(MiniSecretKey::ED25519_MODE);
    let context = signing_context(&contract_bytes);

    let message = context.bytes(&receiver_bytes);
    let signature = coupon_secret_key.sign(message, &coupon_secret_key.to_public());
    let hex_signature = hex::encode(signature.to_bytes());

    Ok("0x".to_string().add(&hex_signature))
}
