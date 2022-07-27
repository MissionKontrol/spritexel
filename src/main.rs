mod components;
mod block;
mod actor;
mod laser;
use std::{fs::File, io::{BufReader, Read}};
use bevy_inspector_egui::{WorldInspectorPlugin};
use bevy::{prelude::*, utils::{HashSet, HashMap}, sprite::collide_aabb::collide};
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
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
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
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
            .with_system(block_support_scan_system)
            .with_system(block_falling_system.after(block_support_scan_system))
            .with_system(remove_unsupported_block.after(block_falling_system))
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

#[derive(Component)]
pub struct BlockHeat(pub u32);

impl BlockHeat {
    fn new() -> Self {
        BlockHeat(1000)  // avoid that negative situation for  while I guess
    }
}

#[derive(Component)]
pub struct Unsupported(pub Vec3);

#[derive(Component)]
pub struct BlockFalling;

fn block_support_scan_system(mut commands: Commands,
    query_unsupported: Query<&Unsupported>,
    mut query_blocks: Query<(Entity, &mut Transform), With<Block>>
) {
    for unsupported in query_unsupported.iter() {
        for (block_entity, block) in query_blocks.iter_mut() {
            let mut probe_start = unsupported.0;
            let y_length = SCREEN_HEIGHT;      
            probe_start[1] += y_length/2.0; // scan from the top-ish
            let probe_size = Vec2::new(1.0, y_length);
            let target_size = Vec2::new(10.,10.);

            if let Some(_) = collide(probe_start, probe_size, block.translation, target_size){
                commands.entity(block_entity)
                    .insert(BlockFalling);
            }
        }
    }
}

fn _falling_block_group_system(mut _commands: Commands,
    falling_query: Query<(&BlockSize,&Transform), With<BlockFalling>>,
)   {
    const X: usize = 0;
    const Y: usize = 1;
    const Z: usize = 2;
    const ROW_OFFSET: f32 = GRID_WIDTH - BLOCK_SPRITE_OFFSET;

    let _falling_dict = falling_query.iter()
        .map(|(block_size, transform)| {
            let x_y: TupleI32 = (transform.translation[X],transform.translation[Y]).into();

            (x_y, (block_size, transform))
            })
        .into_iter()
        .collect::<HashMap<TupleI32,(&BlockSize,&Transform)>>();
}

#[derive(PartialEq, Eq, Hash)]
struct TupleI32(i32,i32);

impl From<(f32,f32)> for TupleI32 {
    fn from((x,y): (f32, f32)) -> Self {
        TupleI32(x.floor() as i32, y.floor() as i32)
    }
}

fn remove_unsupported_block(mut commands: Commands,
    query: Query<Entity, With<Unsupported>>,
){
    for entity in query.iter(){
        commands.entity(entity).despawn();  // move to next frame
    }
}

fn block_falling_system(mut commands: Commands, 
    mut falling_query: Query<(Entity, &mut Transform, &BlockSize), With<BlockFalling>>,
    collision_query: Query<(&Transform, &BlockSize), (With<Block>, Without<BlockFalling>)>    
){
        for ( falling_entity, mut falling_transform, falling_block) in falling_query.iter_mut() {
            let mut collision: bool = false;
            let falling_block_size;

            match falling_block {
                BlockSize::Large(size) => falling_block_size = Vec2::new(*size as f32,*size as f32),
                BlockSize::Medium(size) => falling_block_size = Vec2::new(*size as f32,*size as f32),
                _ => falling_block_size = Vec2::new(1.0,1.0),
            };

            for (collision_transform, collision_block) in collision_query.iter() {
                let collision_block_size;
            
                match collision_block {
                    BlockSize::Large(size) => collision_block_size = Vec2::new(*size as f32,*size as f32),
                    BlockSize::Medium(size) => collision_block_size = Vec2::new(*size as f32,*size as f32),
                    _ => collision_block_size = Vec2::new(1.0,1.0),
                };

                if let Some(_) = collide(
                    falling_transform.translation - 1.,
                    falling_block_size,
                    collision_transform.translation,
                    collision_block_size,
                ){
                    commands.entity(falling_entity)
                        .remove::<BlockFalling>();
                    
                    collision = true;
                    break;
                }
             }
            
            if !collision {
                falling_transform.translation[1] += -1.0;
            }
        }
    }