use crate::enemy::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Component, Deref, DerefMut)]
pub struct BulletTimer(pub Timer);

#[derive(Component)]
pub struct BulletDamage(pub u32);

pub fn fly(time: Res<Time>, mut query: Query<(&mut BulletTimer, &mut Transform), With<Bullet>>) {
    for (mut timer, mut transform) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.x += 7.5;
        }
    }
}

pub fn collide(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &Transform, &BulletDamage), With<Bullet>>,
    mut enemy_query: Query<(Entity, &Transform, &mut EnemyHealth), With<Enemy>>,
) {
    for (bullet_entity, bullet_transform, bullet_damage) in &mut bullet_query {
        for (enemy_entity, enemy_transform, mut enemy_health) in &mut enemy_query {
            let distance = bullet_transform
                .translation
                .distance(enemy_transform.translation);

            // Check for collision based on a distance threshold
            if distance < 30.0 {
                // Despawn the bullet
                commands.entity(bullet_entity).despawn();
                // Decrease enemy health
                enemy_health.0 = enemy_health.0.saturating_sub(bullet_damage.0);

                commands
                    .entity(enemy_entity)
                    .insert(DamageCountdownTimer(Timer::from_seconds(
                        1.0,
                        TimerMode::Once,
                    )));

                break;
            }
        }
    }
}
