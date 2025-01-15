use wasm_bindgen::prelude::*;
use js_sys::Float32Array;

// Expose the function to Javascript
#[wasm_bindgen]
pub fn generate_wave(frequency: f32, duration: f32) -> Float32Array {
    let sample_rate = 44100.0;
    let total_samples = (duration * sample_rate) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let sample = (2.0 * std::f32::consts::PI * frequency * i as f32 / sample_rate).sin();
        samples.push(sample);
    }

    Float32Array::from(samples.as_slice())
}
