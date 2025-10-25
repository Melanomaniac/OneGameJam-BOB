use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Debug, Clone)]
pub struct Loot {
    pub loot_type: LootType,
    pub quantity: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LootType {
    Head,
    Arms,
    Body,
    Legs,
}

impl Loot {
    pub fn new(loot_type: LootType, quantity: u32) -> Self {
        Self {
            loot_type,
            quantity,
        }
    }
}

/// Generates random loot with weighted probabilities
pub fn generate_random_loot() -> Loot {
    let mut rng = rand::thread_rng();
    
    // Random roll to determine loot type (0-100)
    let roll = rng.gen_range(0..100);
    
    let loot_type = match roll {
        0..=24 => LootType::Head,   // 25% chance
        25..=49 => LootType::Arms,  // 25% chance
        50..=74 => LootType::Body,  // 25% chance
        75..=99 => LootType::Legs,  // 25% chance
        _ => LootType::Head,        // Fallback
    };
    
    // All body parts have quantity of 1
    let quantity = 1;
    
    Loot::new(loot_type, quantity)
}

/// Generates multiple random loot items
pub fn generate_loot_batch(count: u32) -> Vec<Loot> {
    (0..count)
        .map(|_| generate_random_loot())
        .collect()
}

/// Generates loot with a specific type and random quantity
pub fn generate_loot_of_type(loot_type: LootType) -> Loot {
    // All body parts have quantity of 1
    let quantity = 1;
    
    Loot::new(loot_type, quantity)
}
