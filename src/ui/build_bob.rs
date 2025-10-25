use bevy::prelude::*;

pub fn setup_build_bob_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Px(300.0),
            height: Val::Px(300.0),
            position_type: PositionType::Absolute,
            right: Val::Px(20.0),
            bottom: Val::Px(20.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            padding: UiRect::all(Val::Px(10.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
        BorderColor::all(Color::srgb(0.5, 0.5, 0.6)),
    )).with_children(|parent| {
        // Head slot (top)
        parent.spawn((
            Node {
                width: Val::Px(60.0),
                height: Val::Px(60.0),
                position_type: PositionType::Relative,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::bottom(Val::Px(5.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ));

        // Body + Arms row (horizontal container)
        parent.spawn(
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::SpaceEvenly,
                margin: UiRect::bottom(Val::Px(5.0)),
                ..default()
            }
        )
        .with_children(|body_arm_parent| {
            // Left arm
            body_arm_parent.spawn((
                Node {
                    width: Val::Px(90.0),
                    height: Val::Px(30.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::BLACK),
            ));

            // Body (center)
            body_arm_parent.spawn((
                Node {
                    width: Val::Px(60.0),
                    height: Val::Px(90.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::BLACK),
            ));

            // Right arm
            body_arm_parent.spawn((
                Node {
                    width: Val::Px(90.0),
                    height: Val::Px(30.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::BLACK),
            ));
        });

        // Legs row (horizontal container for two legs)
        parent.spawn(
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(70.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            }
        )
        .with_children(|leg_parent| {
            // Left leg
            leg_parent.spawn((
                Node {
                    width: Val::Px(30.0),
                    height: Val::Px(85.0),
                    border: UiRect::all(Val::Px(2.0)),
                    margin: UiRect::right(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::BLACK),
            ));

            // Right leg
            leg_parent.spawn((
                Node {
                    width: Val::Px(30.0),
                    height: Val::Px(85.0),
                    border: UiRect::all(Val::Px(2.0)),
                    margin: UiRect::left(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::BLACK),
            ));
        });
    });
}