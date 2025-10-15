use bevy::prelude::*;

mod bullet;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, player::setup))
        .add_systems(Update, (player::animate, player::control, player::shoot, bullet::spawn, bullet::fly))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
