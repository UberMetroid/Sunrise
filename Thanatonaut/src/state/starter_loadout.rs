// File: Thanatonaut/src/state/starter_loadout.rs
// Title: Default Season of Arrivals Guardian Loadout Factory
// Plain English: Generates rich starter loadouts with classic and exotic weapons, armor, and sparrows.

use crate::state::account::{CharacterClass, CharacterState};
use crate::state::inventory::{ItemBucketType, ItemInstance};

pub struct StarterLoadoutFactory;

impl StarterLoadoutFactory {
    pub fn create_default_character(char_id: u64, class_type: CharacterClass) -> CharacterState {
        let mut character = CharacterState::new(char_id, class_type);

        // 1. Kinetic Weapons (hashes only - see Downloads vault)
        let mut witherhoard = ItemInstance::new(101, 2357508056, 1, 1050, ItemBucketType::KineticWeapons);
        witherhoard.is_equipped = true;
        character.inventory.add_item(witherhoard);
        character.inventory.add_item(ItemInstance::new(102, 3993415705, 1, 1050, ItemBucketType::KineticWeapons));
        character.inventory.add_item(ItemInstance::new(103, 347366834, 1, 1050, ItemBucketType::KineticWeapons));

        // 2. Energy Weapons (hashes only)
        let mut recluse = ItemInstance::new(201, 2354271539, 1, 1050, ItemBucketType::EnergyWeapons);
        recluse.is_equipped = true;
        character.inventory.add_item(recluse);
        character.inventory.add_item(ItemInstance::new(202, 2222560548, 1, 1050, ItemBucketType::EnergyWeapons));
        character.inventory.add_item(ItemInstance::new(203, 814876684, 1, 1050, ItemBucketType::EnergyWeapons));

        // 3. Power Weapons (hashes only)
        let mut sword = ItemInstance::new(301, 614426548, 1, 1050, ItemBucketType::PowerWeapons);
        sword.is_equipped = true;
        character.inventory.add_item(sword);
        character.inventory.add_item(ItemInstance::new(302, 1891561814, 1, 1050, ItemBucketType::PowerWeapons));
        character.inventory.add_item(ItemInstance::new(303, 2220014607, 1, 1050, ItemBucketType::PowerWeapons));

        // 4. Class Armor Sets (Helmet, Gauntlets, Chest, Legs, Class Item)
        let (helm_hash, arms_hash, chest_hash, legs_hash, class_hash) = match class_type {
            CharacterClass::Titan => (1357901, 1357902, 1357903, 1357904, 1357905),
            CharacterClass::Hunter => (2468001, 2468002, 2468003, 2468004, 2468005),
            CharacterClass::Warlock => (3579101, 3579102, 3579103, 3579104, 3579105),
        };

        let mut helm = ItemInstance::new(401, helm_hash, 1, 1050, ItemBucketType::Helmet);
        helm.is_equipped = true;
        character.inventory.add_item(helm);

        let mut arms = ItemInstance::new(501, arms_hash, 1, 1050, ItemBucketType::Gauntlets);
        arms.is_equipped = true;
        character.inventory.add_item(arms);

        let mut chest = ItemInstance::new(601, chest_hash, 1, 1050, ItemBucketType::ChestArmor);
        chest.is_equipped = true;
        character.inventory.add_item(chest);

        let mut legs = ItemInstance::new(701, legs_hash, 1, 1050, ItemBucketType::LegArmor);
        legs.is_equipped = true;
        character.inventory.add_item(legs);

        let mut class_item = ItemInstance::new(801, class_hash, 1, 1050, ItemBucketType::ClassArmor);
        class_item.is_equipped = true;
        character.inventory.add_item(class_item);

        // 5. Ghost Shell & Exotic Sparrow (Always on Time)
        let mut ghost = ItemInstance::new(901, 10000001, 1, 1050, ItemBucketType::Ghost);
        ghost.is_equipped = true;
        character.inventory.add_item(ghost);

        let mut sparrow = ItemInstance::new(902, 1903459814, 1, 1050, ItemBucketType::Vehicle);
        sparrow.is_equipped = true;
        character.inventory.add_item(sparrow);

        character.recalculate_light();
        character
    }
}
