// File: Linux-Server/src/state/inventory.rs
// Title: Inventory and Item Bucket State
// Plain English: Models item definitions, item instances, and inventory containers.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemBucketType {
    KineticWeapons = 1,
    EnergyWeapons = 2,
    PowerWeapons = 3,
    Helmet = 4,
    Gauntlets = 5,
    ChestArmor = 6,
    LegArmor = 7,
    ClassArmor = 8,
    Ghost = 9,
    Ship = 10,
    Vehicle = 11,
    Emblem = 12,
    Consumables = 13,
    Modifications = 14,
    Vault = 15,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemInstance {
    pub item_instance_id: u64,
    pub item_hash: u32,
    pub quantity: u32,
    pub power_level: u32,
    pub is_equipped: bool,
    pub is_locked: bool,
    pub bucket: ItemBucketType,
}

impl ItemInstance {
    pub fn new(
        item_instance_id: u64,
        item_hash: u32,
        quantity: u32,
        power_level: u32,
        bucket: ItemBucketType,
    ) -> Self {
        Self {
            item_instance_id,
            item_hash,
            quantity,
            power_level,
            is_equipped: false,
            is_locked: false,
            bucket,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CharacterInventory {
    pub items: Vec<ItemInstance>,
}

impl CharacterInventory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: ItemInstance) {
        self.items.push(item);
    }

    pub fn get_equipped_item(&self, bucket: ItemBucketType) -> Option<&ItemInstance> {
        self.items.iter().find(|i| i.bucket == bucket && i.is_equipped)
    }

    pub fn equip_item(&mut self, instance_id: u64) -> bool {
        let target_bucket = match self.items.iter().find(|i| i.item_instance_id == instance_id) {
            Some(i) => i.bucket,
            None => return false,
        };

        for item in self.items.iter_mut() {
            if item.bucket == target_bucket {
                item.is_equipped = item.item_instance_id == instance_id;
            }
        }
        true
    }

    pub fn set_locked(&mut self, instance_id: u64, locked: bool) -> bool {
        if let Some(item) = self.items.iter_mut().find(|i| i.item_instance_id == instance_id) {
            item.is_locked = locked;
            true
        } else {
            false
        }
    }
}
