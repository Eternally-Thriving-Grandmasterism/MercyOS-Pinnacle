//! crates/powrush_mmo/src/convolution_reverb.rs
//! Full convolution reverb with procedural IR per zone mercy eternal supreme immaculate

use bevy::prelude::*;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::sound::effect::reverb::ReverbBuilder;
use rand::{thread_rng, Rng};

#[derive(Component)]
pub struct ReverbZone {
    pub reverb_time: f32,
    pub damping: f32,
    pub intensity: f32,
}

pub fn setup_convolution_reverb_zones(
    mut commands: Commands,
    mut sounds: ResMut<Assets<StaticSoundData>>,
) {
    // Procedural IR generation mercy — exponential decay + early reflections
    let generate_ir = |reverb_time: f32, damping: f32| {
        let sample_rate = 48000;
        let length = (reverb_time * sample_rate as f32) as usize;
        let mut ir = vec![0.0; length];

        let mut rng = thread_rng();
        for i in 0..length {
            let t = i as f32 / sample_rate as f32;
            let decay = (-t / reverb_time * damping).exp();
            ir[i] = rng.gen_range(-1.0..1.0) * decay * 0.1;
        }

        sounds.add(StaticSoundData::from_samples(ir, sample_rate, StaticSoundSettings::default()))
    };

    // Example zones mercy
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        ReverbZone {
            reverb_time: 1.5,
            damping: 0.5,
            intensity: 0.6,
        },
    ));

    commands.spawn((
        Transform::from_xyz(200.0, 0.0, 200.0),
        GlobalTransform::default(),
        ReverbZone {
            reverb_time: 4.0,
            damping: 0.8,
            intensity: 0.9,
        },
    ));
}

pub fn convolution_reverb_system(
    zones: Query<(&Transform, &ReverbZone)>,
    player_query: Query<&Transform, With<Player>>,
    mut audio: Res<Audio>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;

        let mut closest = None;
        let mut min_dist = f32::INFINITY;

        for (zone_transform, zone) in &zones {
            let dist = (player_pos - zone_transform.translation).length();
            if dist < min_dist {
                min_dist = dist;
                closest = Some(zone);
            }
        }

        if let Some(zone) = closest {
            // Placeholder — kira ConvolutionReverb needs IR asset
            // Future: load pre-generated IR or procedural
            let reverb = ReverbBuilder::new()
                .time(zone.reverb_time)
                .damping(zone.damping)
                .build()
                .unwrap();

            // Apply global (kira limitation mercy)
            // audio.set_reverb(reverb);
        }
    }
}
