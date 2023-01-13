//! Demonstrates rotating entities in 2D using quaternions.
use bevy::{prelude::*};
use my_game::enemy::*;
use my_game::player::*;
use my_game::projectile::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_system(projectile_movement_system)
        .add_system(bevy::window::close_on_esc.label("esc"))
        .run();
}
