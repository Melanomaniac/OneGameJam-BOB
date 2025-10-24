use bevy::{input_focus::InputFocus, prelude::*};

#[derive(Component)]
enum MenuButton {
    Attack,
    Build,
    Scout,
}

const NORMAL_ATTACK: Color  = Color::srgb(1.0,0.0, 0.0);
const NORMAL_BUILD: Color   = Color::srgb(0.9,0.3, 0.0);
const NORMAL_SCOUT: Color   = Color::srgb(0.0, 0.0, 1.0);
//const PRESSED_COLOR: Color  = Color::WHITE; might use it later for clicked?


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<InputFocus>()
        .add_systems(Startup, setup)
        //.add_systems(Update, button_system)
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
                ..default()
            },
            BackgroundColor(colour),
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