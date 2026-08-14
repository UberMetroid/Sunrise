// File: Thanatonaut/src/manifest/item_definition.rs
// Title: Destiny 2 Item & Asset Manifest Definition
// Plain English: Models items, stats, damage archetypes, and flavor text extracted from Bungie manifests.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemStatDefinition {
    pub stat_hash: u32,
    pub stat_name: String,
    pub value: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemDefinition {
    pub item_hash: u32,
    pub name: String,
    pub description: String,
    pub item_type: String,
    pub tier_type: String,
    pub icon_path: String,
    pub is_exotic: bool,
    pub stats: HashMap<u32, ItemStatDefinition>,
}

impl DestinyItemDefinition {
    pub fn new(
        item_hash: u32,
        name: impl Into<String>,
        description: impl Into<String>,
        item_type: impl Into<String>,
        tier_type: impl Into<String>,
    ) -> Self {
        let tier = tier_type.into();
        let is_exotic = tier.eq_ignore_ascii_case("Exotic");
        Self {
            item_hash,
            name: name.into(),
            description: description.into(),
            item_type: item_type.into(),
            tier_type: tier,
            icon_path: String::new(),
            is_exotic,
            stats: HashMap::new(),
        }
    }

    pub fn add_stat(&mut self, stat_hash: u32, stat_name: impl Into<String>, value: i32) {
        self.stats.insert(
            stat_hash,
            ItemStatDefinition {
                stat_hash,
                stat_name: stat_name.into(),
                value,
            },
        );
    }
}
