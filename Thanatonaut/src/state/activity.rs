// File: Thanatonaut/src/state/activity.rs
// Title: Activity Matchmaking and Destination State
// Plain English: Coordinates destination loading, activity hashes, and session lifecycle.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityPhase {
    Idle = 0,
    Matchmaking = 1,
    Loading = 2,
    InWorld = 3,
    Ending = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivitySession {
    pub session_id: u64,
    pub activity_hash: u32,
    pub destination_hash: u32,
    pub phase: ActivityPhase,
    pub participants: Vec<u64>,
}

impl ActivitySession {
    pub fn new(session_id: u64, activity_hash: u32, destination_hash: u32) -> Self {
        Self {
            session_id,
            activity_hash,
            destination_hash,
            phase: ActivityPhase::Idle,
            participants: Vec::new(),
        }
    }

    pub fn join(&mut self, membership_id: u64) {
        if !self.participants.contains(&membership_id) {
            self.participants.push(membership_id);
        }
    }

    pub fn leave(&mut self, membership_id: u64) {
        self.participants.retain(|&id| id != membership_id);
    }

    pub fn advance_phase(&mut self, new_phase: ActivityPhase) {
        self.phase = new_phase;
    }
}
