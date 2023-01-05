use crate::constants;
use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub movement_speed: f32,
    pub collision_radius: f32,
}

pub fn projectile_movement_system(mut projectile_query: Query<(&Projectile, &mut Transform)>) {
    for (projectile, mut projectile_transform) in &mut projectile_query {
        let movement_direction = projectile_transform.rotation * Vec3::Y;

        let projectile_movement_distance = projectile.movement_speed * constants::TIME_STEP;
        let projectile_translation_delta = movement_direction * projectile_movement_distance;

        projectile_transform.translation += projectile_translation_delta;
    }
}
