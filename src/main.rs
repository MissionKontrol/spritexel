mod components;
use components::*;
use bevy::prelude::*;

const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 1000.0;
const SCREEN_HEIGHT_OFFSET: f32 = SCREEN_HEIGHT / 2.0;
const SCREEN_WIDTH_OFFSET: f32 = SCREEN_WIDTH / 2.0;

const BLOCK_SPRITE_SIZE: (f32,f32) = (70.0,70.0);

const ACTOR_SPRITE_SIZE: (f32,f32) = (70.0,70.0);


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
    .add_plugin(GameSetup)
    .run();
}

pub struct GameSetup;

impl Plugin for GameSetup {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(actor_setup_system)
        .add_startup_system(block_setup_system)
        .add_startup_system(game_setup_system)
        .add_system(asset_setup_system)
        .add_system(actor_move_system);
        // .add_system(actor_move_system);
    }
}

fn game_setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

}

#[derive(Component)]
pub struct GameAssets{
    pub actor_animation_sprite: Handle<TextureAtlas>,
}

fn asset_setup_system(mut commands: Commands, asset_server: Res<AssetServer>, mut atlas_server: ResMut<Assets<TextureAtlas>> ) {
    const ACTOR_ANIMATE_SPRITE: &str = "laserRightFire-sprites.png";
    const ACTOR_ANIMATE_SPRITE_ROWS: usize = 4;
    const ACTOR_ANIMATE_SPRITE_COLS: usize = 4;

    let texture_handle = asset_server.load(ACTOR_ANIMATE_SPRITE);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(ACTOR_SPRITE_SIZE.0,ACTOR_SPRITE_SIZE.1), ACTOR_ANIMATE_SPRITE_COLS, ACTOR_ANIMATE_SPRITE_ROWS);
    let texture_atlas_handle = atlas_server.add(texture_atlas);

    commands.insert_resource(GameAssets { actor_animation_sprite: texture_atlas_handle});

}

fn block_setup_system(mut commands: Commands, asset_server: Res<AssetServer>){
    const BLOCK_SPRITE: &str = "metalCenter.png";
    const BLOCK_SCALE: f32 = 1.0;
    const BLOCK_SPRITE_OFFSET: f32 = BLOCK_SPRITE_SIZE.0 / 2.0;

    let (x,y) = (SCREEN_WIDTH_OFFSET - BLOCK_SPRITE_OFFSET ,-SCREEN_HEIGHT_OFFSET + BLOCK_SPRITE_OFFSET );
    // let (x,y) = (0.0 ,0.0);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(BLOCK_SPRITE),
            transform: Transform {
                scale: Vec3::new(BLOCK_SCALE, BLOCK_SCALE, 1.),
                translation: Vec3::new(x, y, 2.0),
                ..Default::default()
            },
            ..Default::default()
            });
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Actor;

fn actor_move_system(mut commands: Commands, mut query: Query<(&Velocity, &mut Transform), With<Actor>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x;
        translation.y += velocity.y * 10.0;
    }
}

fn actor_setup_system(mut commands: Commands, asset_server: Res<AssetServer>){
    const ACTOR_SPRITE: &str = "laserUp.png";
    const ACTOR_SCALE: f32 = 1.0;
    const ACTOR_SPRITE_OFFSET: f32 = ACTOR_SPRITE_SIZE.0 / 2.0;

    let (x,y) = (-SCREEN_WIDTH_OFFSET + ACTOR_SPRITE_OFFSET,-SCREEN_HEIGHT_OFFSET + ACTOR_SPRITE_OFFSET);
    // let (x,y) = (0.0,1.0);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(ACTOR_SPRITE),
            transform: Transform {
                scale: Vec3::new(ACTOR_SCALE, ACTOR_SCALE, 1.),
                translation: Vec3::new(x, y, 2.0),
                rotation: Quat::from_rotation_z(4.71_f32),  // rads is 270 degrees counter-clockwise
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Actor)
        .insert(Velocity { x:0.0, y:1.0 });
}