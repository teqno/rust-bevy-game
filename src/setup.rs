use crate::constants;
use crate::enemy::{Enemy, RotateToPlayer, SnapToPlayer};
use bevy::prelude::*;
/// Add the game's entities to our world and creates an orthographic camera for 2D rendering.
///
/// The Bevy coordinate system is the same for 2D and 3D, in terms of 2D this means that:
///
/// * `X` axis goes from left to right (`+X` points right)
/// * `Y` axis goes from bottom to top (`+Y` point up)
/// * `Z` axis goes from far to near (`+Z` points towards you, out of the screen)
///
/// The origin is at the center of the screen.
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let ship_handle = asset_server.load("textures/simplespace/ship_C.png");
    let enemy_a_handle = asset_server.load("textures/simplespace/enemy_A.png");
    let enemy_b_handle = asset_server.load("textures/simplespace/enemy_B.png");

    // 2D orthographic camera
    commands.spawn_bundle(Camera2dBundle::default());

    let horizontal_margin = constants::BOUNDS.x / 4.0;
    let vertical_margin = constants::BOUNDS.y / 4.0;

    // enemy that snaps to face the player spawns on the bottom and left
    commands
        .spawn_bundle(SpriteBundle {
            texture: enemy_a_handle.clone(),
            transform: Transform::from_xyz(0.0 - horizontal_margin, 0.0, 0.0),
            ..default()
        })
        .insert(Enemy {
            collision_radius: 16.0,
        })
        .insert(SnapToPlayer);
    commands
        .spawn_bundle(SpriteBundle {
            texture: enemy_a_handle,
            transform: Transform::from_xyz(0.0, 0.0 - vertical_margin, 0.0),
            ..default()
        })
        .insert(Enemy {
            collision_radius: 16.0,
        })
        .insert(SnapToPlayer);

    // enemy that rotates to face the player enemy spawns on the top and right
    commands
        .spawn_bundle(SpriteBundle {
            texture: enemy_b_handle.clone(),
            transform: Transform::from_xyz(0.0 + horizontal_margin, 0.0, 0.0),
            ..default()
        })
        .insert(Enemy {
            collision_radius: 16.0,
        })
        .insert(RotateToPlayer {
            rotation_speed: f32::to_radians(45.0), // degrees per second
        });
    commands
        .spawn_bundle(SpriteBundle {
            texture: enemy_b_handle,
            transform: Transform::from_xyz(0.0, 0.0 + vertical_margin, 0.0),
            ..default()
        })
        .insert(Enemy {
            collision_radius: 16.0,
        })
        .insert(RotateToPlayer {
            rotation_speed: f32::to_radians(90.0), // degrees per second
        });
}
