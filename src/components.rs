use bevy::prelude::*;

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Component)]
pub struct GameTextures {
    pub actor_animation_sprite: Handle<TextureAtlas>,
    pub explosion_animation_sprite: Handle<TextureAtlas>,
    pub actor: Handle<Image>,
    pub block_large: Handle<Image>,
    pub block_medium: Handle<Image>,
    // pub laser: Handle<Image>,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Actor;

#[derive(Component)]
pub struct Laser;

#[derive(Component, Clone)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3);

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, true))
    }
}


#[derive(Component)]
pub struct BlockToDecimate(pub Vec3);