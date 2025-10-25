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

//#[derive(Component)]
//struct Selected;

#[derive(Component)]
enum MenuButton {
    Attack,
    Build,
    Scout,
}

#[derive(Component)]
enum BobState {
    Attacking,
    Idling,
    Scouting,
}

#[derive(Component)]
struct OriginalColor(Color);


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .init_resource::<InputFocus>()
        .init_resource::<GridState>()
        .add_observer(on_build_bob)
        .add_observer(on_scout)
        .add_systems(Startup, (setup, test_data))
        .add_systems(Update, (button_system, scouting_system))
        .run();
}


//TODO, DELETE THIS LATER, ONLY FOR TESTING
fn test_data(mut commands:Commands) {
    commands.spawn(HeadInventory { count: 10 });
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d); //camera setup

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

    spawn_button(&mut root, MenuButton::Attack, "Attack!", NORMAL_ATTACK);
    spawn_button(&mut root, MenuButton::Build,  "Build!",  NORMAL_BUILD);
    spawn_button(&mut root, MenuButton::Scout,  "Scout!",  NORMAL_SCOUT);
    
    //Should implement a bob spawning grid here (To the side of the build button
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

fn scouting_system(
    mut query: Query<(Entity, &mut Transform, &BobState), With<Head>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut transform, state) in query.iter_mut() {
        if matches!(state, BobState::Scouting) {
            // Move towards bottom of screen
            let movement_speed = 50.0; // pixels per second
            let target = Vec2::new(0.0, -400.0); //maybe choose dynamically where the end of the screen is instead of hard numbers

            let current_position = transform.translation.xy();
            let distance = target.distance(current_position);
            if distance > 5.0 {
                let direction = (target-current_position).normalize();
                transform.translation.x += direction.x * movement_speed * time.delta_secs();
                transform.translation.y += direction.y * movement_speed * time.delta_secs();
            }
            else {
                println!("Scout {:?} has reached the bottom! Removing from screen.", entity);
                commands.entity(entity).despawn(); //Should probably add some more logic beyond this to set them into a queue or something
                //so that they can return to idling after a certain amount of time, maybe a separate system?
            }
        }
    }
}

fn on_attack(
    _trigger: On<AttackEvent>,
    mut commands: Commands,
) {
    todo!()
}

fn on_scout(
    _trigger: On<ScoutEvent>,
    mut query: Query<(Entity, &mut BobState), With<Head>>,
    mut grid_state: ResMut<GridState>,
) {
    // Find the first idle bob and change its state directly
    if let Some((entity, mut state)) = query.iter_mut().find(|(_, state)| matches!(**state, BobState::Idling)) {
        *state = BobState::Scouting;
        grid_state.next_position -= 1; // since one bob has left the grid, we decrease next position in grid
        println!("Sent head {:?} on scouting mission!", entity);
    } else { 
        println!("There are no idle bobs available!"); 
    }
}

// New system to handle bob building
fn on_build_bob (
    _trigger: On<BuildBobEvent>,
    mut commands: Commands,
    mut query: Query<&mut HeadInventory>, //can be changed to inventory later?
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
                Mesh2d(meshes.add(Rectangle::new(25.0, 25.0))),
                MeshMaterial2d(materials.add(HEAD_COLOR)),
                Transform::from_xyz(grid_pos.x, grid_pos.y, 2.),
                Name::new("Head"),
                BobState::Idling,
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