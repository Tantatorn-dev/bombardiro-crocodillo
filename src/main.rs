use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, initialize)
        .add_systems(Update, hello_world)
        .run();
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn initialize(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((Position { x: 0.0, y: 0.0 },));
}

fn hello_world() {
    println!("hello world!");
}
