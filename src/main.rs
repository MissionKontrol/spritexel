mod components;
mod block;
mod actor;
use bevy::{prelude::*, sprite::collide_aabb::collide, utils::HashSet};
use block::*;
use actor::*;
use components::*;

const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 1000.0;
const SCREEN_HEIGHT_OFFSET: f32 = SCREEN_HEIGHT / 2.0;
const SCREEN_WIDTH_OFFSET: f32 = SCREEN_WIDTH / 2.0;

const LASER_SPRITE: &str = "laserGreenHorizontal.png";
const _LASER_SPRITE_SIZE: (f32, f32) = (70.0, 70.0);  // yes correct but not good for collisions
const LASER_SCALE: f32 = 1.0;
// const LASER_SPRITE_OFFSET: f32 = LASER_SPRITE_SIZE.0 / 2.0;

const EXPLOSION_LENGTH: usize = 6;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            mode: bevy::window::WindowMode::Windowed,
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(InspectorPlugin::<Data>::new())
        .add_startup_system(asset_setup_system)
        .add_startup_system(game_setup_system)
        .add_startup_system(actor_setup_system.after(asset_setup_system))
        .add_startup_system(block_large_setup_system.after(asset_setup_system))
        .add_startup_system(block_medium_setup_system.after(asset_setup_system))
        .add_system(actor_keyboard_event_system)
        .add_system(actor_move_system)
        .add_system(actor_laser_spawn_system)
        .add_system(laser_move_system)
        .add_system(laser_hit_system)
        .add_system(explosion_to_spawn_system)
        .add_system(animate_explosion_system)
        .add_system(block_decimate_system)
        .run();
}


fn game_setup_system(mut commands: Commands, windows: Res<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary().unwrap();
    let (win_width, win_height) = (window.width(), window.height());
    let win_size = WinSize {
        w: win_width,
        h: win_height,
    };
    commands.insert_resource(win_size);
}



fn asset_setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_server: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(ACTOR_ANIMATE_SPRITE);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(ACTOR_SPRITE_SIZE.0, ACTOR_SPRITE_SIZE.1),
        ACTOR_ANIMATE_SPRITE_COLS,
        ACTOR_ANIMATE_SPRITE_ROWS,
    );
    let texture_atlas_handle = atlas_server.add(texture_atlas);

    const EXPLOSION_ANIMATE_SPRITE: &str = "explosionGreen-sheet2x3.png";
    const EXPLOSION_SPRITE_SIZE: (f32, f32) = (70.0, 70.0);
    const EXPLOSION_ANIMATE_SPRITE_ROWS: usize = 2;
    const EXPLOSION_ANIMATE_SPRITE_COLS: usize = 3;

    let texture_handle = asset_server.load(EXPLOSION_ANIMATE_SPRITE);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(EXPLOSION_SPRITE_SIZE.0, EXPLOSION_SPRITE_SIZE.1),
        EXPLOSION_ANIMATE_SPRITE_COLS,
        EXPLOSION_ANIMATE_SPRITE_ROWS,
    );
    let explosion_atlas_handle = atlas_server.add(texture_atlas);

    let game_textures = GameTextures {
        actor: asset_server.load(ACTOR_SPRITE),
        block_large: asset_server.load(BLOCK_SPRITE),
        block_medium: asset_server.load(BLOCK_MEDIUM_SPRITE),
        // laser: asset_server.load(LASER_SPRITE),
        actor_animation_sprite: texture_atlas_handle,
        explosion_animation_sprite: explosion_atlas_handle,
    };
    commands.insert_resource(game_textures);

    // In Game resources
    commands.insert_resource(DespawnedList(HashSet::new()))
}

fn laser_move_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Velocity, &mut Transform), With<Laser>>,
    win_size: Res<WinSize>,
) {
    for (entity, velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * 5.0;

        if translation.x >= win_size.w / 2.0 {
            commands.entity(entity).despawn();
        }
    }
}

struct DespawnedList(pub HashSet<Entity>);

fn laser_hit_system(
    mut commands: Commands,
    mut despawned_list: ResMut<DespawnedList>,
    mut laser_query: Query<(Entity, &Transform, &SpriteSize), With<Laser>>,
    block_query: Query<(Entity, &Transform, &SpriteSize, &BlockSize), With<Block>>,
) {
    let despawned = &mut despawned_list.0;
    for (laser_entity, laser_transform, laser_sprite_size) in laser_query.iter_mut() {
        if despawned.contains(&laser_entity) {
            continue;
        }
            for (block_entity, block_transform, block_sprite_size, block_size) in block_query.iter() {
                if despawned.contains(&block_entity) || despawned.contains(&laser_entity) {
                    continue;
                }

                let collision = collide(
                    laser_transform.translation,
                    laser_sprite_size.0,
                    block_transform.translation,
                    block_sprite_size.0,
                );
    
                // perform collision
                if let Some(_) = collision {
                    // remove the block
                    despawned.insert(block_entity);
                    commands.entity(block_entity).despawn();
    
                    // remove the laser
                    despawned.insert(laser_entity);
                    commands.entity(laser_entity).despawn();
    
                    // spawn the explosionToSpawn
                    let mut explosion_location = block_transform.translation.clone();
                    // move up the Z
                    explosion_location[2] = 500.0;
    
                    commands
                        .spawn()
                        .insert(ExplosionToSpawn(explosion_location))
                        .insert(BlockToDecimate(block_transform.translation.clone()))
                        .insert(block_size.clone());
                }
            }
    }
}

fn explosion_to_spawn_system(
    mut commands: Commands,
    query: Query<(Entity, &ExplosionToSpawn)>,
    game_textures: Res<GameTextures>,
) {
    for (entity, explosion_to_spawn) in query.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: game_textures.explosion_animation_sprite.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(ExplosionTimer::default());

        commands.entity(entity).despawn();
    }
}

fn animate_explosion_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>,
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1; // move to next sprite cell
            if sprite.index >= EXPLOSION_LENGTH {
                commands.entity(entity).despawn()
            }
        }
    }
}


fn block_decimate_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&BlockToDecimate, &BlockSize)>,
) {
    for ( target_block, block_size) in query.iter() {
        if let BlockSize::Medimum(_) = block_size {
            continue
        }

        // commands.entity(entity).despawn();

        let mut x = target_block.0.x;
        let mut y = target_block.0.y;

        x += -30.0;
        y += -30.0;

        const FOONUM: f32 = 7.0;

        for row in 0..FOONUM as i32 {
            for col in 0..FOONUM as i32 {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: asset_server.load(BLOCK_MEDIUM_SPRITE),
                        transform: Transform {
                            translation: Vec3::new(
                                x + row as f32 * 10.0,
                                y + col as f32 * 10.0,
                                2.0,
                            ),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(SpriteSize::from(BLOCK_MEDIUM_SPRITE_SIZE))
                    .insert(Block)
                    .insert(BlockSize::Medimum(10));
            }
        }
    }
}
