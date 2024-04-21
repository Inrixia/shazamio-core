use rubato::Resampler;
use rubato::SincFixedIn;
use rubato::SincInterpolationType;
use rubato::SincInterpolationParameters;

pub fn resample(sample_rate: u32, channel_count: usize, samples: &Vec<f32>, target_rate: i32) -> Vec<i16> {
    let resample_ratio = target_rate as f64 / sample_rate as f64;
    let max_resample_ratio_relative = 2.0;
    let chunk_size = samples.len() / channel_count;
    let parameters = SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: SincInterpolationType::Cubic,
        oversampling_factor: 160,
        window: rubato::WindowFunction::BlackmanHarris2,
    };
    let mut resampler = SincFixedIn::new(
        resample_ratio,
        max_resample_ratio_relative,
        parameters,
        chunk_size,
        1,
    ).expect("Failed to create resampler");

    let mut mono_samples = vec![0f32; samples.len() / channel_count];
    for (i, sample) in samples.iter().enumerate() {
        mono_samples[i / channel_count] += sample / channel_count as f32;
    }

    let resampled_samples = resampler.process(&[&mono_samples], None).expect("Failed to resample");
    let result: Vec<i16> = resampled_samples[0]
        .iter()
        .map(|&sample| (sample * i16::MAX as f32) as i16)
        .collect();

    result
}