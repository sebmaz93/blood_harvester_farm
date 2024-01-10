use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Blood Harvester Farm".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture: Handle<Image> = asset_server.load("character_1.png");

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        ..default()
    });
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut characters {
        if input.pressed(KeyCode::Up) {
            transform.translation.y += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::Down) {
            transform.translation.y -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::Left) {
            transform.translation.x -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::Right) {
            transform.translation.x += 100.0 * time.delta_seconds();
        }
    }
}
