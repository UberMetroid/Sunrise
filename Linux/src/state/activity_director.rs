// File: Linux/src/state/activity_director.rs
// Title: Destination & Vaulted Activity Director
// Plain English: Maps Destiny 2 destination hashes, raid activities, and spawn coordinates.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationManifest {
    pub name: String,
    pub destination_hash: u32,
    pub activity_hash: u32,
    pub recommended_light: u32,
    pub description: String,
}

pub struct ActivityDirector;

impl ActivityDirector {
    pub fn get_available_destinations() -> Vec<DestinationManifest> {
        vec![
            DestinationManifest {
                name: "Titan // New Pacific Arcology".to_string(),
                destination_hash: 373711905,
                activity_hash: 2166136261,
                recommended_light: 750,
                description: "Submerged Golden Age metropolis on the methane seas of Titan.".to_string(),
            },
            DestinationManifest {
                name: "Io // The Rupture & Lost Oasis".to_string(),
                destination_hash: 4136426117,
                activity_hash: 3373516314,
                recommended_light: 750,
                description: "The last site touched by the Traveler before the Collapse.".to_string(),
            },
            DestinationManifest {
                name: "Mars // Hellas Basin & Mindlab".to_string(),
                destination_hash: 2877881079,
                activity_hash: 3737671190,
                recommended_light: 750,
                description: "Frozen red dunes and the central server core of Warmind Rasputin.".to_string(),
            },
            DestinationManifest {
                name: "Mercury // The Lighthouse & Forest".to_string(),
                destination_hash: 3455118012,
                activity_hash: 4188371101,
                recommended_light: 750,
                description: "Vex simulation machine under the blazing solar storms.".to_string(),
            },
            DestinationManifest {
                name: "The Leviathan // Calus Palace & Raid".to_string(),
                destination_hash: 2693136600,
                activity_hash: 2693136601,
                recommended_light: 750,
                description: "Emperor Calus's flagship world-eater with the royal pleasure gardens.".to_string(),
            },
            DestinationManifest {
                name: "The Farm // European Dead Zone".to_string(),
                destination_hash: 2171538392,
                activity_hash: 2171538393,
                recommended_light: 750,
                description: "The peaceful refugee sanctuary from the Red War campaign.".to_string(),
            },
            DestinationManifest {
                name: "Tangled Shore // High Plains".to_string(),
                destination_hash: 1475704108,
                activity_hash: 1475704109,
                recommended_light: 750,
                description: "Lawless frontier of the reef, asteroid debris, and Fallen barons.".to_string(),
            },
            DestinationManifest {
                name: "The Dreaming City // Divalian Mists".to_string(),
                destination_hash: 1475704110,
                activity_hash: 1475704111,
                recommended_light: 750,
                description: "Sacred Awoken sanctuary hidden behind cloaking shields.".to_string(),
            },
        ]
    }

    pub fn lookup_destination(hash: u32) -> Option<DestinationManifest> {
        Self::get_available_destinations()
            .into_iter()
            .find(|d| d.destination_hash == hash || d.activity_hash == hash)
    }
}
