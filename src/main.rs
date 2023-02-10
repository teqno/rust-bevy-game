//! Demonstrates rotating entities in 2D using quaternions.
use bevy::prelude::*;
use my_game::background::BackgroundPlugin;
use my_game::enemy::*;
use my_game::player::*;
use my_game::projectile::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BackgroundPlugin)
        .add_plugin(PlayerPlugin)                   // Setup player attributes, Spawn player, Control system
        .add_plugin(EnemyPlugin)                    // Setup enemy attributes, Spawn enemies, Collision system
        .add_system(projectile_movement_system)     // Projectile spawn and movement systems
        .add_system(bevy::window::close_on_esc.label("esc"))
        .run();
}
