use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct Brain {
    pub lifetime: Timer,
}

#[derive(Resource)]
pub struct Money(pub f32);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Blood Harvester Farm".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (character_movement, spawn_brain, brain_lifetime))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::WindowSize(0.2);

    commands.spawn(camera);

    let texture: Handle<Image> = asset_server.load("character_1.png");

    // tuples will be component bundles
    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player { speed: 500.0 },
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::Up) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::Down) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::Left) {
            transform.translation.x -= movement_amount;
        }
        if input.pressed(KeyCode::Right) {
            transform.translation.x += movement_amount;
        }
    }
}

fn spawn_brain(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();

    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("spent $10 on a brain, remaning money: ${:?}", money.0);

        let texture = asset_server.load("brain.png");

        commands.spawn((
            SpriteBundle {
                texture,
                transform: *player_transform,
                ..default()
            },
            Brain {
                lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ));
    }
}

fn brain_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut brains: Query<(Entity, &mut Brain)>,
    mut money: ResMut<Money>,
) {
    for (brain_entity, mut brain) in &mut brains {
        brain.lifetime.tick(time.delta());

        if brain.lifetime.finished() {
            money.0 += 15.0;

            commands.entity(brain_entity).despawn();

            info!("Brain sold for $15! Current Money: ${:?}", money.0);
        }
    }
}
