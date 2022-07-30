use crate::*;
use bevy::prelude::*;

pub const BLOCK_LARGE_SPRITE_SIZE: (f32, f32) = (64.0, 64.0);
pub const BLOCK_LARGE_SPRITE: &str = "base64/metalCenterSticker-64.png";
pub const BLOCK_LARGE_SPRITE_OFFSET: f32 = BLOCK_LARGE_SPRITE_SIZE.0 / 2.0;

pub const BLOCK_SUPPORT_SPRITE_SIZE: (f32, f32) = (64.0, 64.0);
pub const BLOCK_SUPPORT_SPRITE: &str = "base64/beamBoltsHoles-64.png";
pub const BLOCK_SUPPORT_SPRITE_OFFSET: f32 = BLOCK_LARGE_SPRITE_SIZE.0 / 2.0;

pub const BLOCK_MEDIUM_SPRITE_SIZE: (f32, f32) = (16.0, 16.0);
pub const BLOCK_MEDIUM_SIZE: f32 = 16.;
pub const BLOCK_MEDIUM_SPRITE: &str = "base64/metalCenterWarning-16.png";
pub const _BLOCK_MEDIUM_SCALE: f32 = 1.0;
pub const _BLOCK_MEDIUM_SPRITE_OFFSET: f32 = BLOCK_LARGE_SPRITE_SIZE.0 / 2.0;

pub const GRID_WIDTH: f32 = 64.;
const NUMBER_COLS: usize = 16;

pub fn block_large_setup_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    block_map: Res<BlockMap>,
) {
    let blocks = get_blocks_from_map('#', block_map);

    for (x, y) in blocks {
        let (screen_x, screen_y) = (
            x as f32 * GRID_WIDTH - SCREEN_WIDTH / 2. + BLOCK_LARGE_SPRITE_OFFSET,
            -(y as f32 * GRID_WIDTH - SCREEN_HEIGHT / 2. + BLOCK_LARGE_SPRITE_OFFSET),
        );
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.block_large.clone(),
                transform: Transform {
                    translation: Vec3::new(screen_x, screen_y, 2.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(SpriteSize::from(BLOCK_LARGE_SPRITE_SIZE))
            .insert(Block)
            .insert(BlockHeat::new())
            .insert(BlockSize::Large(64));
    }
}

fn get_blocks_from_map(block_selector: char, block_map: Res<BlockMap>) -> Vec<(usize, usize)> {
    block_map
        .0
        .iter()
        .enumerate()
        .filter(|(i, x)| char::from(**x) == block_selector)
        .map(|(n, _)| {
            let x: usize = n % (NUMBER_COLS);
            let y: usize = n / (NUMBER_COLS);
            (x, y)
        })
        .collect::<Vec<(usize, usize)>>()
}

pub fn block_support_setup_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    block_map: Res<BlockMap>,
) {
    let blocks = get_blocks_from_map('S', block_map);
    dbg!(&blocks);

    for (x, y) in blocks {
        let (screen_x, screen_y) = (
            x as f32 * GRID_WIDTH - SCREEN_WIDTH / 2. + BLOCK_LARGE_SPRITE_OFFSET,
            -(y as f32 * GRID_WIDTH - SCREEN_HEIGHT / 2. + BLOCK_LARGE_SPRITE_OFFSET),
        );
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.block_support.clone(),
                transform: Transform {
                    translation: Vec3::new(screen_x, screen_y, 2.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(SpriteSize::from(BLOCK_SUPPORT_SPRITE_SIZE))
            .insert(Block)
            .insert(BlockHeat::new())
            .insert(BlockSize::Large(64));
    }
}

pub fn block_map_setup_system(mut commands: Commands) {
    let map_input = File::open("assets/map.txt").expect("no Map file found.");

    let mut reader = BufReader::new(map_input);
    let mut file_buffer: Vec<u8> = Vec::new();

    if reader.read_to_end(&mut file_buffer).is_ok() {
        file_buffer.retain(|x| char::from(*x) != '\n');

        commands.insert_resource(BlockMap(file_buffer));
    }
}

pub fn block_decimate_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(&BlockToDecimate, &BlockSize)>,
) {
    for (target_block, block_size) in query.iter() {
        if let BlockSize::Medium(_) = block_size {
            continue;
        }
        const MEDIUM_ROW_RATIO: f32 = 4.0; // :large
        const OFFSET: f32 = 24.;

        let x = target_block.0.x - OFFSET;
        let y = target_block.0.y - OFFSET;

        for row in 0..MEDIUM_ROW_RATIO as i32 {
            for col in 0..MEDIUM_ROW_RATIO as i32 {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: game_textures.block_medium.clone(),
                        transform: Transform {
                            translation: Vec3::new(
                                x + row as f32 * BLOCK_MEDIUM_SIZE,
                                y + col as f32 * BLOCK_MEDIUM_SIZE,
                                10.0,
                            ),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(SpriteSize::from(BLOCK_MEDIUM_SPRITE_SIZE))
                    .insert(Block)
                    .insert(BlockHeat::new())
                    .insert(BlockSize::Medium(10));
            }
        }
    }
}
