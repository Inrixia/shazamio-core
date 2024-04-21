mod fingerprinting;
use wasm_bindgen::prelude::*;
use fingerprinting::{algorithm::SignatureGenerator, signature_format::DecodedSignature};
use console_error_panic_hook;

/// Recognizes an audio fingerprint fron song bytes and returns decoded signatures.
/// # Arguments
/// * `bytes` - Bytes of the song file
/// * `offset` - When to start sampling from in seconds
/// * `seconds` - Seconds to sample from offset
#[wasm_bindgen(js_name = "recognizeBytes")]
pub fn recognize_bytes(bytes: Vec<u8>, offset: Option<usize>, seconds: Option<usize>) -> Result<Vec<DecodedSignature>, JsValue> {
    match SignatureGenerator::make_signature_from_bytes(bytes, offset, seconds) {
        Ok(sig) => Ok(sig),
        Err(e) => return Err(JsValue::from_str(&e.to_string()))
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}