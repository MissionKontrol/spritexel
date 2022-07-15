use bevy::prelude::*;

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        title: "I am a window!".to_string(),
        mode: bevy::window::WindowMode::Windowed,
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .run();
}
