use bevy::prelude::*;
use rand::Rng;

use crate::bullet::{Bullet, BulletTimer, BulletOwner, DamageCountdownTimer};

use crate::player::PlayerStatus;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Deref, DerefMut)]
pub struct MoveTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AttackTimer(pub Timer);

#[derive(Component)]
pub struct EnemyHealth(pub u32);

#[derive(Component)]
pub struct OriginalPosition(pub Vec3);

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
) -> Entity {
    let texture = asset_server.load("mobs/chimpanzini.png");
    let mut sprite = Sprite::from_image(texture);
    sprite.flip_x = true;
    commands
        .spawn((
            sprite,
            Transform::from_translation(position).with_scale(Vec3::splat(0.1)),
            MoveTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
            AttackTimer(Timer::from_seconds(3., TimerMode::Repeating)),
            EnemyHealth(5),
            OriginalPosition(position),
            Enemy,
        ))
        .id()
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut MoveTimer, &OriginalPosition), With<Enemy>>,
) {
    for (mut transform, mut timer, original_position) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.x -= 5.2;
            transform.translation.y =
                original_position.0.y + (transform.translation.x / 30.).sin() * 120.;
        }
    }
}

pub fn damaged(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut DamageCountdownTimer), With<Enemy>>,
) {
    for (entity, mut sprite, mut damage_timer) in &mut query {
        sprite.color = Color::srgba(1.0, 0.0, 0.0, 0.8);
        damage_timer.tick(time.delta());
        if damage_timer.just_finished() {
            commands.entity(entity).remove::<DamageCountdownTimer>();
            commands.entity(entity).remove::<AudioPlayer>();
            sprite.color = Color::WHITE;
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    mut query: Query<(Entity, &EnemyHealth), With<Enemy>>,
    mut player_query: Query<&mut PlayerStatus>,
) {
    for (entity, health) in &mut query {
        if health.0 == 0 {
            commands.entity(entity).despawn();
            if let Ok(mut player_status) = player_query.single_mut() {
                player_status.score += 10;
            }
        }
    }
}

pub fn attack(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&Transform, &mut AttackTimer), With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    for (transform, mut timer) in &mut query {
        timer.tick(time.delta());
        if timer.finished() {
            let mut rng = rand::rng();

            if rng.random_bool(0.5) {
                commands.spawn((
                    Sprite::from_image(asset_server.load("particles/banana.png")),
                    Transform::from_translation(transform.translation)
                        .with_scale(Vec3::splat(0.05)),
                    BulletTimer(Timer::from_seconds(0.02, TimerMode::Repeating)),
                    Bullet {
                        owner: BulletOwner::Enemy,
                        damage: 1,
                        movement: |transform| {
                            let mut new_transform = transform;
                            new_transform.translation.x -= 5.0;

                            new_transform.rotate(Quat::from_rotation_z(-0.2));

                            new_transform
                        },
                    },
                ));
            }
        }
    }
}
