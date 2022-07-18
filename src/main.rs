mod components;
mod block;
mod actor;
mod laser;
use std::{fs::File, io::{BufReader, Read}};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use bevy::{prelude::*, utils::HashSet};
use block::*;
use actor::*;
use components::*;
use laser::*;

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
        .add_state(GameState::StartUp)
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            mode: bevy::window::WindowMode::Windowed,
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(InspectorPlugin::<Data>::new())
        .add_system_set(SystemSet::on_enter(GameState::StartUp)
            .with_system(asset_setup_system)
            .with_system(block_map_setup_system)
            .with_system(game_setup_system)
        )
        .add_system_set(SystemSet::on_enter(GameState::GameSetup)
            .with_system(actor_setup_system)
            .with_system(block_large_setup_system)
            .with_system(block_medium_setup_system)
            .with_system(game_run_system))
        .add_system_set(SystemSet::on_update(GameState::Running)
            .with_system(actor_keyboard_event_system)
            .with_system(actor_move_system)
            .with_system(actor_laser_spawn_system)
            .with_system(laser_move_system)
            .with_system(laser_hit_system)
            .with_system(explosion_to_spawn_system)
            .with_system(explosion_animate_system)
            .with_system(block_decimate_system))
        .run();
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum GameState {
    StartUp,
    GameSetup,
    Running,
}

fn game_run_system(mut state: ResMut<State<GameState>>) {
    state.set(GameState::Running).unwrap();
}

fn game_setup_system(mut commands: Commands, windows: Res<Windows>, mut state: ResMut<State<GameState>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary().unwrap();
    let (win_width, win_height) = (window.width(), window.height());
    let win_size = WinSize {
        w: win_width,
        h: win_height,
    };
    commands.insert_resource(win_size);

    state.set(GameState::GameSetup).unwrap();
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

pub fn explosion_to_spawn_system(
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

fn explosion_animate_system(
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



