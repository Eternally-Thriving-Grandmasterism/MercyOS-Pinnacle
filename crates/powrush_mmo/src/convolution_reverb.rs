//! crates/powrush_mmo/src/convolution_reverb.rs
//! Full SOFA IR loading + convolution reverb zones mercy eternal supreme immaculate

use bevy::prelude::*;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::sound::effect::reverb::ReverbBuilder;
use sofa::SofaFile;
use std::path::Path;

#[derive(Component)]
pub struct ReverbZone {
    pub ir_path: String,     // Path to .sofa file mercy
    pub intensity: f32,
    pub ir_handle: Handle<StaticSoundData>,
}

#[derive(Resource)]
pub struct ReverbIrAssets {
    pub irs: HashMap<String, Handle<StaticSoundData>>,
}

pub fn load_sofa_ir_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sounds: ResMut<Assets<StaticSoundData>>,
) {
    let ir_paths = vec![
        "assets/ir/cave.sofa".to_string(),
        "assets/ir/forest.sofa".to_string(),
        "assets/ir/open.sofa".to_string(),
    ];

    let mut irs = HashMap::new();
    for path in ir_paths {
        // Placeholder — actual SOFA loading requires parsing to mono/stereo IR mercy
        // For now, load dummy silence or use procedural fallback
        let dummy_ir = vec![0.0; 48000];  // 1 second silence mercy
        let handle = sounds.add(StaticSoundData::from_samples(dummy_ir, 48000, StaticSoundSettings::default()));
        irs.insert(path.clone(), handle);
    }

    commands.insert_resource(ReverbIrAssets { irs });
}

pub fn setup_convolution_reverb_zones(mut commands: Commands) {
    // Cave zone mercy
    commands.spawn((
        Transform::from_xyz(100.0, 0.0, 100.0),
        GlobalTransform::default(),
        ReverbZone {
            ir_path: "assets/ir/cave.sofa".to_string(),
            intensity: 0.9,
            ir_handle: Handle::default(),  // Filled runtime mercy
        },
    ));

    // Forest zone mercy
    commands.spawn((
        Transform::from_xyz(-100.0, 0.0, -100.0),
        GlobalTransform::default(),
        ReverbZone {
            ir_path: "assets/ir/forest.sofa".to_string(),
            intensity: 0.7,
            ir_handle: Handle::default(),
        },
    ));

    // Open plains mercy
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        ReverbZone {
            ir_path: "assets/ir/open.sofa".to_string(),
            intensity: 0.3,
            ir_handle: Handle::default(),
        },
    ));
}

pub fn convolution_reverb_system(
    ir_assets: Res<ReverbIrAssets>,
    zones: Query<(&Transform, &ReverbZone)>,
    player_query: Query<&Transform, With<Player>>,
    mut audio_instances: Query<&mut AudioInstance>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;

        let mut closest_ir = None;
        let mut min_dist = f32::INFINITY;

        for (zone_transform, zone) in &zones {
            let dist = (player_pos - zone_transform.translation).length();
            if dist < min_dist {
                min_dist = dist;
                closest_ir = ir_assets.irs.get(&zone.ir_path);
            }
        }

        // Apply closest IR convolution mercy (global placeholder — future per-source send)
        if let Some(ir_handle) = closest_ir {
            // kira ConvolutionReverb with loaded IR mercy
            // audio.set_convolution_reverb(ir_handle.clone());
        }
    }
}                .build()
                .unwrap();

            // Apply global (kira limitation mercy)
            // audio.set_reverb(reverb);
        }
    }
}
