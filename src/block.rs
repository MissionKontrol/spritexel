use crate::*;
use bevy::prelude::*;

pub const BLOCK_LARGE_SPRITE_SIZE: (f32, f32) = (64.0, 64.0);

pub const BLOCK_LARGE_SPRITE: &str = "base64/metalCenterSticker-64.png";
pub const _BLOCK_SCALE: f32 = 1.0;
pub const BLOCK_LARGE_SPRITE_OFFSET: f32 = BLOCK_LARGE_SPRITE_SIZE.0 / 2.0;

pub const BLOCK_MEDIUM_SPRITE_SIZE: (f32, f32) = (16.0, 16.0);
pub const BLOCK_MEDIUM_SIZE: f32 = 16.;
pub const BLOCK_MEDIUM_SPRITE: &str = "base64/metalCenterWarning-16.png";
pub const _BLOCK_MEDIUM_SCALE: f32 = 1.0;
pub const _BLOCK_MEDIUM_SPRITE_OFFSET: f32 = BLOCK_LARGE_SPRITE_SIZE.0 / 2.0;

pub const GRID_WIDTH: f32 = 64.;

pub fn block_large_setup_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    block_map: Res<BlockMap>,
) {
    let initial_y_position = -(SCREEN_HEIGHT / 2.0) + BLOCK_LARGE_SPRITE_OFFSET; // align bottom
    for (x, y) in block_map.0.iter() {
        let (screen_x, screen_y) = (
            *x as f32 * GRID_WIDTH - SCREEN_WIDTH / 2. ,
            initial_y_position + (BLOCK_LARGE_SPRITE_SIZE.1 * *y as f32), 
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
            .insert(BlockSize::Large(70));
    }
}

pub fn _block_medium_setup_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    let (mut x, mut y) = (
        SCREEN_WIDTH_OFFSET - _BLOCK_MEDIUM_SPRITE_OFFSET,
        -SCREEN_HEIGHT_OFFSET + _BLOCK_MEDIUM_SPRITE_OFFSET,
    );
    (x, y) = (x, y + 55.0);

    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.block_medium.clone(),
            transform: Transform {
                scale: Vec3::new(_BLOCK_MEDIUM_SCALE, _BLOCK_MEDIUM_SCALE, 1.),
                translation: Vec3::new(x, y, 2.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SpriteSize::from(BLOCK_MEDIUM_SPRITE_SIZE))
        .insert(Block)
        .insert(BlockHeat::new())
        .insert(BlockSize::Medium(10));
}

pub fn block_map_setup_system(mut commands: Commands) {
    const NUMBER_COLS: usize = 16;

    let map_input = File::open("assets/map.txt").expect("no Map file found.");

    let mut reader = BufReader::new(map_input);
    let mut file_buffer: Vec<u8> = Vec::new();

    if reader.read_to_end(&mut file_buffer).is_ok() {
        let blocks_to_spawn = file_buffer
        .iter()
        .enumerate()
        .filter(|(_, x)| char::from(**x) == '#')
        .map(|(n, _)| {
            let x: usize = n % (NUMBER_COLS + 1); // guess the \n is a countable char too
            let y: usize = n / (NUMBER_COLS + 1); // so add it in
            (x, y)
        })
        .collect::<Vec<(usize, usize)>>();
        dbg!(&blocks_to_spawn);

        commands.insert_resource(BlockMap(blocks_to_spawn));
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
