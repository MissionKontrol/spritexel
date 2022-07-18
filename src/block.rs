use bevy::prelude::*;
use crate::*;

pub const BLOCK_SPRITE_SIZE: (f32, f32) = (70.0, 70.0);

pub const BLOCK_SPRITE: &str = "metalCenter.png";
pub const _BLOCK_SCALE: f32 = 1.0;
pub const BLOCK_SPRITE_OFFSET: f32 = BLOCK_SPRITE_SIZE.0 / 2.0;

pub const BLOCK_MEDIUM_SPRITE_SIZE: (f32, f32) = (10.0, 10.0);
pub const BLOCK_MEDIUM_SIZE: f32 = 10.;
pub const BLOCK_MEDIUM_SPRITE: &str = "metalSmallCenterSticker.png";
pub const BLOCK_MEDIUM_SCALE: f32 = 1.0;
pub const BLOCK_MEDIUM_SPRITE_OFFSET: f32 = BLOCK_SPRITE_SIZE.0 / 2.0;

pub fn block_large_setup_system(mut commands: Commands, game_textures: Res<GameTextures>, raw_map: Res<RawMap>) {
    const NUMBER_COLS: usize = 10;

    let blocks_to_spawn = raw_map.0.iter().enumerate()
        .filter(|(_, x)| char::from(**x) == '#' )
        .map(|(n,_)| {
            let x: usize = n % (NUMBER_COLS + 1);   // guess the \n is a countable char too
            let y: usize = n / (NUMBER_COLS + 1);   // so add it in
            (x,y)
        }).collect::<Vec<(usize,usize)>>();
    
    for (x,y) in blocks_to_spawn.iter() {
        let (screen_x, screen_y) = (
            *x as f32 * 100. - SCREEN_WIDTH / 2. + (BLOCK_SPRITE_OFFSET * (NUMBER_COLS as f32 - *x as f32)) + BLOCK_SPRITE_OFFSET,
            *y as f32 * 100. - SCREEN_HEIGHT / 2. - (BLOCK_SPRITE_OFFSET * *y as f32) + BLOCK_SPRITE_OFFSET,
        );
        commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.block_large.clone(),
            transform: Transform {
                // scale: Vec3::new(1.4285, 1.4285, 1.), // 10/7 - scale to 100px
                translation: Vec3::new(screen_x, screen_y, 2.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SpriteSize::from(BLOCK_SPRITE_SIZE))
        .insert(Block)
        .insert(BlockSize::Large(100));
    }
}

pub fn block_medium_setup_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    let (mut x, mut y) = (
        SCREEN_WIDTH_OFFSET - BLOCK_MEDIUM_SPRITE_OFFSET,
        -SCREEN_HEIGHT_OFFSET + BLOCK_MEDIUM_SPRITE_OFFSET,
    );
    (x, y) = (x, y + 55.0);

    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.block_medium.clone(),
            transform: Transform {
                scale: Vec3::new(BLOCK_MEDIUM_SCALE, BLOCK_MEDIUM_SCALE, 1.),
                translation: Vec3::new(x, y, 2.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SpriteSize::from(BLOCK_MEDIUM_SPRITE_SIZE))
        .insert(Block)
        .insert(BlockSize::Medium(10));
}

pub fn block_map_setup_system(mut commands: Commands) {
    let map_input = File::open("assets/map.txt").expect("no file or something");

    let mut reader = BufReader::new(map_input);
    let mut file_buffer: Vec<u8> =Vec::new();

    if let Ok(_) = reader.read_to_end(&mut file_buffer){
        commands.insert_resource(RawMap(file_buffer));
    }
}

pub fn block_decimate_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(&BlockToDecimate, &BlockSize)>,
) {
    for ( target_block, block_size) in query.iter() {
        if let BlockSize::Medium(_) = block_size {
            continue
        }
        const MEDIUM_ROW_RATIO: f32 = 7.0;    // 7:1 med:large
        const OFFSET: f32 = 30.;

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
                    .insert(BlockSize::Medium(10));
            }
        }
    }
}

