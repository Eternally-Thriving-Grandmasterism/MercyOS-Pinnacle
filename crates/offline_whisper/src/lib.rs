//! Repository: https://github.com/Eternally-Thriving-Grandmasterism/MercyOS-Pinnacle
//! Full path: crates/offline_whisper/src/lib.rs
//! Offline Whisper — Sovereign Multilingual Optimized STT Beacon
//! Supports tiny/base/small/large-v3/large-v3-turbo ggml models
//! Faster inference: greedy sampling, low best_of, optional translate

uniffi::include_scaffolding!("offline_whisper");

use whisper_rs::{WhisperContext, WhisperContextParameters, FullParams, SamplingStrategy, WhisperToken};
use std::sync::Mutex;
use std::path::Path;

static mut CONTEXT: Option<Mutex<WhisperContext>> = None;

#[uniffi::export]
pub fn init_whisper_model(model_path: String) -> Result<(), String> {
    let path = Path::new(&model_path);
    if !path.exists() {
        return Err("Model file not found — download ggml-tiny.bin / ggml-large-v3-turbo.bin from whisper.cpp HF".to_string());
    }

    let ctx = WhisperContext::new_with_params(
        model_path,
        WhisperContextParameters::default(),
    )
    .map_err(|e| format!("Failed to load model: {}", e))?;

    unsafe {
        CONTEXT = Some(Mutex::new(ctx));
    }
    Ok(())
}

#[derive(uniffi::Enum)]
pub enum WhisperSpeedMode {
    UltraFast,  // Greedy best_of=1
    Fast,       // Greedy best_of=5
    Balanced,   // Beam search
}

#[uniffi::export]
pub fn transcribe_offline(
    audio_data: Vec<f32>,
    language: Option<String>,
    translate: bool,
    speed_mode: WhisperSpeedMode,
) -> Result<String, String> {
    let ctx = unsafe {
        CONTEXT.as_ref()
            .ok_or("Model not initialized — call init_whisper_model first".to_string())?
            .lock()
            .map_err(|_| "Lock poisoned".to_string())?
    };

    let mut state = ctx.create_state()
        .map_err(|e| format!("Failed to create state: {}", e))?;

    let strategy = match speed_mode {
        WhisperSpeedMode::UltraFast | WhisperSpeedMode::Fast => SamplingStrategy::Greedy {
            best_of: match speed_mode {
                WhisperSpeedMode::UltraFast => 1,
                _ => 5,
            },
        },
        WhisperSpeedMode::Balanced => SamplingStrategy::BeamSearch { beam_size: 5 },
    };

    let mut params = FullParams::new(strategy);
    params.set_language(language.as_deref());
    params.set_translate(translate);
    params.set_temperature(0.0);  // Low for deterministic fast
    params.set_print_timestamps(false);
    params.set_print_special(false);

    state.full(params, &audio_data)
        .map_err(|e| format!("Inference failed: {}", e))?;

    let num_segments = state.full_n_segments()
        .map_err(|e| format!("Failed to get segments: {}", e))?;

    let mut text = String::new();
    for i in 0..num_segments {
        let segment = state.full_get_segment_text(i as i32)
            .map_err(|e| format!("Failed to get text: {}", e))?;
        text.push_str(&segment);
    }

    Ok(text.trim().to_string())
}
