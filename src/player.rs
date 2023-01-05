use crate::constants;
use crate::projectile::Projectile;
use bevy::{prelude::*, time::FixedTimestep};
use std::{
    io::{self, Write},
    time::Instant,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_setup).add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(constants::TIME_STEP as f64))
                .with_system(player_movement_system),
        );
    }
}

fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_handle = asset_server.load("textures/simplespace/ship_C.png");
    commands
        .spawn_bundle(SpriteBundle {
            texture: ship_handle,
            ..default()
        })
        .insert_bundle(PlayerBundle {
            movement_speed: PlayerMovementSpeed(500.0),
            rotation_speed: PlayerRotationSpeed(f32::to_radians(360.0)),
            _p: Player,
        });
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct PlayerMovementSpeed(f32);

#[derive(Component)]
struct PlayerRotationSpeed(f32);

#[derive(Bundle)]
struct PlayerBundle {
    movement_speed: PlayerMovementSpeed,
    rotation_speed: PlayerRotationSpeed,
    _p: Player,
}

fn player_movement_system(
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut player_query: Query<
        (&PlayerMovementSpeed, &PlayerRotationSpeed, &mut Transform),
        (With<Player>, Without<Camera>),
    >,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let (ship_movement_speed, ship_rotation_speed, mut transform) = player_query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        // fire projectile
        let ship_handle = asset_server.load("textures/simplespace/ship_C.png");
        println!("Fire");
        commands
            .spawn_bundle(SpriteBundle {
                texture: ship_handle,
                transform: *transform,
                ..default()
            })
            .insert(Projectile {
                movement_speed: 1000.0,
                collision_radius: 32.0,
            });
    }

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    transform.rotate_z(rotation_factor * ship_rotation_speed.0 * constants::TIME_STEP);

    // get the ship's forward vector by applying the current rotation to the ships initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let movement_distance = movement_factor * ship_movement_speed.0 * constants::TIME_STEP;
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;

    let start = Instant::now();

    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation += translation_delta;
    transform.translation += translation_delta;

    let duration = start.elapsed();
    print!("\x1B[2J");
    println!("Time elapsed: {:?}", duration.as_nanos());
    io::stdout().flush().unwrap();
    // bound the ship within the invisible level bounds
    // let extents = Vec3::from((constants::BOUNDS / 2.0, 0.0));
    // transform.translation = transform.translation.min(extents).max(-extents);
}
