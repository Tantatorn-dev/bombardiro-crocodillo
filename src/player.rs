use crate::bullet::{Bullet, BulletDamage, BulletMovement, BulletTimer};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerStatus {
    pub health: u32,
    pub score: u32,
    pub ammo: PlayerAmmo,
}

const MAX_HEALTH: u32 = 10;

enum PlayerAmmo {
    Standard,
    Spread,
    Explosive,
}

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("mobs/bombardiro.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(256), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 3 };

    let mut sprite = Sprite::from_atlas_image(
        texture,
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
    );

    sprite.flip_x = true;

    commands.spawn((
        sprite,
        Transform::from_xyz(-520., 0., 0.).with_scale(Vec3::splat(0.5)),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        PlayerStatus {
            health: MAX_HEALTH,
            score: 0,
            ammo: PlayerAmmo::Standard,
        },
        Player,
    ));
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

pub fn control(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in &mut query {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
            let speed = 300.0;
            transform.translation.x += direction.x * speed * time.delta_secs();
            transform.translation.y += direction.y * speed * time.delta_secs();
        }
    }
}

pub fn shoot(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for transform in &mut query {
            let mut bullet_transform = transform.clone();
            bullet_transform.translation.x += 60.0;
            bullet_transform.translation.y += 40.0;

            let texture = asset_server.load("particles/bullet.png");
            let sprite = Sprite::from_image(texture);

            commands.spawn((
                sprite,
                bullet_transform,
                AudioPlayer::new(asset_server.load("audio/fx/blaster.ogg")),
                BulletTimer(Timer::from_seconds(0.01, TimerMode::Repeating)),
                BulletDamage(1),
                BulletMovement(|position| position + Vec3::new(7.5, 0.0, 0.0)),
                Bullet,
            ));
        }
    }
}
