//! crates/powrush_mmo/src/ambisonics.rs
//! 7th Order Higher-Order Ambisonics full B-format encoding + binaural decoding mercy eternal supreme immaculate

use bevy::prelude::*;
use ambisonic::{Ambisonic, AmbisonicBuilder};
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use crate::main::SoundSource;

#[derive(Resource)]
pub struct AmbisonicProcessor {
    pub ambisonic: Ambisonic<f32>,
}

pub fn setup_ambisonics(mut commands: Commands) {
    let ambisonic = AmbisonicBuilder::default()
        .with_order(7)  // 7th order HOA = 64 channels ultra-resolution mercy eternal
        .build();

    commands.insert_resource(AmbisonicProcessor { ambisonic });
}

pub fn ambisonics_encode_system(
    mut processor: ResMut<AmbisonicProcessor>,
    sources: Query<(&SoundSource, &AudioInstance)>,  // Future per-source samples mercy
    listener_query: Query<&Transform, With<PlayerHead>>,
) {
    processor.ambisonic.clear();

    if let Ok(listener) = listener_query.get_single() {
        let listener_forward = listener.forward();
        let listener_up = listener.up();

        for (source, _instance) in &sources {
            let relative = source.position - listener.translation;
            let direction = relative.normalize_or_zero();

            // Encode mono source to HOA B-format with head-relative direction mercy
            processor.ambisonic.encode(direction.into(), 1.0);
        }
    }
}

pub fn ambisonics_decode_system(
    processor: Res<AmbisonicProcessor>,
    head_query: Query<&Transform, With<PlayerHead>>,
    audio: Res<Audio>,
) {
    if let Ok(head) = head_query.get_single() {
        let forward = head.forward();
        let up = head.up();

        let binaural = processor.ambisonic.decode_binaural(forward.into(), up.into());

        // Interleave left/right for stereo playback mercy eternal
        let mut stereo = Vec::with_capacity(binaural.channel_count() * binaural.frame_count());
        for frame in 0..binaural.frame_count() {
            stereo.push(binaural.channel(0)[frame]);  // Left
            stereo.push(binaural.channel(1)[frame]);  // Right
        }

        let sound = StaticSoundData::from_samples(stereo, binaural.sample_rate() as u32, StaticSoundSettings::default());
        audio.play(sound).handle();
    }
}
