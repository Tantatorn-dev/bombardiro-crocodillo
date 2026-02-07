use bevy::{prelude::*};

use crate::{GameState, GameOverEntity};

pub fn setup(mut commands: Commands) {
    commands.spawn((
        GameOverEntity,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            // center children
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
	    flex_direction: FlexDirection::Column,
	    row_gap: Val::Px(10.),
            ..default()
        },
        BackgroundColor(Color::BLACK),
        children![(
            Text::new("Game Over"),
            TextFont {
                font_size: 67.0,
                ..default()
            },
            TextColor(Color::srgb(1., 0., 0.)),
            Node {
                margin: UiRect::all(Val::Px(50.)),
                ..default()
            },
        ),
	(
            Text::new("Please Enter to Restart"),
            TextFont {
                font_size: 36.0,
                ..default()
            },
            TextColor(Color::srgb(1., 0., 0.)),
            Node {
                margin: UiRect::all(Val::Px(80.)),
                ..default()
            },
	),
	],
    ));
}


pub fn control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        game_state.set(GameState::Playing);
    }
}
