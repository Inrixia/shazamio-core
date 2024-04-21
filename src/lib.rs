mod fingerprinting;
use std::error::Error;

use wasm_bindgen::prelude::*;
use fingerprinting::{samples_from_bytes::samples_from_bytes, decoded_signature::DecodedSignature};
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
    let (signal_spec, samples) = samples_from_bytes(bytes, offset_seconds + seconds.unwrap_or(12))?;

    let sample_rate = signal_spec.rate;
    let num_channels = signal_spec.channels.count() as usize;

    let sample_ratio = sample_rate  as usize * num_channels;

    let offset_samples = offset_seconds * sample_ratio;
    let _12s_samples = sample_ratio * 12;
    let samples_len = samples.len();

    // Calculate the number of slices needed, adjusting start index by the offset
    let num_slices = ((samples_len.saturating_sub(offset_samples) + _12s_samples - 1) / _12s_samples).max(1);
    let mut decoded_signatures = Vec::with_capacity(num_slices);
    if num_slices == 1 {
        let samples_slice = &samples[offset_samples..];
        decoded_signatures.push(DecodedSignature::new(samples_slice.into(), sample_rate, num_channels));
    } else {
        let mut start_index = offset_samples;
        while start_index < samples_len {
            let end_index = (start_index + _12s_samples).min(samples_len);
            let samples_slice = &samples[start_index..end_index];
            decoded_signatures.push(DecodedSignature::new(samples_slice.into(), sample_rate, num_channels));
            start_index = end_index;
        }
    }

    Ok(decoded_signatures)
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}