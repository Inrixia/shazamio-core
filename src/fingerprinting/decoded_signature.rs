use base64::engine::general_purpose;
use base64::Engine;
use byteorder::{LittleEndian, WriteBytesExt};
use crc32fast::Hasher;
use wasm_bindgen::prelude::wasm_bindgen;

use std::error::Error;
use std::io::{Cursor, Seek, SeekFrom, Write};

use crate::fingerprinting::resample::resample;
use crate::fingerprinting::signature_generator::SignatureGenerator;

const DATA_URI_PREFIX: &str = "data:audio/vnd.shazam.sig;base64,";

#[wasm_bindgen]
pub struct DecodedSignature {
    sample_rate_hz: u32,
    orig_sample_rate_hz: u32,
    orig_channel_count: usize,
    f32_buffer: Vec<f32>,
    _i16_buffer: Vec<i16>,
    uri: String,
}

#[wasm_bindgen]
impl DecodedSignature {
    pub fn new(f32_buffer: Vec<f32>, orig_sample_rate_hz: u32, orig_channel_count: usize) -> DecodedSignature {
        DecodedSignature {
            sample_rate_hz: 16000,
            orig_sample_rate_hz,
            orig_channel_count,
            _i16_buffer: Vec::new(),
            f32_buffer,
            uri: String::new(),
        }
    }

    fn encode_to_binary(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut cursor = Cursor::new(vec![]);

        // Please see the RawSignatureHeader structure definition above for
        // information about the following fields.

        cursor.write_u32::<LittleEndian>(0xcafe2580)?; // magic1
        cursor.write_u32::<LittleEndian>(0)?; // crc32 - Will write later
        cursor.write_u32::<LittleEndian>(0)?; // size_minus_header - Will write later
        cursor.write_u32::<LittleEndian>(0x94119c00)?; // magic2
        cursor.write_u32::<LittleEndian>(0)?; // void1
        cursor.write_u32::<LittleEndian>(0)?;
        cursor.write_u32::<LittleEndian>(0)?;
        cursor.write_u32::<LittleEndian>(
            match self.sample_rate_hz {
                8000 => 1,
                11025 => 2,
                16000 => 3,
                32000 => 4,
                44100 => 5,
                48000 => 6,
                _ => {
                    panic!("Invalid sample rate passed when encoding Shazam packet");
                }
            } << 27,
        )?; // shifted_sample_rate_id
        cursor.write_u32::<LittleEndian>(0)?; // void2
        cursor.write_u32::<LittleEndian>(0)?;
        cursor.write_u32::<LittleEndian>(
            self.number_samples() as u32 + (self.sample_rate_hz as f32 * 0.24) as u32,
        )?; // number_samples_plus_divided_sample_rate
        cursor.write_u32::<LittleEndian>((15 << 19) + 0x40000)?; // fixed_value

        cursor.write_u32::<LittleEndian>(0x40000000)?;
        cursor.write_u32::<LittleEndian>(0)?; // size_minus_header - Will write later

        let frequency_band_to_sound_peaks = SignatureGenerator::frequency_band_to_sound_peaks(&self.i16_buffer());
        let mut sorted_iterator: Vec<_> = frequency_band_to_sound_peaks.iter().collect();
        sorted_iterator.sort_by(|x, y| x.0.cmp(y.0));

        for (frequency_band, frequency_peaks) in sorted_iterator {
            let mut peaks_cursor = Cursor::new(vec![]);

            let mut fft_pass_number = 0;

            for frequency_peak in frequency_peaks {
                assert!(frequency_peak.fft_pass_number >= fft_pass_number);

                if frequency_peak.fft_pass_number - fft_pass_number >= 255 {
                    peaks_cursor.write_u8(0xff)?;
                    peaks_cursor.write_u32::<LittleEndian>(frequency_peak.fft_pass_number)?;

                    fft_pass_number = frequency_peak.fft_pass_number;
                }

                peaks_cursor.write_u8((frequency_peak.fft_pass_number - fft_pass_number) as u8)?;

                peaks_cursor.write_u16::<LittleEndian>(frequency_peak.peak_magnitude)?;
                peaks_cursor
                    .write_u16::<LittleEndian>(frequency_peak.corrected_peak_frequency_bin)?;

                fft_pass_number = frequency_peak.fft_pass_number;
            }

            let peaks_buffer = peaks_cursor.into_inner();

            cursor.write_u32::<LittleEndian>(0x60030040 + *frequency_band as u32)?;
            cursor.write_u32::<LittleEndian>(peaks_buffer.len() as u32)?;
            cursor.write_all(&peaks_buffer)?;
            for _padding_index in 0..((4 - peaks_buffer.len() as u32 % 4) % 4) {
                cursor.write_u8(0)?;
            }
        }

        let buffer_size = cursor.position() as u32;

        cursor.seek(SeekFrom::Start(8))?;
        cursor.write_u32::<LittleEndian>(buffer_size - 48)?;

        cursor.seek(SeekFrom::Start(48 + 4))?;
        cursor.write_u32::<LittleEndian>(buffer_size - 48)?;

        cursor.seek(SeekFrom::Start(4))?;
        let mut hasher = Hasher::new();
        hasher.update(&cursor.get_ref()[8..]);
        cursor.write_u32::<LittleEndian>(hasher.finalize())?; // crc32

        Ok(cursor.into_inner())
    }

    #[wasm_bindgen(getter)]
    pub fn uri(&mut self) -> String {
        if self.uri.len() != 0 {
            return self.uri.clone();
        }
        let binary = self.encode_to_binary();
        if binary.is_err() {
            return String::new();
        }
        self.uri = format!(
            "{}{}",
            DATA_URI_PREFIX,
            general_purpose::STANDARD.encode(binary.unwrap())
        ).clone();    
        return self.uri.clone();
    }

    #[wasm_bindgen(getter)]
    pub fn samplems(&mut self) -> u32 {
        (self.number_samples() as f64 / self.sample_rate_hz as f64 * 1000.0) as u32
    }

    #[wasm_bindgen(getter)]
    pub fn number_samples(&mut self) -> usize {
        self.i16_buffer().len()
    }

    fn i16_buffer(&mut self) -> &Vec<i16> {
        if self._i16_buffer.len() == 0 {
            self._i16_buffer = resample(self.orig_sample_rate_hz, self.orig_channel_count, &self.f32_buffer, 16000);
        }
        &self._i16_buffer
    }
}
