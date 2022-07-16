mod components;
use components::*;
use bevy::prelude::*;

const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 1000.0;
const SCREEN_HEIGHT_OFFSET: f32 = SCREEN_HEIGHT / 2.0;
const SCREEN_WIDTH_OFFSET: f32 = SCREEN_WIDTH / 2.0;



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
        .add_system(game_setup_system)
        .add_system(actor_setup_system)
        .add_system(block_setup_system)
        .add_system(asset_setup_system);
    }
}

fn game_setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

}

fn asset_setup_system(mut commands: Commands, asset_server: Res<AssetServer> ) {
}

fn block_setup_system(mut commands: Commands, asset_server: Res<AssetServer>){
    const BLOCK_SPRITE: &str = "metalCenter.png";
    const BLOCK_SPRITE_SIZE: (f32,f32) = (70.0,70.0);
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
pub struct Actor;

fn _actor_move_system(mut commands: Commands, query: Query<(&Transform), With<Actor>>) {
}

fn actor_setup_system(mut commands: Commands, asset_server: Res<AssetServer>){
    const ACTOR_SPRITE: &str = "laserUp.png";
    const _ACTOR_SPRITE_SIZE: (f32,f32) = (70.0,70.0);
    const ACTOR_SCALE: f32 = 1.0;
    const ACTOR_SPRITE_OFFSET: f32 = _ACTOR_SPRITE_SIZE.0 / 2.0;

    let (x,y) = (-SCREEN_WIDTH_OFFSET + ACTOR_SPRITE_OFFSET,-SCREEN_HEIGHT_OFFSET + ACTOR_SPRITE_OFFSET);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(ACTOR_SPRITE),
            transform: Transform {
                scale: Vec3::new(ACTOR_SCALE, ACTOR_SCALE, 1.),
                translation: Vec3::new(x, y, 2.0),
                rotation: Quat::from_rotation_z(4.71_f32),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Actor);
}