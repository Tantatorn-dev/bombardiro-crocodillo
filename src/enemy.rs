use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Deref, DerefMut)]
pub struct EnemyTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct DamageCountdownTimer(pub Timer);

#[derive(Component)]
pub struct EnemyHealth(pub u32);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("mobs/chimpanzini.png");

    let mut sprite = Sprite::from_image(texture);

    sprite.flip_x = true;

    commands.spawn((
        sprite,
        Transform::from_xyz(520., 0., 0.).with_scale(Vec3::splat(0.1)),
        EnemyTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
        EnemyHealth(5),
        Enemy,
    ));
}

pub fn animate(time: Res<Time>, mut query: Query<(&mut Transform, &mut EnemyTimer), With<Enemy>>) {
    for (mut transform, mut timer) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.x -= 5.2;
            transform.translation.y = (transform.translation.x / 30.).sin() * 120.;
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
            sprite.color = Color::WHITE;
        }
    }
}

pub fn despawn(mut commands: Commands, mut query: Query<(Entity, &EnemyHealth), With<Enemy>>) {
    for (entity, health) in &mut query {
        if health.0 == 0 {
            commands.entity(entity).despawn();
        }
    }
}
