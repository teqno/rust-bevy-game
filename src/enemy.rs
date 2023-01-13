use crate::constants;
use crate::player::*;
use crate::projectile::*;
use bevy::time::FixedTimestep;
use bevy::{math::Vec3Swizzles, prelude::*};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(constants::TIME_STEP as f64))
                .with_system(rotate_to_player_system)
                .with_system(snap_to_player_system)
                .with_system(move_forward_system)
                .with_system(collision_system)
                .with_system(spawn_enemies_system)
        );
    }
}

// snap to player ship behavior
#[derive(Component)]
pub struct SnapToPlayer;

#[derive(Component)]
pub struct EnemyMovementSpeed(f32);

#[derive(Component)]
pub struct EnemyRotationSpeed(f32);

#[derive(Component)]
pub struct Enemy {
    pub collision_radius: f32,
}

#[derive(Bundle)]
struct EnemyABundle {
    movement_speed: EnemyMovementSpeed,
    rotation_speed: EnemyRotationSpeed,
    _e: Enemy,
}

#[derive(Bundle)]
struct EnemyBBundle {
    movement_speed: EnemyMovementSpeed,
    snap_to_player: SnapToPlayer,
    _e: Enemy,
}

pub fn collision_system(
    mut projectile_query: Query<(Entity, &Projectile, &Transform)>,
    mut enemy_query: Query<(Entity, &Enemy, &Transform)>,
    mut commands: Commands,
) {
    for (projectile_entity, projectile, projectile_transform) in &mut projectile_query {
        for (enemy_entity, enemy, enemy_transform) in &mut enemy_query {
            let projectile_xy = projectile_transform.translation.xy();
            let enemy_xy = enemy_transform.translation.xy();

            if projectile_xy.distance(enemy_xy)
                < projectile.collision_radius + enemy.collision_radius
            {
                commands.entity(enemy_entity).despawn_recursive();
                commands.entity(projectile_entity).despawn_recursive();
            }
        }
    }
}

/// Demonstrates snapping the enemy ship to face the player ship immediately.
pub fn snap_to_player_system(
    mut query: Query<&mut Transform, (With<SnapToPlayer>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();
    // get the player translation in 2D
    let player_translation = player_transform.translation.xy();

    for mut enemy_transform in &mut query {
        // get the vector from the enemy ship to the player ship in 2D and normalize it.
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();

        // get the quaternion to rotate from the initial enemy facing direction to the direction
        // facing the player
        let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));

        // rotate the enemy to face the player
        enemy_transform.rotation = rotate_to_player;
    }
}

/// Demonstrates rotating an enemy ship to face the player ship at a given rotation speed.
///
/// This method uses the vector dot product to determine if the enemy is facing the player and
/// if not, which way to rotate to face the player. The dot product on two unit length vectors
/// will return a value between -1.0 and +1.0 which tells us the following about the two vectors:
///
/// * If the result is 1.0 the vectors are pointing in the same direction, the angle between them
///   is 0 degrees.
/// * If the result is 0.0 the vectors are perpendicular, the angle between them is 90 degrees.
/// * If the result is -1.0 the vectors are parallel but pointing in opposite directions, the angle
///   between them is 180 degrees.
/// * If the result is positive the vectors are pointing in roughly the same direction, the angle
///   between them is greater than 0 and less than 90 degrees.
/// * If the result is negative the vectors are pointing in roughly opposite directions, the angle
///   between them is greater than 90 and less than 180 degrees.
///
/// It is possible to get the angle by taking the arc cosine (`acos`) of the dot product. It is
/// often unnecessary to do this though. Beware than `acos` will return `NaN` if the input is less
/// than -1.0 or greater than 1.0. This can happen even when working with unit vectors due to
/// floating point precision loss, so it pays to clamp your dot product value before calling
/// `acos`.
pub fn rotate_to_player_system(
    player_query: Query<&Transform, With<Player>>,
    mut new_query: Query<(&EnemyRotationSpeed, &mut Transform), Without<Player>>
) {
    let player_transform = player_query.single();
    // get the player translation in 2D
    let player_translation = player_transform.translation.xy();

    for (rotation_speed, mut enemy_transform) in &mut new_query {
        // get the enemy ship forward vector in 2D (already unit length)
        let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();

        // get the vector from the enemy ship to the player ship in 2D and normalize it.
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();

        // get the dot product between the enemy forward vector and the direction to the player.
        let forward_dot_player = enemy_forward.dot(to_player);

        // if the dot product is approximately 1.0 then the enemy is already facing the player and
        // we can early out.
        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            continue;
        }

        // get the right vector of the enemy ship in 2D (already unit length)
        let enemy_right = (enemy_transform.rotation * Vec3::X).xy();

        // get the dot product of the enemy right vector and the direction to the player ship.
        // if the dot product is negative them we need to rotate counter clockwise, if it is
        // positive we need to rotate clockwise. Note that `copysign` will still return 1.0 if the
        // dot product is 0.0 (because the player is directly behind the enemy, so perpendicular
        // with the right vector).
        let right_dot_player = enemy_right.dot(to_player);

        // determine the sign of rotation from the right dot player. We need to negate the sign
        // here as the 2D bevy co-ordinate system rotates around +Z, which is pointing out of the
        // screen. Due to the right hand rule, positive rotation around +Z is counter clockwise and
        // negative is clockwise.
        let rotation_sign = -f32::copysign(1.0, right_dot_player);

        // limit rotation so we don't overshoot the target. We need to convert our dot product to
        // an angle here so we can get an angle of rotation to clamp against.
        let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos(); // clamp acos for safety

        // calculate angle of rotation with limit
        let rotation_angle =
            rotation_sign * (rotation_speed.0 * constants::TIME_STEP).min(max_angle);

        // rotate the enemy to face the player
        enemy_transform.rotate_z(rotation_angle);
    }
}


pub fn move_forward_system(
    mut new_query: Query<(&EnemyMovementSpeed, &mut Transform)>
) {
    for (movement_speed, mut enemy_transform) in &mut new_query {
        let movement_direction = enemy_transform.rotation * Vec3::Y;
        // get the distance the ship will move based on direction, the ship's movement speed and delta time
        let movement_distance = movement_speed.0 * constants::TIME_STEP;
        // create the change in translation using the new movement direction and distance
        let translation_delta = movement_direction * movement_distance;
        // rotate the enemy to face the player
        enemy_transform.translation += translation_delta;
    }
}

pub fn spawn_enemies_system(
   mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>
   ) {
    let time_in_seconds = time.time_since_startup().as_secs();
    println!("{:?}", time_in_seconds);
    if time.time_since_startup().as_secs() % 2 != 0 {
        return;
    }

    let enemy_a_handle = asset_server.load("textures/simplespace/enemy_A.png");
    let enemy_b_handle = asset_server.load("textures/simplespace/enemy_B.png");

    commands.spawn_bundle(SpriteBundle {
        texture: enemy_a_handle,
        ..default()
    })
    .insert_bundle(EnemyABundle {
        movement_speed: EnemyMovementSpeed(200.0),
        rotation_speed: EnemyRotationSpeed(f32::to_radians(180.0)),
        _e: Enemy { collision_radius: 32.0 }
    });

    commands.spawn_bundle(SpriteBundle {
        texture: enemy_b_handle,
        ..default()
    })
    .insert_bundle(EnemyBBundle {
        movement_speed: EnemyMovementSpeed(200.0),
        snap_to_player: SnapToPlayer,
        _e: Enemy { collision_radius: 32.0 }
    });
}
