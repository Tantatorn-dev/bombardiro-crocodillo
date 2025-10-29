use crate::{
    enemy::*,
    player::{Player, PlayerStatus},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub owner: BulletOwner,
    pub damage: u32,
    pub movement: fn(Transform) -> Transform,
}

#[derive(PartialEq)]
pub enum BulletOwner {
    Player,
    Enemy,
}

#[derive(Component, Deref, DerefMut)]
pub struct BulletTimer(pub Timer);

pub fn fly(
    time: Res<Time>,
    mut query: Query<(&mut BulletTimer, &mut Transform, &Bullet), With<Bullet>>,
) {
    for (mut timer, mut transform, bullet) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            *transform = (bullet.movement)(*transform);
        }
    }
}

pub fn collide(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &Transform, &Bullet), With<Bullet>>,
    mut enemy_query: Query<(Entity, &Transform, &mut EnemyHealth), With<Enemy>>,
    mut player_query: Query<(Entity, &Transform, &mut PlayerStatus), With<Player>>,
    asset_server: Res<AssetServer>,
) {
    for (bullet_entity, bullet_transform, bullet) in &mut bullet_query {
        for (enemy_entity, enemy_transform, mut enemy_health) in &mut enemy_query {
            let distance = bullet_transform
                .translation
                .distance(enemy_transform.translation);

            // Check for collision based on a distance threshold
            if distance < 30.0 && bullet.owner != BulletOwner::Enemy {
                // Despawn the bullet
                commands.entity(bullet_entity).despawn();
                // Decrease enemy health
                enemy_health.0 = enemy_health.0.saturating_sub(bullet.damage);

                commands.entity(enemy_entity).insert((
                    DamageCountdownTimer(Timer::from_seconds(1.0, TimerMode::Once)),
                    AudioPlayer::new(asset_server.load("audio/fx/hit.ogg")),
                ));

                break;
            }
        }

        for (player_entity, player_transform, mut player_status) in &mut player_query {
            let distance = bullet_transform
                .translation
                .distance(player_transform.translation);

            // Check for collision based on a distance threshold
            if distance < 50.0 && bullet.owner != BulletOwner::Player {
                // Despawn the bullet
                commands.entity(bullet_entity).despawn();
                // Decrease player health
                player_status.health = player_status.health.saturating_sub(bullet.damage);

                commands
                    .entity(player_entity)
                    .insert((AudioPlayer::new(asset_server.load("audio/fx/hit.ogg")),));

                break;
            }
        }
    }
}
