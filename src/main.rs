use bevy::prelude::*;

mod background;
mod bullet;
mod enemy;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (setup, player::setup, background::setup, enemy::setup),
        )
        .add_systems(
            Update,
            (
                player::animate,
                player::control,
                player::shoot,
                bullet::fly,
                bullet::collide,
                enemy::animate,
                enemy::damaged,
                enemy::despawn,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
