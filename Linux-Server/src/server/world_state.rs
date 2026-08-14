// File: Linux-Server/src/server/world_state.rs
// Title: Per-Membership Player Snapshot Store
// Plain English: Tracks the last-known position of every connected player; snapshots for fanout.

use std::collections::HashMap;
use std::sync::Mutex;

use crate::protocol::udp_packet::{PlayerPosition, WorldSnapshot};

pub struct WorldState {
    players: Mutex<HashMap<u64, PlayerPosition>>,
    sequence: Mutex<u32>,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            players: Mutex::new(HashMap::new()),
            sequence: Mutex::new(0),
        }
    }

    pub fn update_position(&self, membership_id: u64, position: PlayerPosition) {
        let mut players = self.players.lock().expect("WorldState mutex poisoned");
        players.insert(membership_id, position);
    }

    pub fn remove_player(&self, membership_id: u64) {
        let mut players = self.players.lock().expect("WorldState mutex poisoned");
        players.remove(&membership_id);
    }

    pub fn snapshot(&self) -> WorldSnapshot {
        let players = self.players.lock().expect("WorldState mutex poisoned");
        let mut seq = self.sequence.lock().expect("WorldState mutex poisoned");
        *seq = seq.wrapping_add(1);
        let entries: Vec<(u64, PlayerPosition)> = players
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect();
        WorldSnapshot {
            sequence: *seq,
            players: entries,
        }
    }

    pub fn player_count(&self) -> usize {
        let players = self.players.lock().expect("WorldState mutex poisoned");
        players.len()
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_and_snapshot_roundtrip() {
        let ws = WorldState::new();
        ws.update_position(1001, PlayerPosition { x: 1.0, y: 2.0, z: 3.0, yaw: 0.0, pitch: 0.0 });
        ws.update_position(1002, PlayerPosition { x: 4.0, y: 5.0, z: 6.0, yaw: 1.0, pitch: 0.5 });

        let snap = ws.snapshot();
        assert_eq!(snap.players.len(), 2);
        assert!(snap.sequence >= 1);

        let mut by_id: HashMap<u64, PlayerPosition> = snap.players.into_iter().collect();
        assert_eq!(by_id.remove(&1001).unwrap().x, 1.0);
        assert_eq!(by_id.remove(&1002).unwrap().z, 6.0);
    }

    #[test]
    fn remove_player_drops_from_snapshot() {
        let ws = WorldState::new();
        ws.update_position(1001, PlayerPosition { x: 0.0, y: 0.0, z: 0.0, yaw: 0.0, pitch: 0.0 });
        ws.update_position(1002, PlayerPosition { x: 1.0, y: 1.0, z: 1.0, yaw: 0.0, pitch: 0.0 });
        assert_eq!(ws.player_count(), 2);

        ws.remove_player(1001);
        assert_eq!(ws.player_count(), 1);
    }

    #[test]
    fn sequence_increments_per_snapshot() {
        let ws = WorldState::new();
        let s1 = ws.snapshot().sequence;
        let s2 = ws.snapshot().sequence;
        assert_eq!(s2, s1.wrapping_add(1));
    }
}