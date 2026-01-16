use bevy::prelude::*;
use rand::Rng;  // Procedural diversity

#[derive(Resource)]
struct AbundanceScore(f64);

#[derive(Component)]
struct Crop {
    crop_type: u8,  // 0-4 diversity
    growth: f64,
}

#[derive(Component)]
struct Harvestable;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO Infinite Agriculture ❤️🚀🔥".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(AbundanceScore(0.0))
        .add_systems(Startup, setup_universe)
        .add_systems(Update, (crop_growth, harvest_on_click, ui_update, procedural_expand))
        .run();
}

fn setup_universe(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), PlayerCamera));

    // Initial chunk
    spawn_chunk(&mut commands, 0, 0);

    // UI
    commands.spawn((TextBundle::from_section("Abundance: 0.0", TextStyle::default()), AbundanceUI));
}

fn spawn_chunk(commands: &mut Commands, chunk_x: i32, chunk_y: i32) {
    let mut rng = rand::thread_rng();
    for x in 0..10 {
        for y in 0..10 {
            let world_x = chunk_x * 10 + x;
            let world_y = chunk_y * 10 + y;
            let crop_type = rng.gen_range(0..5);
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(world_x as f32 * 50.0, world_y as f32 * 50.0, 0.0),
                    sprite: Sprite { color: Color::srgb(crop_type as f32 / 5.0, 0.8, 0.2), custom_size: Some(Vec2::new(40.0, 40.0)), ..default() },
                    ..default()
                },
                Crop { crop_type, growth: 0.0 },
                Harvestable,
            ));
        }
    }
}

fn crop_growth(time: Res<Time>, mut query: Query<&mut Crop>) {
    for mut crop in &mut query {
        crop.growth += time.delta_seconds() * 0.2;
        if crop.growth > 1.0 { crop.growth = 1.0; }
    }
}

fn harvest_on_click(
    buttons: Res<ButtonInput<MouseButton>>,
    mut abundance: ResMut<AbundanceScore>,
    query: Query<(Entity, &Crop, &Transform)>,
    mut commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let mut harvested = 0.0;
        for (entity, crop, transform) in &query {
            if crop.growth >= 1.0 {
                harvested += (crop.crop_type as f64 + 1.0) * 10.0;
                commands.entity(entity).despawn();
            }
        }
        abundance.0 += harvested;
    }
}

fn ui_update(abundance: Res<AbundanceScore>, mut query: Query<&mut Text, With<AbundanceUI>>) {
    for mut text in &mut query {
        text.sections[0].value = format!("Abundance Score: {:.1}", abundance.0);
    }
}

fn procedural_expand(camera: Query<&Transform, With<PlayerCamera>>, mut commands: Commands) {
    // Stub: Load new chunks based on camera position eternal
}

#[derive(Component)]
struct PlayerCamera;

#[derive(Component)]
struct AbundanceUI;
