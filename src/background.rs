use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let base_texture = asset_server.load("bg/1.png");

    let moon_texture = asset_server.load("bg/2.png");

    let cloud_texture_1 = asset_server.load("bg/3.png");

    let cloud_texture_2 = asset_server.load("bg/4.png");

    commands.spawn((
        Sprite::from_image(base_texture),
        Transform::from_xyz(0., 0., -1.).with_scale(Vec3::splat(2.3)),
        Background,
    ));

    commands.spawn((
        Sprite::from_image(moon_texture),
        Transform::from_xyz(0., 0., -1.).with_scale(Vec3::splat(2.3)),
        Background,
    ));

    commands.spawn((
        Sprite::from_image(cloud_texture_1),
        Transform::from_xyz(0., 0., -1.).with_scale(Vec3::splat(2.3)),
        Background,
    ));

    commands.spawn((
        Sprite::from_image(cloud_texture_2),
        Transform::from_xyz(0., 0., -1.).with_scale(Vec3::splat(2.3)),
        Background,
    ));
}
