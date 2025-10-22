use bevy::prelude::*;

mod background;
mod bullet;
mod enemy;
mod hud;
mod player;
mod spawn;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                setup,
                player::setup,
                background::setup,
                spawn::setup,
                hud::setup,
            ),
        )
        .add_systems(
            Update,
            (
                player::animate,
                background::animate,
                player::control,
                player::shoot,
                bullet::fly,
                bullet::collide,
                spawn::spawn,
                enemy::animate,
                enemy::damaged,
                enemy::despawn,
                hud::update,
                enemy::attack,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
