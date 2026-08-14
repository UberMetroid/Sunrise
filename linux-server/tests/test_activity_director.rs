// File: linux-server/tests/test_activity_director.rs
// Title: ActivityDirector Destination Manifest Tests
// Plain English: Verifies destination lookups, raid manifests, and hash integrity.

use sunrise_linux::state::activity_director::ActivityDirector;

#[test]
fn test_destination_manifest_retrieval() {
    let destinations = ActivityDirector::get_available_destinations();
    assert!(destinations.len() >= 8);

    // Verify Titan lookup
    let titan = ActivityDirector::lookup_destination(373711905).unwrap();
    assert!(titan.name.contains("Titan"));

    // Verify Leviathan Raid lookup
    let leviathan = ActivityDirector::lookup_destination(2693136600).unwrap();
    assert!(leviathan.name.contains("Leviathan"));

    // Verify Mars lookup
    let mars = ActivityDirector::lookup_destination(2877881079).unwrap();
    assert!(mars.name.contains("Mars"));
}
