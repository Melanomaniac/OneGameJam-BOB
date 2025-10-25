use bevy::{input_focus::InputFocus, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
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
struct StartScoutingEvent;

#[derive(Event)]
struct StartAttackingEvent;

#[derive(Event)]
struct TargetReachedEvent {
    entity: Entity,
}

#[derive(Component)]
struct Head;

#[derive(Component)]
struct HeadInventory {
    count: u32,
}

//#[derive(Component)]
//struct Selected;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Bob{
    state: BobState,
}

enum BobState {
    Attacking,
    Idling,
    Scouting,
}


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

#[derive(Component)]
struct Movement {
    speed: f32,
    target: Vec2,
}

#[derive(Component)]
struct Scout;

#[derive(Component)]
struct Attack;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_plugins(EguiPlugin::default())
        .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .init_resource::<InputFocus>()
        .init_resource::<GridState>()
        .add_observer(on_build_bob)
        .add_observer(on_attack)
        .add_observer(on_scout)
        .add_observer(on_target_reached)  // Add this line!
        .add_systems(Startup, (setup, test_data))
        .add_systems(Update, (button_system, bob_system, movement_system, scouting_system))
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
        Sprite{ color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(100.0,100.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 270.0, 1.0),
        Health::new(100.0),
        Name::new("Enemy"),
    ));

    //Buttons setup
    let mut root = commands.spawn(
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(50.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            ..default()
        }
    );
    
    // spawn_sprite(&mut root, 100.0, 100.0, Color::srgb(0.8, 0.2, 0.2));
    
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
                    MenuButton::Attack => {commands.trigger(StartAttackingEvent); println!("Clicked on Attack");},
                    MenuButton::Build => {commands.trigger(BuildBobEvent); println!("Clicked on build");},
                    MenuButton::Scout => {commands.trigger(StartScoutingEvent); println!("Clicked on Scout");},
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

fn movement_system(
    mut query: Query<(Entity, &mut Transform, &Movement)>,
    mut commands: Commands,
    time: Res<Time>, 
) {    
    for (entity, mut transform, movement) in query.iter_mut() {

        let movement_speed = movement.speed; // pixels per second
        let target = movement.target;

        let current_position = transform.translation.xy();
        let distance = target.distance(current_position);
        if distance > 5.0 {
            let direction = (target-current_position).normalize();
            transform.translation.x += direction.x * movement_speed * time.delta_secs();
            transform.translation.y += direction.y * movement_speed * time.delta_secs();
        }
    }
}

fn scouting_system(
    mut query: Query<(Entity, &Bob), With<Head>>,
    mut commands: Commands,
) {
    for (entity, bob) in query.iter_mut() {
        if matches!(bob.state, BobState::Scouting) {
            // Additional scouting logic can go here
            // For now, when a bob finishes scouting (Movement component removed), despawn it
            // This will be triggered after movement_system removes the Movement component
        }
    }
}

fn bob_system(
    mut query: Query<(Entity, &Bob, &Transform, &mut Movement, Option<&Scout>, Option<&Attack>), With<Head>>,
    mut commands: Commands,
) {

    let attack_target = Vec2::new(0.0, 200.0); // Position of the enemy for attacking
    let scout_target = Vec2::new(0.0, -400.0); // Position for scouting

    for (entity, bob,  transform, mut movement, maybe_scout, maybe_attack) in query.iter_mut() {
        match bob.state {
            BobState::Attacking => {
                // Attack logic here - check if it has Movement component for attack behavior
                let distance_to_target = transform.translation.xy().distance(attack_target);
                if(distance_to_target > 5.0){
                     movement.target = attack_target;
                     movement.speed = 100.0;
                }
            },
            BobState::Idling => {
                // Idle logic here - make sure no Movement component
                return;
            },
            BobState::Scouting => {                
                let distance_to_target = transform.translation.xy().distance(scout_target);
                if  distance_to_target > 5.0 {
                     movement.target = scout_target;
                     movement.speed = 100.0;
                }
            }
        }
    }
}


fn on_attack(
    _trigger: On<StartAttackingEvent>,
    mut query: Query<(Entity, &mut Bob)>,
    mut grid_state: ResMut<GridState>,
) {
    // Find the first idle bob and change its state directly
    if let Some((entity, mut bob)) = query.iter_mut().find(|(_, bob)| matches!(bob.state, BobState::Idling)) {
        bob.state = BobState::Attacking;
        grid_state.next_position -= 1; // since one bob has left the grid, we decrease next position in grid
        println!("Sent head {:?} on attacking mission!", entity);
    } else {
        println!("There are no idle bobs available!");
    }
}

fn on_scout(
    _trigger: On<StartScoutingEvent>,
    mut query: Query<(Entity, &mut Bob)>,
    mut grid_state: ResMut<GridState>,
) {
    // Find the first idle bob and change its state directly
    if let Some((entity, mut bob)) = query.iter_mut().find(|(_, bob)| matches!(bob.state, BobState::Idling)) {
        bob.state = BobState::Scouting;
        grid_state.next_position -= 1; // since one bob has left the grid, we decrease next position in grid
        println!("Sent head {:?} on scouting mission!", entity);
    } else { 
        println!("There are no idle bobs available!"); 
    }
}

fn on_target_reached(
    trigger: On<TargetReachedEvent>,
    query: Query<&Bob>,
    mut commands: Commands,
) {
    let entity = trigger.event().entity;
    
    if let Ok(bob) = query.get(entity) {
        if matches!(bob.state, BobState::Scouting) {
            println!("Scouting bob {:?} has reached its target and will be despawned.", entity);
            commands.entity(entity).insert(Scout);
        } else if matches!(bob.state, BobState::Attacking) {
            println!("Attacking bob {:?} has reached its target and will start attacking.", entity);
            commands.entity(entity).insert(Attack);
        }
    }
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
                Bob { state: BobState::Idling },
                Head,
                Health::new(50.0),
                Sprite {
                    color: HEAD_COLOR,
                    custom_size: Some(Vec2::new(25.0, 25.0)),
                    ..default()
                },
                Transform::from_xyz(grid_pos.x, grid_pos.y, 2.),
                Name::new("Bob"),
                Movement{
                    speed: 0.0,
                    target: Vec2::ZERO,
                },
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