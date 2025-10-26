use bevy::prelude::*;
use crate::ComponentsInventory;

const GREEN_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);

#[derive(Event)]
pub struct ResetBuilderUIEvent;

// Add component markers to identify each body part slot
#[derive(Component)]
pub struct HeadSlot;

#[derive(Component)]
pub struct BodySlot;

#[derive(Component)]
pub struct LeftArmSlot;

#[derive(Component)]
pub struct RightArmSlot;

#[derive(Component)]
pub struct LeftLegSlot;

#[derive(Component)]
pub struct RightLegSlot;

// Component to track if a slot is filled
#[derive(Component)]
pub struct SlotFilled(pub bool);

pub fn on_reset_ui(
    _trigger: On<ResetBuilderUIEvent>,
    mut slot_query: Query<(Entity, &mut SlotFilled, &mut BackgroundColor), Or<(With<HeadSlot>, With<BodySlot>, With<LeftArmSlot>, With<RightArmSlot>, With<LeftLegSlot>, With<RightLegSlot>)>>,
) {
    for (entity, mut slot_filled, mut bg_color) in slot_query.iter_mut() {
        if slot_filled.0 {
            slot_filled.0 = false;
            *bg_color = BackgroundColor(Color::BLACK);
            println!("Reset slot: {:?}", entity);
        }
    }
}

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
        //text label/tile for the builder UI
        parent.spawn((
            Text::new("Construct a B.O.B."),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
        ));
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
            Interaction::None, // Make clickable
            HeadSlot, // Add identifier
            SlotFilled(false), // Track if filled
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
                Interaction::None,
                LeftArmSlot,
                SlotFilled(false),
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
                Interaction::None,
                BodySlot,
                SlotFilled(false),
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
                Interaction::None,
                RightArmSlot,
                SlotFilled(false),
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
                Interaction::None,
                LeftLegSlot,
                SlotFilled(false),
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
                Interaction::None,
                RightLegSlot,
                SlotFilled(false),
            ));
        });
    });
}


pub fn build_bob_ui_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut SlotFilled), 
        (Changed<Interaction>, Or<(With<HeadSlot>, With<BodySlot>, With<LeftArmSlot>, With<RightArmSlot>, With<LeftLegSlot>, With<RightLegSlot>)>)
    >,
    mut inventory_query: Query<&mut ComponentsInventory>,
) {
    for (interaction, mut bg_color, mut slot_filled) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if let Ok(mut inventory) = inventory_query.single_mut() {
                // Check if slot is already filled
                if slot_filled.0 {
                    println!("Slot already filled!");
                    continue;
                }

                // Check if we have components in inventory
                if inventory.count > 0 {
                    // Fill the slot (turn green)
                    *bg_color = BackgroundColor(GREEN_COLOR);
                    slot_filled.0 = true;
                    
                    // Decrease inventory (you might want to be more specific about which type)
                    // For now, just decrease the first available component
                    if inventory.count > 0 {
                        inventory.count -= 1;
                        println!("Used 1 component. Remaining: {}", inventory.count);
                    }
                } else {
                    println!("No components available in inventory!");
                }
            } else {
                println!("No inventory found!");
            }
        }
    }
}