use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            // center children
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
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
        ),],
    ));
}
