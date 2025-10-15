use bevy::prelude::*;

use crate::player::PlayerBullet;

#[derive(Component)]
pub struct Bullet;

#[derive(Component, Deref, DerefMut)]
pub struct BulletTimer(Timer);

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Transform, With<PlayerBullet>>,
) {
    for transform in &mut query {
        let texture = asset_server.load("bullet.png");

        let sprite = Sprite::from_image(texture);

        commands.spawn((
            sprite,
            transform.clone(),
            BulletTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Bullet,
        ));
    }
}

pub fn fly(time: Res<Time>, mut query: Query<(&mut BulletTimer, &mut Transform), With<Bullet>>) {
    for (mut timer, mut transform) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            transform.translation.x += 10.;
        }
    }
}
