mod fingerprinting;
use std::error::Error;

use wasm_bindgen::prelude::*;
use fingerprinting::{decode::samples_from_bytes, decoded_signature::DecodedSignature, resample::resample};
use console_error_panic_hook;

/// Recognizes an audio fingerprint fron song bytes and returns decoded signatures.
/// # Arguments
/// * `bytes` - Bytes of the song file
/// * `offset` - When to start sampling from in seconds
/// * `seconds` - Seconds to sample from offset
#[wasm_bindgen(js_name = "recognizeBytes")]
pub fn recognize_bytes(bytes: Vec<u8>, offset: Option<usize>, seconds: Option<usize>) -> Result<Vec<DecodedSignature>, JsValue> {
    match signatures_from_bytes(bytes, offset, seconds) {
        Ok(sig) => Ok(sig),
        Err(e) => return Err(JsValue::from_str(&e.to_string()))
    }
}

fn signatures_from_bytes(bytes: Vec<u8>, offset: Option<usize>, seconds: Option<usize>) -> Result<Vec<DecodedSignature>, Box<dyn Error>> {
    let offset_seconds = offset.unwrap_or(0);
    let (signal_spec, samples) = samples_from_bytes(bytes, seconds.unwrap_or(12) + offset_seconds)?;

    let target_rate = 16000;
    let resampled_samples = resample(signal_spec, samples, target_rate)?;

    let offset_samples = offset_seconds * target_rate as usize;

    // Calculate the number of slices needed, adjusting start index by the offset
    let num_slices = ((resampled_samples.len().saturating_sub(offset_samples) + (12 * 16000) - 1) / (12 * 16000)).max(1);
    let mut decoded_signatures = Vec::with_capacity(num_slices);

    if num_slices == 1 {
        let samples_slice = &resampled_samples[offset_samples..];
        decoded_signatures.push(DecodedSignature::new(samples_slice.into()));
    } else {
        let mut start_index = offset_samples;
        while start_index < resampled_samples.len() {
            let end_index = (start_index + (12 * 16000)).min(resampled_samples.len());
            let samples_slice = &resampled_samples[start_index..end_index];
            decoded_signatures.push(DecodedSignature::new(samples_slice.into()));
            start_index = end_index;
        }
    }
    
    Ok(decoded_signatures)
}


#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}