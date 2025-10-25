use bevy::{input_focus::InputFocus, prelude::*};
//use bevy::picking::pointer::PointerInteraction; Useful for selectable meshes


const NORMAL_ATTACK: Color = Color::srgb(1.0,0.0, 0.0);
const NORMAL_BUILD: Color = Color::srgb(0.9,0.3, 0.0);
const NORMAL_SCOUT: Color = Color::srgb(0.0, 0.0, 1.0);
const HOVER_COLOR: Color =  Color::WHITE;
const HEAD_COLOR: Color = Color::srgb(0.0, 0.0, 1.0); //Removed later when not just squares

#[derive(Resource)]
struct GridState {
    next_position: usize,
} 

impl Default for GridState {
    fn default() -> Self {
        Self { next_position: 0}
    }
}

#[derive(Event)]
struct BuildBobEvent;

#[derive(Event)]
struct ScoutEvent;

#[derive(Event)]
struct AttackEvent;

#[derive(Component)]
struct Head;

#[derive(Component)]
struct HeadInventory {
    count: u32,
}

#[derive(Component)]
struct Selected;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health {
    current: f32,
    max: f32,
}

impl Health {
    fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
        }
    }

    fn take_damage(&mut self, damage: f32) {
        self.current = (self.current - damage).max(0.0);
    }

    fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

}

#[derive(Component)]
enum MenuButton {
    Attack,
    Build,
    Scout,
}

#[derive(Component)]
struct OriginalColor(Color);


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .init_resource::<InputFocus>()
        .init_resource::<GridState>()
        .add_observer(on_build_bob)
        .add_observer(on_attack)
        .add_systems(Startup, (setup, test_data))
        .add_systems(Update, button_system)
        .run();
}


//TODO, DELETE THIS LATER, ONLY FOR TESTING
fn test_data(mut commands:Commands) {
    commands.spawn(HeadInventory { count: 10 });
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d); //camera setup

    // Spawn an enemy entity
    commands.spawn((
        Enemy,
        Health::new(100.0),
        Name::new("Enemy"),
    ));

    //Buttons setup
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
    
    spawn_sprite(&mut root, 100.0, 100.0, Color::srgb(0.8, 0.2, 0.2));
    
    spawn_button(&mut root, MenuButton::Attack, "Attack!", NORMAL_ATTACK);
    spawn_button(&mut root, MenuButton::Build,  "Build!",  NORMAL_BUILD);
    spawn_button(&mut root, MenuButton::Scout,  "Scout!",  NORMAL_SCOUT);
    
    //Should implement a bob spawning grid here (To the side of the build button
}

fn spawn_sprite(
    parent: &mut EntityCommands,
    width: f32,
    height: f32,
    colour: Color,
) {
    parent.with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Px(width),
                height: Val::Px(height),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colour),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Enemy"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    });
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
            OriginalColor(colour),
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
    mut interaction_query: Query<(&Interaction, &OriginalColor, &MenuButton, &mut BackgroundColor, &mut BorderColor ),
    (Changed<Interaction>, With<Button>)>,
    mut commands: Commands,
) {
    for (interaction, original_color, button_type, mut bg_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let click_color = Color::srgb(
                    original_color.0.to_srgba().red * 0.7,
                    original_color.0.to_srgba().green * 0.7,
                    original_color.0.to_srgba().blue * 0.7,
                );
                *bg_color = click_color.into();

                match button_type {
                    MenuButton::Attack => {commands.trigger(AttackEvent); println!("Clicked on Attack");},
                    MenuButton::Build => {commands.trigger(BuildBobEvent); println!("Clicked on build");},
                    MenuButton::Scout => {commands.trigger(ScoutEvent); println!("Clicked on Scout");},
                }
            },

            Interaction::Hovered => {
                *border_color = HOVER_COLOR.into();
                *bg_color = original_color.0.into();
            },

            Interaction::None => {
                // Reset everything
                *bg_color = original_color.0.into();
                *border_color = original_color.0.into();
            },
        }
    }
}

fn on_attack(
    _trigger: On<AttackEvent>,
    mut enemy_query: Query<&mut Health, With<Enemy>>,
) {
    if let Ok(mut health) = enemy_query.single_mut() {
        health.take_damage(10.0);
        println!("Enemy took 10 damage! Current health: {}/{}", health.current, health.max);
        
        if health.is_dead() {
            println!("Enemy defeated!");
        }
    }
}

fn on_scout(
    _trigger: On<ScoutEvent>,
    mut commands: Commands,
) {
    todo!()
}

// New system to handle bob building
fn on_build_bob (
    _trigger: On<BuildBobEvent>,
    mut commands: Commands,
    mut query: Query<&mut HeadInventory>, //can be changed to inventory later?
    mut grid_state: ResMut<GridState>
) {
    if let Ok(mut inventory) = query.single_mut() {
        println!("Found {} heads in inventory", inventory.count);
        
        if inventory.count > 0 {
            // Deduct one head from inventory
            inventory.count -= 1;
            println!("Used 1 head, {} remaining", inventory.count);
            
            let grid_pos = calculate_grid_position(grid_state.next_position);
            grid_state.next_position += 1;

            commands.spawn((
                Head,
                Health::new(50.0),
                Sprite {
                    color: HEAD_COLOR,
                    custom_size: Some(Vec2::new(25.0, 25.0)),
                    ..default()
                },
                Transform::from_xyz(grid_pos.x, grid_pos.y, 2.),
                Name::new("Head"),
            ));//.observe(update_selected_on);

        } else {
            println!("No heads available in inventory!");
        }
    } else {
        println!("No inventory found!");
    }
}

/*fn update_selected_on(trigger: On<Pointer<Press>>, mut commands: Commands, mut query: Query<Entity, (With<Head>, With<Selected>)>)
{
    let clicked_entity =  query.get_mut(trigger.event_target()).unwrap(); //Should maybe not use unwrap here?
    println!("Head clicked: {:?}", clicked_entity);

    // Remove Selected from all other heads
    for selected_entity in query.iter() {
        if selected_entity != clicked_entity {
            commands.entity(selected_entity).remove::<Selected>();
            println!("Removed selection from entity: {:?}", selected_entity);
        }
    }

    // Add Selected to clicked entity
    commands.entity(clicked_entity).insert(Selected);
    println!("Selected head entity: {:?}", clicked_entity);

}*/


fn calculate_grid_position(position: usize) -> Vec2 {
    const GRID_COLS: usize = 3;
    const SLOT_SIZE: f32 = 25.0; //one bob should fit in here
    const PADDING: f32 = 10.0; //padding between bobs
    const TOTAL_SPACING: f32 = SLOT_SIZE + PADDING; 

    const GRID_START_X: f32 = - 250.0; // start grid a bit to the left, with 3 items per row one row width should be 180 px and build button is 65 px wide =~ 250
    const GRID_START_Y: f32 = -75.0;

    let col = position % GRID_COLS;
    let row = position / GRID_COLS;

    let x = GRID_START_X + (col as f32 * TOTAL_SPACING);
    let y = GRID_START_Y + (row as f32 * TOTAL_SPACING);

    Vec2::new(x,y)
}