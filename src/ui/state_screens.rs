use bevy::prelude::*;
use crate::GameStates;

#[derive(Component)]
pub struct PlayAgainButton;

#[derive(Component)]
pub struct WinScreen;

#[derive(Component)]
pub struct LossScreen;

pub fn win_screen(mut commands: Commands) {
    commands.spawn((
        WinScreen,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)), // Semi-transparent background
    )).with_children(|parent| {
        //Winner text
        parent.spawn((
            Text::new("YOU WIN!"),
            TextFont {
                font_size: 60.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 1.0, 0.0)),
        ));
        // Play Again button
        parent.spawn((
            Button,
            PlayAgainButton,
            Node { 
                width: Val::Px(200.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(20.0)),
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.6, 0.1)),
        )).with_children(|button_parent| {
            button_parent.spawn((
                Text::new("Play Again"),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn loss_screen(mut commands: Commands) {
    commands.spawn((
        LossScreen,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)), // Semi-transparent background
    )).with_children(|parent| {
        //Player lost text
        parent.spawn((
            Text::new("GAME OVER"),
            TextFont {
                font_size: 60.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.0, 0.0)),
        ));
        // Play Again button
        parent.spawn((
            Button,
            PlayAgainButton,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(20.0)),
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.6, 0.1)),
        )).with_children(|button_parent| {
            button_parent.spawn((
                Text::new("Play Again"),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    });
}


pub fn play_again_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayAgainButton>, With<Button>)
    >,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Set the next state to Playing when button is clicked
                crate::restart_game();
                println!("Play Again button pressed - restarting...");
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.2, 0.7, 0.2));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.1, 0.6, 0.1));
            }
        }
    }
}

pub fn cleanup_win_screen(
    mut commands: Commands,
    query: Query<Entity, With<WinScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_loss_screen(
    mut commands: Commands,
    query: Query<Entity, With<LossScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}