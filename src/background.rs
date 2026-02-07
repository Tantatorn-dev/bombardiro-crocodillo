use bevy::prelude::*;
use crate::PlayingEntity;

#[derive(Component, DerefMut, Deref)]
pub struct BackgroundTimer(pub Timer);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let base_texture = asset_server.load("bg/1.png");

    let moon_texture = asset_server.load("bg/2.png");

    let cloud_texture_1 = asset_server.load("bg/3.png");

    let cloud_texture_2 = asset_server.load("bg/4.png");

    commands.spawn((
        Sprite::from_image(base_texture),
        Transform::from_xyz(0., 0., -4.).with_scale(Vec3::splat(2.3)),
        PlayingEntity,
    ));

    commands.spawn((
        Sprite::from_image(moon_texture),
        Transform::from_xyz(0., 0., -3.).with_scale(Vec3::splat(2.3)),
        PlayingEntity,
    ));

    commands.spawn((
        Sprite::from_image(cloud_texture_1.clone()),
        Transform::from_xyz(0., 0., -2.).with_scale(Vec3::splat(2.3)),
        BackgroundTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        PlayingEntity,
    ));

    commands.spawn((
        Sprite::from_image(cloud_texture_1.clone()),
        Transform::from_xyz(1300., 0., -2.).with_scale(Vec3::splat(2.3)),
        BackgroundTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        PlayingEntity,
    ));

    commands.spawn((
        Sprite::from_image(cloud_texture_2.clone()),
        Transform::from_xyz(0., 0., -1.).with_scale(Vec3::splat(2.3)),
        BackgroundTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        PlayingEntity,
    ));

    commands.spawn((
        Sprite::from_image(cloud_texture_2.clone()),
        Transform::from_xyz(1300., 0., -1.).with_scale(Vec3::splat(2.3)),
        BackgroundTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
        PlayingEntity,
    ));
}

pub fn animate(time: Res<Time>, mut query: Query<(&mut Transform, &mut BackgroundTimer)>) {
    for (mut transform, mut timer) in &mut query {
	timer.tick(time.delta());
	if timer.just_finished() {
	    transform.translation.x -= 1.5;

	    if transform.translation.x < -1300. {
		transform.translation.x = 1300.;
	    }
	}
    }

}