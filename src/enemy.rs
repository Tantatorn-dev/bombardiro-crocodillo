use bevy::prelude::*;

use crate::player::PlayerStatus;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Deref, DerefMut)]
pub struct EnemyTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct DamageCountdownTimer(pub Timer);

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
            EnemyTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
            EnemyHealth(5),
            OriginalPosition(position),
            Enemy,
        ))
        .id()
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut EnemyTimer, &OriginalPosition), With<Enemy>>,
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
                println!("Enemy defeated! +10 score.");
                player_status.score += 10;
            }
        }
    }
}
