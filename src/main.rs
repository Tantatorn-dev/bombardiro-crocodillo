use bevy::prelude::*;

mod background;
mod bullet;
mod enemy;
mod game_over;
mod hud;
mod player;
mod spawn;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((game_plugin, game_over_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
    Paused,
}

fn game_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Playing),
        (player::setup, background::setup, spawn::setup, hud::setup),
    )
    .add_systems(
        Update,
        (
            player::animate,
            background::animate,
            player::control,
            player::shoot,
            bullet::fly,
            bullet::collide,
            spawn::spawn,
            enemy::animate,
            enemy::damaged,
            enemy::despawn,
            player::damaged,
            hud::update,
            enemy::attack,
            bullet::despawn,
            enemy::despawn,
            player::game_over,
        )
            .run_if(in_state(GameState::Playing)),
    );
}

pub fn game_over_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::GameOver), game_over::setup)
        .add_systems(Update, game_over::control.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
        .add_systems(OnExit(GameState::Playing), cleanup_playing);
}

#[derive(Component)]
pub struct PlayingEntity;

#[derive(Component)]
pub struct GameOverEntity;

fn cleanup_playing(mut commands: Commands, query: Query<Entity, With<PlayingEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
