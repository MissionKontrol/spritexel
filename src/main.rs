mod components;
use components::*;
use bevy::prelude::*;

const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 1000.0;
const SCREEN_HEIGHT_OFFSET: f32 = SCREEN_HEIGHT / 2.0;
const SCREEN_WIDTH_OFFSET: f32 = SCREEN_WIDTH / 2.0;

const BLOCK_SPRITE_SIZE: (f32,f32) = (70.0,70.0);
const _BLOCK_SMALL_SPRITE_SIZE: (f32,f32) = (10.0,10.0);

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

pub struct WinSize {
	pub w: f32,
	pub h: f32,
}

pub struct GameSetup;

impl Plugin for GameSetup {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(actor_setup_system)
        .add_startup_system(block_setup_system)
        .add_startup_system(block_small_setup_system)

        .add_startup_system(game_setup_system)
        .add_system(asset_setup_system)
        .add_system(actor_keyboard_event_system)
        .add_system(actor_move_system)
        .add_system(player_laser_spawn_system)
        .add_system(laser_move_system);
    }
}

fn game_setup_system(mut commands: Commands, windows: Res<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary().unwrap();
    let (win_width, win_height) = (window.width(), window.height());
    let win_size = WinSize{ w: win_width, h: win_height };
    commands.insert_resource(win_size);
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
                scale: Vec3::new(1.4285, 1.4285, 1.),
                translation: Vec3::new(x, y, 2.0),
                ..Default::default()
            },
            ..Default::default()
            });
}

fn block_small_setup_system(mut commands: Commands, asset_server: Res<AssetServer>){
    const BLOCK_SMALL_SPRITE: &str = "metalSmallCenterSticker.png";
    const BLOCK_SMALL_SCALE: f32 = 1.0;
    const BLOCK_SMALL_SPRITE_OFFSET: f32 = BLOCK_SPRITE_SIZE.0 / 2.0;

    let (mut x,mut y) = (SCREEN_WIDTH_OFFSET - BLOCK_SMALL_SPRITE_OFFSET ,-SCREEN_HEIGHT_OFFSET + BLOCK_SMALL_SPRITE_OFFSET );
    (x,y) = (x ,y + 55.0);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(BLOCK_SMALL_SPRITE),
            transform: Transform {
                scale: Vec3::new(BLOCK_SMALL_SCALE, BLOCK_SMALL_SCALE, 1.),
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

fn actor_move_system( mut query: Query<(&Velocity, &mut Transform), With<Actor>>) {
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

fn actor_keyboard_event_system(
	kb: Res<Input<KeyCode>>,
	mut query: Query<&mut Velocity, With<Actor>>,
) {
	if let Ok(mut velocity) = query.get_single_mut() {
		velocity.y = if kb.pressed(KeyCode::Down) {
			-1.
		} else if kb.pressed(KeyCode::Up) {
			1.
		} else {
			0.
		}
	}
}

#[derive(Component)]
pub struct Laser;

fn player_laser_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>, kb: Res<Input<KeyCode>>,
    query: Query<(&Transform), With<Actor>>) {
    const LASER_SPRITE: &str = "laserGreenHorizontal.png";
    // const LASER_SPRITE_SIZE: (f32,f32) = (70.0,70.0);
    const LASER_SCALE: f32 = 1.0;
    // const LASER_SPRITE_OFFSET: f32 = LASER_SPRITE_SIZE.0 / 2.0;

    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x,y) = (player_tf.translation.x, player_tf.translation.y);

            commands.spawn_bundle(SpriteBundle {
                texture: asset_server.load(LASER_SPRITE),
                transform: Transform {
                    scale: Vec3::new(LASER_SCALE, LASER_SCALE, 1.),
                    translation: Vec3::new(x + ACTOR_SPRITE_SIZE.0 , y, 2.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Velocity{x: 5.0, y: 0.0})
            .insert(Laser);
        }
    }
}

fn laser_move_system(mut commands: Commands, mut query: Query<(Entity, &Velocity, &mut Transform), With<Laser>>, win_size: Res<WinSize>) {
    for (entity, velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * 10.0;

        if translation.x >= win_size.w / 2.0 {
            commands.entity(entity).despawn();
        }
    }



}

