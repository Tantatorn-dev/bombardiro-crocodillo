use bevy::prelude::*;
use rand::Rng;
use crate::{enemy::spawn_enemy, PlayingEntity};

#[derive(Component, Deref, DerefMut)]
pub struct SpawnTimer(pub Timer);

pub fn setup(mut commands: Commands) {
    commands.spawn((
        SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
        PlayingEntity,
    ));
}

pub fn spawn(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<&mut SpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    for mut spawn_timer in &mut query {
        spawn_timer.tick(time.delta());
        if spawn_timer.just_finished() {
                let mut rng = rand::rng();

                let y_rand = rng.random_range(-220.0..220.0);

                if rng.random_bool(0.3) {
                    continue;
                }

                spawn_enemy(&mut commands, &asset_server, Vec3::new(620., y_rand, 0.));
        }
    }
}