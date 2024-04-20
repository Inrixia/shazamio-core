mod fingerprinting;

use std::error::Error;
use wasm_bindgen::prelude::*;
use fingerprinting::{algorithm::SignatureGenerator, signature_format::DecodedSignature};
use web_sys::js_sys::{Object, Reflect};

#[wasm_bindgen]
pub struct Recognizer;

#[wasm_bindgen]
pub struct Signature {
    samplems: u32,
    uri: String,
}

#[wasm_bindgen]
impl Signature {
    #[wasm_bindgen(constructor)]
    pub fn new(uri: String, samplems: u32) -> Signature {
        Signature { 
            samplems,
            uri,
        }
    }
    #[wasm_bindgen(getter)]
    pub fn uri(&self) -> String {
        self.uri.clone() // Return a clone of the string
    }
    #[wasm_bindgen(getter)]
    pub fn samplems(&self) -> u32 {
        self.samplems
    }
}

#[wasm_bindgen]
impl Recognizer {
    /// Recognizes an audio fingerprint from a byte slice and converts it into a JavaScript object.
    #[wasm_bindgen(js_name = "recognizeBytes")]
    pub fn recognize_bytes(bytes: Vec<u8>, offset: usize) -> Result<Signature, JsValue> {
        // console_error_panic_hook::set_once();
        let sig = SignatureGenerator::make_signature_from_bytes(bytes, offset).and_then(|sig| convert_to_sig(&sig));
        match sig {
            Ok(sig) => Ok(sig),
            Err(e) => return Err(JsValue::from_str(&e.to_string()))
        }
    }
}

fn convert_to_sig(decoded_sig: &DecodedSignature) -> Result<Signature, Box<dyn Error>> {
    Ok(Signature::new(decoded_sig.encode_to_uri()?, (decoded_sig.number_samples as f64 / decoded_sig.sample_rate_hz as f64 * 1000.0) as u32))
}

/// Converts a `DecodedSignature` into a JavaScript object.
fn convert_to_js(signature: &DecodedSignature) -> Result<JsValue, Box<dyn Error>> {
    let js_obj = Object::new();
    let milliseconds = (signature.number_samples as f64 / signature.sample_rate_hz as f64 * 1000.0) as u32;
    let _ = Reflect::set(&js_obj, &JsValue::from_str("samplems"), &JsValue::from_f64(milliseconds as f64));
    let _ = Reflect::set(&js_obj, &JsValue::from_str("uri"), &JsValue::from_str(&signature.encode_to_uri()?));
    Ok(JsValue::from(js_obj))
}
