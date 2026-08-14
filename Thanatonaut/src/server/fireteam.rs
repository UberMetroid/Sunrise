// File: Thanatonaut/src/server/fireteam.rs
// Title: Multi-Client Activity Session & Frame Fanout
// Plain English: Tracks which clients are in which activity; routes broadcast frames to peers.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::protocol::bap_frame::BapFrame;
use crate::server::client_registry::ClientRegistry;
use crate::state::activity::ActivitySession;

pub struct Fireteam {
    sessions: Mutex<HashMap<u32, ActivitySession>>,
    registry: Arc<ClientRegistry>,
    next_session_id: AtomicU64,
}

impl Fireteam {
    pub fn new(registry: Arc<ClientRegistry>) -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            registry,
            next_session_id: AtomicU64::new(1),
        }
    }

    pub fn join(&self, activity_hash: u32, destination_hash: u32, membership_id: u64) {
        let mut sessions = self.sessions.lock().expect("Fireteam mutex poisoned");
        let next_id = self.next_session_id.fetch_add(1, Ordering::Relaxed);
        let session = sessions.entry(activity_hash).or_insert_with(|| {
            ActivitySession::new(next_id, activity_hash, destination_hash)
        });
        session.join(membership_id);
    }

    pub fn leave(&self, activity_hash: u32, membership_id: u64) {
        let mut sessions = self.sessions.lock().expect("Fireteam mutex poisoned");
        if let Some(session) = sessions.get_mut(&activity_hash) {
            session.leave(membership_id);
            if session.participants.is_empty() {
                sessions.remove(&activity_hash);
            }
        }
    }

    pub fn broadcast_to_others(&self, activity_hash: u32, sender_id: u64, frame: BapFrame) {
        let snapshot: Vec<u64> = match self
            .sessions
            .lock()
            .expect("Fireteam mutex poisoned")
            .get(&activity_hash)
        {
            Some(s) => s.participants.clone(),
            None => return,
        };
        for peer_id in snapshot {
            if peer_id == sender_id {
                continue;
            }
            if let Some(handle) = self.registry.get(peer_id) {
                handle.outbound.push(frame.clone());
            }
        }
    }

    pub fn participant_count(&self, activity_hash: u32) -> usize {
        self.sessions
            .lock()
            .expect("Fireteam mutex poisoned")
            .get(&activity_hash)
            .map(|s| s.participants.len())
            .unwrap_or(0)
    }

    pub fn participants(&self, activity_hash: u32) -> Vec<u64> {
        self.sessions
            .lock()
            .expect("Fireteam mutex poisoned")
            .get(&activity_hash)
            .map(|s| s.participants.clone())
            .unwrap_or_default()
    }

    pub fn session_count(&self) -> usize {
        self.sessions
            .lock()
            .expect("Fireteam mutex poisoned")
            .len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use crate::protocol::opcode::Opcode;

    fn sock(a: u8, b: u8, c: u8, d: u8, port: u16) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(a, b, c, d)), port)
    }

    fn make_fireteam() -> (Arc<Fireteam>, Arc<ClientRegistry>) {
        let registry = ClientRegistry::new();
        let fireteam = Arc::new(Fireteam::new(registry.clone()));
        (fireteam, registry)
    }

    #[test]
    fn join_creates_session_and_is_idempotent() {
        let (ft, _reg) = make_fireteam();
        ft.join(100, 200, 1);
        ft.join(100, 200, 1);
        ft.join(100, 200, 2);
        assert_eq!(ft.participant_count(100), 2);
        assert_eq!(ft.session_count(), 1);
    }

    #[test]
    fn leave_removes_empty_session() {
        let (ft, _reg) = make_fireteam();
        ft.join(100, 200, 1);
        ft.leave(100, 1);
        assert_eq!(ft.session_count(), 0);
        assert_eq!(ft.participant_count(100), 0);
    }

    #[test]
    fn participants_snapshot() {
        let (ft, _reg) = make_fireteam();
        ft.join(100, 200, 1);
        ft.join(100, 200, 2);
        ft.join(100, 200, 3);
        let mut p = ft.participants(100);
        p.sort();
        assert_eq!(p, vec![1, 2, 3]);
    }

    #[test]
    fn broadcast_to_others_skips_sender_and_routes_via_registry() {
        let (ft, reg) = make_fireteam();
        reg.register(1, "A", sock(127, 0, 0, 1, 1));
        reg.register(2, "B", sock(127, 0, 0, 1, 2));
        reg.register(3, "C", sock(127, 0, 0, 1, 3));
        ft.join(100, 200, 1);
        ft.join(100, 200, 2);
        ft.join(100, 200, 3);

        let frame = BapFrame::new(7, Opcode::Signon, vec![1, 2, 3]);
        ft.broadcast_to_others(100, 2, frame);

        assert_eq!(reg.get(1).unwrap().outbound.len(), 1);
        assert_eq!(reg.get(2).unwrap().outbound.len(), 0);
        assert_eq!(reg.get(3).unwrap().outbound.len(), 1);
    }

    #[test]
    fn missing_session_is_safe() {
        let (ft, _reg) = make_fireteam();
        assert_eq!(ft.participant_count(999), 0);
        assert_eq!(ft.participants(999), Vec::<u64>::new());
        ft.leave(999, 1);
        let frame = BapFrame::new(1, Opcode::Signon, vec![]);
        ft.broadcast_to_others(999, 1, frame);
    }
}