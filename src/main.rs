use bevy::{ecs::component, input_focus::InputFocus, log::tracing_subscriber::fmt::time, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
//use bevy::picking::pointer::PointerInteraction; Useful for selectable meshes
use std::process;

mod ui;
use ui::*;
mod loot;
use loot::*;

const NORMAL_ATTACK: Color = Color::srgb(1.0,0.0, 0.0);
const NORMAL_BUILD: Color = Color::srgb(0.9,0.3, 0.0);
const NORMAL_SCOUT: Color = Color::srgb(0.0, 0.0, 1.0);
const HOVER_COLOR: Color =  Color::WHITE;
const HEAD_COLOR: Color = Color::srgb(0.0, 0.0, 1.0); //Removed later when not just squares

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameStates {
    #[default]
    Playing,
    Win,
    Loss,
}


#[derive(Resource)]
struct GridState {
    occupied_positions: Vec<bool>,  // Track which positions are occupied
} 

impl Default for GridState {
    fn default() -> Self {
        Self { 
            occupied_positions: vec![false; 50]  // Support up to 50 grid positions
        }
    }
}

impl GridState {
    // Find the first available (unoccupied) grid position
    fn find_first_available(&self) -> Option<usize> {
        self.occupied_positions.iter().position(|&occupied| !occupied)
    }
    
    // Mark a position as occupied
    fn occupy(&mut self, position: usize) {
        if position < self.occupied_positions.len() {
            self.occupied_positions[position] = true;
        }
    }
    
    // Mark a position as free
    fn free(&mut self, position: usize) {
        if position < self.occupied_positions.len() {
            self.occupied_positions[position] = false;
        }
    }
}

#[derive(Event)]
struct BuildBobEvent;

#[derive(Event)]
struct StartScoutingEvent;

#[derive(Event)]
struct StartAttackingEvent;

#[derive(Component)]
struct Head;

#[derive(Component)]
struct Body;

#[derive(Component)]
struct Legs;

#[derive(Component)]
struct Arms;


#[derive(Component)]
struct ComponentsInventory {
    count: u32,
}

//#[derive(Component)]
//struct Selected;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct HomeBase;

#[derive(Component)]
struct Bob{
    state: BobState,
    grid_position: usize,  // Each Bob remembers its own grid slot
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
struct Attack{
    target_entity: Entity,  // Reference to the enemy entity
    damage: f32,
    max_cooldown: f32,
    current_cooldown: f32,
}

#[derive(Component)]
struct Size(Vec2);

impl Size {
    fn new(width: f32, height: f32) -> Self {
        Self(Vec2::new(width, height))
    }
    
    fn square(size: f32) -> Self {
        Self(Vec2::new(size, size))
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_plugins(EguiPlugin::default())

        .init_state::<GameStates>()
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .init_resource::<InputFocus>()
        .init_resource::<GridState>()
        .add_observer(on_build_bob)
        .add_observer(on_reset_ui)
        .add_observer(on_attack)
        .add_observer(on_scout)
        .add_systems(OnEnter(GameStates::Win), win_screen)
        .add_systems(OnEnter(GameStates::Loss), loss_screen)
        .add_systems(OnExit(GameStates::Win), cleanup_win_screen)
        .add_systems(OnExit(GameStates::Loss), cleanup_loss_screen)
        .add_systems(Startup, (setup, test_data, setup_build_bob_ui))
        .add_systems(Update, (
            button_system, 
            bob_system, 
            build_bob_ui_system,
            movement_system, 
            scouting_system, 
            attacking_system,
            enemy_system,
            play_again_button_system,
        ))
        .run();
}


//TODO, DELETE THIS LATER, ONLY FOR TESTING
fn test_data(mut commands:Commands) {
    commands.spawn(ComponentsInventory { count: 10 });
}

fn setup(mut commands: Commands, q_window: Query<&Window>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d); //camera setup

    // Spawn an enemy entity
    let enemy_size = Size::new(200.0, 200.0);
    let enemy_size_vec = enemy_size.0;  // Extract Vec2 before moving

    let homeBase_size = Size::new(300.0, 300.0);
    let homeBase_size_vec = homeBase_size.0;
    commands.spawn((
        HomeBase,
        homeBase_size,
        Sprite{
            image: asset_server.load("sprites/HomeBase.png"),
            custom_size: Some(homeBase_size_vec),
            ..default()
        },
        Transform::from_xyz(0.0, 50.0, 1.0),
        Name::new("Home Base"),
        Health::new(500.0),
    ));

        commands.spawn((
        Enemy,
        enemy_size,
        Sprite{ 
            image: asset_server.load("sprites/Enemy.png"),
            custom_size: Some(enemy_size_vec),
            ..default()
        },
        Transform::from_xyz(0.0, 320.0, 1.0),
        Health::new(1000.0),
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
    
    // Spawn fullscreen background sprite
    if let Ok(window) = q_window.single() {
        let sprite_size = Vec2::new(window.width(), window.height());
        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/Background.png"),
                custom_size: Some(sprite_size),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0), // Centered, behind everything (z=0)
            Name::new("Background"),
        ));
    }
}


fn spawn_background_sprite() {

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
    time: Res<Time>, 
) {    
    for (_entity, mut transform, movement) in query.iter_mut() {

        let movement_speed = movement.speed; // pixels per second
        let target = movement.target;

        let current_position = transform.translation.xy();
        let distance = target.distance(current_position);
        if distance >= 2.0 {
            let direction = (target-current_position).normalize();
            transform.translation.x += direction.x * movement_speed * time.delta_secs();
            transform.translation.y += direction.y * movement_speed * time.delta_secs();
        }
    }
}

fn scouting_system(
    mut query: Query<(Entity, &mut Bob, Option<&Scout>), With<Head>>,
    mut inventory_query: Query<&mut ComponentsInventory>,
    mut commands: Commands,
    mut grid_state: ResMut<GridState>,
) {
    for (entity, mut bob, maybe_scout) in query.iter_mut() {
        if matches!(bob.state, BobState::Scouting) {
            // When scout component is added, the bob has reached the target
            if maybe_scout.is_some() {
                // Generate random loot!
                let loot = generate_random_loot();
                println!("Scout {:?} found loot: {:?} x{}", entity, loot.loot_type, loot.quantity);
                
                // TODO: Add loot to inventory
                // For now, just attach it as a component to the bob
                commands.entity(entity).insert(loot);
                // add to the component inventory
                let mut inventory = inventory_query.single_mut().unwrap();
                inventory.count += 3;

                commands.entity(entity).remove::<Scout>();
                
                // Find first available grid position and assign it
                if let Some(new_grid_pos) = grid_state.find_first_available() {
                    bob.grid_position = new_grid_pos;
                    grid_state.occupy(new_grid_pos);
                    bob.state = BobState::Idling;
                } else {
                    println!("Warning: No available grid position for returning scout!");
                }
            }
        }
    }
}

fn attacking_system(
    mut attacker_query: Query<(Entity, Option<&mut Bob>, &mut Attack)>,
    mut health_query: Query<&mut Health>,
    time: Res<Time>,
    mut commands: Commands,
    mut grid_state: ResMut<GridState>,
    mut next_state: ResMut<NextState<GameStates>>, // Add this parameter
) {
    // Handle all attacks (both Bobs and Enemies)
    for (entity, maybe_bob, mut attack) in attacker_query.iter_mut() {
        if attack.current_cooldown <= 0.0 {
            // Try to get the target's health and apply damage
            if let Ok(mut health) = health_query.get_mut(attack.target_entity) {
                health.take_damage(attack.damage);
                
                if let Some(_) = maybe_bob {
                    println!("Bob {:?} dealt {} damage! Target health: {}/{}", entity, attack.damage, health.current, health.max);
                } else {
                    println!("Enemy {:?} dealt {} damage! Target health: {}/{}", entity, attack.damage, health.current, health.max);
                }
                
                if health.is_dead() {
                    if maybe_bob.is_some() {
                        println!("Target defeated by Bob!");
                        next_state.set(GameStates::Win);
                    } else {
                        println!("Home Base destroyed!");
                        next_state.set(GameStates::Loss);
                    }
                    //commands.entity(attack.target_entity).despawn();
                }
            } else {
                // Target no longer exists, remove Attack component
                commands.entity(entity).remove::<Attack>();
                
                // If this is a Bob, return it to grid
                if let Some(mut bob) = maybe_bob {
                    if let Some(new_grid_pos) = grid_state.find_first_available() {
                        bob.grid_position = new_grid_pos;
                        grid_state.occupy(new_grid_pos);
                        bob.state = BobState::Idling;
                    } else {
                        println!("Warning: No available grid position for returning attacker!");
                    }
                }
                continue;
            }
            attack.current_cooldown = attack.max_cooldown;
        } else {
            attack.current_cooldown -= time.delta_secs();
        }
    }
}

fn bob_system(
    mut query: Query<(Entity, &Bob, &Transform, Option<&mut Movement>, Option<&Scout>, Option<&Attack>), With<Head>>,
    enemy_query: Query<(Entity, &Transform, &Size, &Enemy, &Health)>,
    mut commands: Commands,
) {

    // Position for scouting
    let scout_target = Vec2::new(0.0, -400.0); 

    for (entity, bob,  transform, maybe_movement, maybe_scout, maybe_attack) in query.iter_mut() {
        match bob.state {
            BobState::Attacking => {
                // Attack logic here - check if it has Movement component for attack behavior

                // query for any entity with enemy component
                let enemy_data = if let Some(data) = enemy_query.iter().next() {
                    data
                } else {
                    // No enemy found, exit early without doing anything
                    return;
                };
                
                let (enemy_entity, enemy_transform, enemy_size, _enemy, enemy_health) = enemy_data;
                
                // Calculate attack position just below the enemy sprite
                let enemy_pos = enemy_transform.translation.xy();
                let attack_target_pos = Vec2::new(
                    enemy_pos.x,
                    enemy_pos.y - (enemy_size.0.y / 2.0) // Just below the bottom edge
                );

                // is it in distance of attack target?
                let distance_to_target = transform.translation.xy().distance(attack_target_pos);
                if distance_to_target > 2.0 {
                    // not in range yet, make it move towards the target
                    if let Some(mut movement) = maybe_movement {
                        // Update existing movement target
                        movement.target = attack_target_pos;
                    } else {
                        // No movement component yet, insert one
                        commands.entity(entity).insert(Movement {
                            speed: 100.0,
                            target: attack_target_pos,
                        });
                    }
                } else {
                    if enemy_health.is_dead() == false {
                        if maybe_attack.is_none() {
                            commands.entity(entity).insert(Attack {
                                target_entity: enemy_entity,
                                damage: 10.0,
                                max_cooldown: 1.0,
                                current_cooldown: 0.0,  // Start at 0 to attack immediately
                            });
                        }
                    }
                }                
            },
            BobState::Idling => {
                let grid_pos = calculate_grid_position(bob.grid_position);
                let distance_to_target = transform.translation.xy().distance(grid_pos);
                if distance_to_target > 2.0 {
                    // not in range yet, make it move towards the target
                    if let Some(mut movement) = maybe_movement {
                        // Update existing movement target
                        movement.target = grid_pos;
                    } else {
                        // No movement component yet, insert one
                        commands.entity(entity).insert(Movement {
                            speed: 100.0,
                            target: grid_pos,
                        });
                    }
                } else {
                    // in range, remove Movement component to stop moving 
                    if maybe_movement.is_some() {
                        commands.entity(entity).remove::<Movement>();
                    } 
                }
            },
            BobState::Scouting => {                

                // is it in distance of scouting target?
                let distance_to_target = transform.translation.xy().distance(scout_target);
                if distance_to_target > 2.0 {
                    // not in range yet, make it move towards the target
                    if let Some(mut movement) = maybe_movement {
                        // Update existing movement target
                        movement.target = scout_target;
                    } else {
                        // No movement component yet, insert one
                        commands.entity(entity).insert(Movement {
                            speed: 100.0,
                            target: scout_target,
                        });
                    }
                } else {
                    // in range, remove Movement component to stop moving 
                    if maybe_movement.is_some() {
                        commands.entity(entity).remove::<Movement>();
                    } 

                    if maybe_scout.is_none() {
                        commands.entity(entity).insert(Scout);
                    }
                }
            }
        }
    }
}


fn enemy_system(
    mut enemy_query: Query<(Entity, &Transform, Option<&mut Movement>, Option<&Attack>), With<Enemy>>,
    home_base_query: Query<(Entity, &Transform, &Size), With<HomeBase>>,
    mut commands: Commands,
) {
    // Get home base data
    let home_base_data = if let Some(data) = home_base_query.iter().next() {
        data
    } else {
        // No home base found, exit early
        return;
    };
    
    let (home_base_entity, home_base_transform, home_base_size) = home_base_data;
    
    // Calculate attack position (above the home base)
    let home_base_pos = home_base_transform.translation.xy();
    let attack_target_pos = Vec2::new(
        home_base_pos.x,
        home_base_pos.y + (home_base_size.0.y / 2.0) // Just above the top edge
    );
    
    for (entity, transform, maybe_movement, maybe_attack) in enemy_query.iter_mut() {
        let distance_to_target = transform.translation.xy().distance(attack_target_pos);            
        
        if distance_to_target > 2.0 {
            // Not in range yet, make it move towards the target
            if let Some(mut movement) = maybe_movement {
                // Update existing movement target
                movement.target = attack_target_pos;
            } else {
                // No movement component yet, insert one
                println!("Inserting Movement component for enemy {:?}", entity);
                commands.entity(entity).insert(Movement {
                    speed: 10.0,
                    target: attack_target_pos,
                });
            }
        } else {
            // In range, remove Movement component to stop moving
            if maybe_movement.is_some() {
                commands.entity(entity).remove::<Movement>();
            }
            
            // Start attacking if not already attacking
            if maybe_attack.is_none() {
                println!("Enemy {:?} reached home base!", entity);
                commands.entity(entity).insert(Attack {
                    target_entity: home_base_entity,
                    damage: 50.0,
                    max_cooldown: 5.0,
                    current_cooldown: 0.0,  // Start at 0 to attack immediately
                });
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
        grid_state.free(bob.grid_position);  // Free up this Bob's grid position
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
        grid_state.free(bob.grid_position);  // Free up this Bob's grid position
        println!("Sent head {:?} on scouting mission!", entity);
    } else { 
        println!("There are no idle bobs available!"); 
    }
}

// New system to handle bob building
fn on_build_bob (
    _trigger: On<BuildBobEvent>,
    mut commands: Commands,
    mut query: Query<&mut ComponentsInventory>, //can be changed to inventory later?
    mut grid_state: ResMut<GridState>,
    slot_query: Query<&SlotFilled, Or<(With<HeadSlot>, With<BodySlot>, With<LeftArmSlot>, With<RightArmSlot>, With<LeftLegSlot>, With<RightLegSlot>)>>,
    asset_server: Res<AssetServer>
) {
    if let Ok(mut inventory) = query.single_mut() {
        println!("Found {} heads in inventory", inventory.count);

         // Check if all 6 slots are filled
        let filled_slots = slot_query.iter().filter(|slot| slot.0).count();
        let total_slots = slot_query.iter().count();
        
        println!("Filled slots: {}/{}", filled_slots, total_slots);
        
        if filled_slots < total_slots {
            println!("Cannot build Bob! Not all body parts are placed. Need {}/{} slots filled.", filled_slots, total_slots);
            return; // Exit early if not all slots are filled
        }
        
        if filled_slots == total_slots {
            // Find the first available grid position
            if let Some(grid_position) = grid_state.find_first_available() {
                // Deduct one head from inventory
                // inventory.count -= 1;
                println!("built 1 bob, {} remaining parts:", inventory.count);
                
                let grid_pos = calculate_grid_position(grid_position);
                grid_state.occupy(grid_position);  // Mark this position as occupied

                let bob_size = Size::square(100.0);
                let bob_size_vec = bob_size.0;  // Extract Vec2 before moving
                commands.spawn((
                    Bob { 
                        state: BobState::Idling,
                        grid_position,
                    },
                    Head,
                    Health::new(50.0),
                    bob_size,
                    Sprite {
                        image: asset_server.load("sprites/BoB.png"),
                        custom_size: Some(bob_size_vec),
                        ..default()
                    },
                    Transform::from_xyz(grid_pos.x, grid_pos.y, 2.),
                    Name::new("Bob"),
                ));//.observe(update_selected_on);
                commands.trigger(ResetBuilderUIEvent); //reset builder
            } else {
                println!("Grid is full! Cannot spawn more Bobs.");
            }
        } else {
            println!("No robot components available in inventory!");
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

fn restart_game() {
    println!("Restarting application...");
    // Get the current executable path and restart
    let current_exe = std::env::current_exe().unwrap();
    process::Command::new(current_exe).spawn().unwrap();
    process::exit(0);
}