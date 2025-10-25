use bevy::{input_focus::InputFocus, prelude::*};

#[derive(Component)]
enum MenuButton {
    Attack,
    Build,
    Scout,
}

const NORMAL_ATTACK: Color = Color::srgb(1.0,0.0, 0.0);
const NORMAL_BUILD: Color = Color::srgb(0.9,0.3, 0.0);
const NORMAL_SCOUT: Color = Color::srgb(0.0, 0.0, 1.0);

const HOVER_COLOR: Color =  Color::WHITE;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<InputFocus>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let mut root = commands.spawn(
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..default()
        }
    );

    spawn_button(&mut root, MenuButton::Attack, "Attack!", NORMAL_ATTACK);
    spawn_button(&mut root, MenuButton::Build,  "Build!",  NORMAL_BUILD);
    spawn_button(&mut root, MenuButton::Scout,  "Scout!",  NORMAL_SCOUT);
}

fn spawn_button(
    parent: &mut EntityCommands,
    kind: MenuButton,
    label: &str,
    colour: Color,
) {
    parent.with_children(|parent| {
        parent.spawn((
            Button,
            Node {
                width: Val::Px(65.0),
                height: Val::Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(colour),
            BorderColor::all(colour),
            kind,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
    });
}

fn button_system(
    mut interaction_query: Query<(&Interaction, &MenuButton, &mut BackgroundColor, &mut BorderColor ), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, button_type, mut bg_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let original_color = match button_type {
                    MenuButton::Attack => NORMAL_ATTACK,
                    MenuButton::Build => NORMAL_BUILD,
                    MenuButton::Scout => NORMAL_SCOUT,
                };
                let click_color = Color::srgb(
                    original_color.to_srgba().red * 0.7,
                    original_color.to_srgba().green * 0.7,
                    original_color.to_srgba().blue * 0.7,
                );
                *bg_color = click_color.into();
            },

            Interaction::Hovered => {
                *border_color = HOVER_COLOR.into();
                let original_color = match button_type {
                    MenuButton::Attack => NORMAL_ATTACK,
                    MenuButton::Build => NORMAL_BUILD,
                    MenuButton::Scout => NORMAL_SCOUT,
                };
                *bg_color = original_color.into();
            },
            
            Interaction::None => {
                // Reset everything
                let original_color = match button_type {
                    MenuButton::Attack => NORMAL_ATTACK,
                    MenuButton::Build => NORMAL_BUILD,
                    MenuButton::Scout => NORMAL_SCOUT,
                };
                *bg_color = original_color.into();
                *border_color = original_color.into();
            },
        }
    }
}