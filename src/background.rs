use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(background_setup);
            // .add_system_set(
            //     SystemSet::new()
            //         .with_run_criteria(FixedTimestep::step(constants::TIME_STEP as f64))
            //         .with_system(background_movement_system),
        // );
    }
}

fn background_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    for i in -50..50 {
        for j in -50..50 {
            let background_image_handle = asset_server.load("textures/background/background.png");
            commands
                .spawn(SpriteBundle {
                    texture: background_image_handle,
                    transform: Transform {
                        translation: Vec3 {
                            x: i as f32 * 500.0,
                            y: j as f32 * 500.0,
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
            .insert(BackgroundBundle {
                _b: Background,
                size: BackgroundSize {
                    width: 500,
                    height: 500,
                },
            });
        }
    }
}

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct BackgroundSize {
    width: i32,
    height: i32,
}

#[derive(Bundle)]
pub struct BackgroundBundle {
    _b: Background,
    size: BackgroundSize,
}

