use bevy::prelude::*;

use crate::player::PlayerStatus;

#[derive(Component)]
pub struct Hud;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Text::new("Health: 10\nScore: 0"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.),
            left: Val::Px(15.),
            ..default()
        },
        Hud,
    ));
}

pub fn update(mut hud_query: Query<&mut Text, With<Hud>>, mut player_query: Query<&PlayerStatus>) {
    for mut text in &mut hud_query {
        if let Ok(player_status) = player_query.single_mut() {
            **text = format!(
                "Health: {}\nScore: {}",
                player_status.health, player_status.score
            );
        }
    }
}
