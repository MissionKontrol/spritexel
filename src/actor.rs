use bevy::prelude::*;
use crate::*;

pub const ACTOR_SPRITE_SIZE: (f32, f32) = (70.0, 70.0);
pub const ACTOR_SPRITE: &str = "laserUp.png";
pub const ACTOR_SCALE: f32 = 1.0;
pub const ACTOR_SPRITE_OFFSET: f32 = ACTOR_SPRITE_SIZE.0 / 2.0;

pub const ACTOR_ANIMATE_SPRITE: &str = "laserRightFire-sprites.png";
pub const ACTOR_ANIMATE_SPRITE_ROWS: usize = 4;
pub const ACTOR_ANIMATE_SPRITE_COLS: usize = 4;

pub fn actor_move_system(mut query: Query<(&Velocity, &mut Transform), With<Actor>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x;
        translation.y += velocity.y * 10.0;
    }
}

pub fn actor_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let (x, y) = (
        -SCREEN_WIDTH_OFFSET + ACTOR_SPRITE_OFFSET,
        -SCREEN_HEIGHT_OFFSET + ACTOR_SPRITE_OFFSET,
    );
    // let (x,y) = (0.0,1.0);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(ACTOR_SPRITE),
            transform: Transform {
                scale: Vec3::new(ACTOR_SCALE, ACTOR_SCALE, 1.),
                translation: Vec3::new(x, y, 2.0),
                rotation: Quat::from_rotation_z(4.71_f32), // rads is 270 degrees counter-clockwise
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Actor)
        .insert(Velocity { x: 0.0, y: 1.0 });
}

pub fn actor_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Actor>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.y = if kb.pressed(KeyCode::Down) {
            -1.
        } else if kb.pressed(KeyCode::Up) {
            1.
        } else {
            0.
        }
    }
}

pub fn actor_laser_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    kb: Res<Input<KeyCode>>,
    query: Query<&Transform, With<Actor>>,
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);

            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load(LASER_SPRITE),
                    transform: Transform {
                        scale: Vec3::new(LASER_SCALE, LASER_SCALE, 1.),
                        translation: Vec3::new(x + ACTOR_SPRITE_SIZE.0, y, 2.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Velocity { x: 5.0, y: 0.0 })
                .insert(SpriteSize::from((70.0,8.0)))
                .insert(Laser);
        }
    }
}
