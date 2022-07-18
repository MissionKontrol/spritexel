use bevy::prelude::*;
use crate::*;

pub const BLOCK_SPRITE_SIZE: (f32, f32) = (70.0, 70.0);
pub const BLOCK_MEDIUM_SPRITE_SIZE: (f32, f32) = (10.0, 10.0);

pub const BLOCK_SPRITE: &str = "metalCenter.png";
pub const _BLOCK_SCALE: f32 = 1.0;
pub const BLOCK_SPRITE_OFFSET: f32 = BLOCK_SPRITE_SIZE.0 / 2.0;

pub const BLOCK_MEDIUM_SPRITE: &str = "metalSmallCenterSticker.png";
pub const BLOCK_MEDIUM_SCALE: f32 = 1.0;
pub const BLOCK_MEDIUM_SPRITE_OFFSET: f32 = BLOCK_SPRITE_SIZE.0 / 2.0;


#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub enum BlockSize {
    _Small(u8),
    Medimum(u8),
    Large(u8),
}

pub fn block_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let (x, y) = (
        SCREEN_WIDTH_OFFSET - BLOCK_SPRITE_OFFSET,
        -SCREEN_HEIGHT_OFFSET + BLOCK_SPRITE_OFFSET,
    );
    // let (x,y) = (0.0 ,0.0);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(BLOCK_SPRITE),
            transform: Transform {
                // scale: Vec3::new(1.4285, 1.4285, 1.), // 10/7 - scale to 100px
                translation: Vec3::new(x, y, 2.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SpriteSize::from(BLOCK_SPRITE_SIZE))
        .insert(Block)
        .insert(BlockSize::Large(100));
}

pub fn block_medium_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let (mut x, mut y) = (
        SCREEN_WIDTH_OFFSET - BLOCK_MEDIUM_SPRITE_OFFSET,
        -SCREEN_HEIGHT_OFFSET + BLOCK_MEDIUM_SPRITE_OFFSET,
    );
    (x, y) = (x, y + 55.0);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(BLOCK_MEDIUM_SPRITE),
            transform: Transform {
                scale: Vec3::new(BLOCK_MEDIUM_SCALE, BLOCK_MEDIUM_SCALE, 1.),
                translation: Vec3::new(x, y, 2.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SpriteSize::from(BLOCK_MEDIUM_SPRITE_SIZE))
        .insert(Block)
        .insert(BlockSize::Medimum(10));
}
