//! crates/powrush_mmo/src/ambisonics.rs
//! Higher-order Ambisonics (3rd order) encoding + binaural decoding mercy eternal supreme immaculate

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
        .with_order(3)  // 3rd order HOA = 16 channels mercy
        .build();

    commands.insert_resource(AmbisonicProcessor { ambisonic });
}

pub fn ambisonics_encode_system(
    mut processor: ResMut<AmbisonicProcessor>,
    sources: Query<(&SoundSource, &AudioInstance)>,  // Future per-source samples mercy
) {
    processor.ambisonic.clear();

    for (source, _instance) in &sources {
        let direction = source.position.normalize();
        // Encode mono source to HOA B-format mercy
        processor.ambisonic.encode(direction.into(), 1.0);
    }

    // Decode to binaural with head orientation mercy (future from PlayerHead)
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

        // Play binaural stereo mercy (future streaming)
        // audio.play(binaural_left + binaural_right interleaved);
    }
}
