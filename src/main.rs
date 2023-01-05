//! Demonstrates rotating entities in 2D using quaternions.
use bevy::{prelude::*, time::FixedTimestep};
use my_game::constants;
use my_game::enemy::*;
use my_game::player::*;
use my_game::projectile::*;
use my_game::setup::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .label("control")
                .with_run_criteria(FixedTimestep::step(constants::TIME_STEP as f64))
                .with_system(snap_to_player_system)
                .with_system(rotate_to_player_system),
        )
        .add_system(projectile_movement_system)
        .add_system(collision_system)
        .add_system(bevy::window::close_on_esc.label("esc"))
        .run();
}
