use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Component, Deref, DerefMut)]
pub struct BulletTimer(pub Timer);

// Bullet spawning is now handled in player::shoot

pub fn fly(time: Res<Time>, mut query: Query<(&mut BulletTimer, &mut Transform), With<Bullet>>) {
    for (mut timer, mut transform) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.x += 7.5;
        }
    }
}
