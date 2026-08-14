// File: linux/tests/test_light_calculation.rs
// Title: Light and Power Level E-M Verification Proofs
// Plain English: Validates 8-slot power averaging, rounding down (floor), and inventory power updates.

use thanatonaut::state::account::{CharacterClass, CharacterState};
use thanatonaut::state::inventory::{ItemBucketType, ItemInstance};
use thanatonaut::state::light_calculator::{
    calculate_base_light, calculate_fractional_light, calculate_light_from_slice,
    calculate_max_cross_character_power, GearSlots,
};

#[test]
fn test_uniform_light_calculation() {
    let slots = GearSlots::new(750, 750, 750, 750, 750, 750, 750, 750);
    assert_eq!(calculate_base_light(&slots), 750);

    let (base, eighths) = calculate_fractional_light(&slots);
    assert_eq!(base, 750);
    assert_eq!(eighths, 0);
}

#[test]
fn test_fractional_light_rounding() {
    // Sum = 750*8 + 7 = 6007 -> 6007 / 8 = 750, remainder 7
    let slots = GearSlots::new(751, 751, 751, 751, 751, 751, 751, 750);
    assert_eq!(calculate_base_light(&slots), 750);

    let (base, eighths) = calculate_fractional_light(&slots);
    assert_eq!(base, 750);
    assert_eq!(eighths, 7);
}

#[test]
fn test_light_from_slice_validation() {
    let valid_slice = [750u32; 8];
    assert_eq!(calculate_light_from_slice(&valid_slice).unwrap(), 750);

    let invalid_slice = [750u32; 7];
    assert!(calculate_light_from_slice(&invalid_slice).is_err());
}

#[test]
fn test_cross_character_power_calculation() {
    let weapons = [1050, 1050, 1050];
    let armor = [1000, 1000, 1000, 1000, 1000];
    // Sum = 3150 + 5000 = 8150 / 8 = 1018 base. Bonus = 15 -> 1033
    let total = calculate_max_cross_character_power(&weapons, &armor, 15);
    assert_eq!(total, 1033);
}

#[test]
fn test_character_inventory_recalculate_light() {
    let mut character = CharacterState::new(2001, CharacterClass::Hunter);

    let buckets = [
        ItemBucketType::KineticWeapons,
        ItemBucketType::EnergyWeapons,
        ItemBucketType::PowerWeapons,
        ItemBucketType::Helmet,
        ItemBucketType::Gauntlets,
        ItemBucketType::ChestArmor,
        ItemBucketType::LegArmor,
        ItemBucketType::ClassArmor,
    ];

    for (idx, &bucket) in buckets.iter().enumerate() {
        let mut item = ItemInstance::new((idx + 1) as u64, 100000 + idx as u32, 1, 950, bucket);
        item.is_equipped = true;
        character.inventory.add_item(item);
    }

    character.recalculate_light();
    assert_eq!(character.light_level, 950);
}
