use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("mobs/chimpanzini.png");

    let mut sprite = Sprite::from_image(texture);

    sprite.flip_x = true;

    commands.spawn((
        sprite,
        Transform::from_xyz(520., 0., 0.).with_scale(Vec3::splat(0.1)),
        Enemy,
    ));
}
