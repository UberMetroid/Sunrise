// File: linux-server/tests/test_profile_store.rs
// Title: ProfileStore Unit and Integration Tests
// Plain English: Verifies character creation, loadout defaults, and disk persistence.

use std::fs;
use std::path::PathBuf;

use sunrise_linux::state::account::CharacterClass;
use sunrise_linux::state::inventory::ItemBucketType;
use sunrise_linux::state::profile_store::ProfileStore;

#[test]
fn test_profile_store_create_and_persist() {
    let test_dir = PathBuf::from("/tmp/sunrise_test_profiles");
    let _ = fs::remove_dir_all(&test_dir);
    let _ = fs::create_dir_all(&test_dir);
    let test_file = test_dir.join("test_profiles.json");

    let store = ProfileStore::with_path(&test_file);
    let account = store.load_or_create_account(4611686018428383838, "GuardianTester");

    assert_eq!(account.display_name, "GuardianTester");
    assert_eq!(account.characters.len(), 3);

    // Verify Titan Starter Loadout
    let titan = account.characters.iter().find(|c| c.class_type == CharacterClass::Titan).unwrap();
    assert!(titan.light_level >= 1050);
    assert!(titan.inventory.get_equipped_item(ItemBucketType::KineticWeapons).is_some());
    assert!(titan.inventory.get_equipped_item(ItemBucketType::EnergyWeapons).is_some());
    assert!(titan.inventory.get_equipped_item(ItemBucketType::PowerWeapons).is_some());

    // Verify Reload from Disk
    let reloaded = store.load_account(4611686018428383838).unwrap();
    assert_eq!(reloaded.membership_id, 4611686018428383838);
    assert_eq!(reloaded.characters.len(), 3);

    let _ = fs::remove_dir_all(&test_dir);
}
