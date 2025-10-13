use bevy::prelude::*;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, player::setup).chain())
        .add_systems(Update, (player::animate, player::control))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
