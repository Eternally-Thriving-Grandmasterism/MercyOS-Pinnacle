use bevy::prelude::*;
use bevy::window::WindowResolution;

// Eternal resources for mercy-gated abundance
#[derive(Resource)]
struct AbundanceScore(f64);

#[derive(Resource)]
struct MercyGateThreshold(f64);

// Components for eternal thriving entities
#[derive(Component)]
struct Crop {
    growth_stage: f64,
    yield_potential: f64,
}

#[derive(Component)]
struct InfiniteField;

#[derive(Component)]
struct PlayerCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO: Infinite Agriculture Universe Genesis ❤️🚀🔥".into(),
                resolution: WindowResolution::new(1920.0, 1080.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(AbundanceScore(0.0))
        .insert_resource(MercyGateThreshold(0.95))  // Positive recurrence sealed at 95% posterior harmony
        .add_systems(Startup, (setup_eternal_universe, setup_ui_beacon))
        .add_systems(Update, (
            eternal_crop_growth,
            procedural_field_expansion,
            mercy_yield_harvest,
            player_camera_controls,
        ))
        .run();
}

fn setup_eternal_universe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 2D camera with player control tag
    commands.spawn((
        Camera2dBundle::default(),
        PlayerCamera,
    ));
    
    // Procedural genesis: Spawn initial infinite field grid (expand eternal)
    for x in -10..10 {
        for y in -10..10 {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x as f32 * 100.0, y as f32 * 100.0, 0.0),
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::new(100.0, 100.0)),
                        ..default()
                    },
                    ..default()
                },
                InfiniteField,
                Crop {
                    growth_stage: 0.0,
                    yield_potential: 1.0 + (x.abs() + y.abs()) as f64 * 0.01,  // Procedural potential variance
                },
            ));
        }
    }
    
    // Example sacred crop beacon (expand to infinite types eternal)
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Eternal Thriving Crops Ascending",
            TextStyle {
                font_size: 30.0,
                color: Color::GOLD,
                ..default()
            },
        ),
        transform: Transform::from_xyz(0.0, 300.0, 1.0),
        ..default()
    });
}

fn setup_ui_beacon(mut commands: Commands) {
    // Sacred genesis UI overlay
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Powrush-MMO ❤️🚀🔥\nAbundance Score: 0.0\nMercy-Gated Positive Recurrence Sealed Supreme",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}

fn eternal_crop_growth(
    time: Res<Time>,
    mut query: Query<&mut Crop>,
    mut transform_query: Query<&mut Transform, With<Crop>>,
) {
    // Mercy-gated growth system – abundance flows eternal
    for mut crop in &mut query {
        crop.growth_stage += time.delta_seconds() as f64 * 0.05;  // Base growth velocity
        if crop.growth_stage > 1.0 {
            crop.growth_stage = 1.0;  // Mature pinnacle
        }
    }
    
    // Visual scale harmony
    for mut transform in &mut transform_query {
        let scale_factor = 1.0 + (time.delta_seconds() * 0.1);
        transform.scale *= scale_factor;
    }
}

fn procedural_field_expansion(
    mut commands: Commands,
    query: Query<&Transform, With<InfiniteField>>,
    camera_query: Query<&Transform, With<PlayerCamera>>,
) {
    // Stub for infinite procedural expansion (trigger on camera edge eternal)
    // Future: Noise-based terrain, mercy-shielded chunks
}

fn mercy_yield_harvest(
    mut abundance: ResMut<AbundanceScore>,
    threshold: Res<MercyGateThreshold>,
    query: Query<&Crop>,
) {
    // Harvest logic: Aggregate yields, mercy-gate if posterior > threshold
    let total_yield: f64 = query.iter().map(|c| c.yield_potential * c.growth_stage).sum();
    if total_yield / query.iter().count() as f64 > threshold.0 {
        abundance.0 += total_yield;
        // UI update stub – positive recurrence sealed ❤️
    }
}

fn player_camera_controls(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<PlayerCamera>>,
    time: Res<Time>,
) {
    // Basic WASD pan for exploring infinite fields
    let speed = 500.0 * time.delta_seconds();
    for mut transform in &mut query {
        if keys.pressed(KeyCode::KeyW) {
            transform.translation.y += speed;
        }
        if keys.pressed(KeyCode::KeyS) {
            transform.translation.y -= speed;
        }
        if keys.pressed(KeyCode::KeyA) {
            transform.translation.x -= speed;
        }
        if keys.pressed(KeyCode::KeyD) {
            transform.translation.x += speed;
        }
    }
}
