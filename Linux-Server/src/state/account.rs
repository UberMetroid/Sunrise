// File: Linux-Server/src/state/account.rs
// Title: Player Account and Character Profiles
// Plain English: Models player membership, character slots (Titan, Hunter, Warlock), and stats.

use serde::{Deserialize, Serialize};
use crate::state::inventory::{CharacterInventory, ItemBucketType};
use crate::state::light_calculator::{calculate_base_light, GearSlots};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharacterClass {
    Titan = 0,
    Hunter = 1,
    Warlock = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterState {
    pub character_id: u64,
    pub class_type: CharacterClass,
    pub light_level: u32,
    pub inventory: CharacterInventory,
}

impl CharacterState {
    pub fn new(character_id: u64, class_type: CharacterClass) -> Self {
        Self {
            character_id,
            class_type,
            light_level: 0,
            inventory: CharacterInventory::new(),
        }
    }

    pub fn recalculate_light(&mut self) {
        let get_power = |b: ItemBucketType| -> u32 {
            self.inventory.get_equipped_item(b).map(|i| i.power_level).unwrap_or(0)
        };

        let slots = GearSlots::new(
            get_power(ItemBucketType::KineticWeapons),
            get_power(ItemBucketType::EnergyWeapons),
            get_power(ItemBucketType::PowerWeapons),
            get_power(ItemBucketType::Helmet),
            get_power(ItemBucketType::Gauntlets),
            get_power(ItemBucketType::ChestArmor),
            get_power(ItemBucketType::LegArmor),
            get_power(ItemBucketType::ClassArmor),
        );

        self.light_level = calculate_base_light(&slots);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountState {
    pub membership_id: u64,
    pub display_name: String,
    pub characters: Vec<CharacterState>,
}

impl AccountState {
    pub fn new(membership_id: u64, display_name: impl Into<String>) -> Self {
        Self {
            membership_id,
            display_name: display_name.into(),
            characters: Vec::new(),
        }
    }

    pub fn add_character(&mut self, character: CharacterState) {
        self.characters.push(character);
    }

    pub fn get_character(&self, character_id: u64) -> Option<&CharacterState> {
        self.characters.iter().find(|c| c.character_id == character_id)
    }

    pub fn get_character_mut(&mut self, character_id: u64) -> Option<&mut CharacterState> {
        self.characters.iter_mut().find(|c| c.character_id == character_id)
    }
}
