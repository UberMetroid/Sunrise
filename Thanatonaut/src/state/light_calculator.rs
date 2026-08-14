// File: linux/src/state/light_calculator.rs
// Title: Light and Power Level Calculator
// Plain English: Computes overall gear power by averaging the 8 active equipment slots.

use crate::error::{Result, SunriseError};

pub const POWER_SLOT_COUNT: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GearSlots {
    pub kinetic: u32,
    pub energy: u32,
    pub power: u32,
    pub helmet: u32,
    pub gauntlets: u32,
    pub chest: u32,
    pub legs: u32,
    pub class_item: u32,
}

impl GearSlots {
    pub fn new(
        kinetic: u32,
        energy: u32,
        power: u32,
        helmet: u32,
        gauntlets: u32,
        chest: u32,
        legs: u32,
        class_item: u32,
    ) -> Self {
        Self {
            kinetic,
            energy,
            power,
            helmet,
            gauntlets,
            chest,
            legs,
            class_item,
        }
    }

    pub fn to_array(&self) -> [u32; POWER_SLOT_COUNT] {
        [
            self.kinetic,
            self.energy,
            self.power,
            self.helmet,
            self.gauntlets,
            self.chest,
            self.legs,
            self.class_item,
        ]
    }
}

pub fn calculate_base_light(slots: &GearSlots) -> u32 {
    let array = slots.to_array();
    let sum: u64 = array.iter().map(|&x| x as u64).sum();
    (sum / (POWER_SLOT_COUNT as u64)) as u32
}

pub fn calculate_light_from_slice(slice: &[u32]) -> Result<u32> {
    if slice.len() != POWER_SLOT_COUNT {
        return Err(SunriseError::InvalidBucketCount(slice.len()));
    }
    let sum: u64 = slice.iter().map(|&x| x as u64).sum();
    Ok((sum / (POWER_SLOT_COUNT as u64)) as u32)
}

pub fn calculate_fractional_light(slots: &GearSlots) -> (u32, u32) {
    let array = slots.to_array();
    let sum: u64 = array.iter().map(|&x| x as u64).sum();
    let base = (sum / 8) as u32;
    let eighths = (sum % 8) as u32;
    (base, eighths)
}

pub fn calculate_max_cross_character_power(
    weapons: &[u32; 3],
    armor: &[u32; 5],
    bonus_power: u32,
) -> u32 {
    let mut sum: u64 = 0;
    for &w in weapons {
        sum += w as u64;
    }
    for &a in armor {
        sum += a as u64;
    }
    let base = (sum / 8) as u32;
    base + bonus_power
}
