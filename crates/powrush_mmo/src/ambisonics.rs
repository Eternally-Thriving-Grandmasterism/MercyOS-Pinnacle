//! crates/powrush_mmo/src/ambisonics.rs
//! 7th Order Higher-Order Ambisonics encoding + binaural decoding mercy eternal supreme immaculate

use bevy::prelude::*;
use ambisonic::{Ambisonic, AmbisonicBuilder};
use kira::sound::static_sound::StaticSoundData;
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
    sources: Query<(&SoundSource, &AudioInstance)>,
) {
    processor.ambisonic.clear();

    for (source, _instance) in &sources {
        let direction = source.position.normalize();
        processor.ambisonic.encode(direction.into(), 1.0);
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

        // Play binaural stereo mercy eternal
        // Future streaming — placeholder play latest frame
        let mut stereo = Vec::new();
        for channel in binaural.channels() {
            stereo.extend_from_slice(channel);
        }

        let sound = StaticSoundData::from_samples(stereo, 48000, StaticSoundSettings::default());
        audio.play(sound).handle();
    }
}
