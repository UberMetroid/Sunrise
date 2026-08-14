// File: Thanatonaut/src/server/client_registry.rs
// Title: Multi-Client Registry & Steam ID Identity Resolver
// Plain English: Tracks live client sessions, maps SteamID64 to membership_id, owns outbound queues.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicU64;

use crate::server::outbound_queue::OutboundQueue;

#[derive(Debug, Clone)]
pub struct ClientHandle {
    pub membership_id: u64,
    pub display_name: String,
    pub ephemeral: bool,
    pub outbound: OutboundQueue,
    udp_source: Arc<Mutex<Option<SocketAddr>>>,
}

impl ClientHandle {
    pub fn new(membership_id: u64, display_name: impl Into<String>, ephemeral: bool) -> Self {
        Self {
            membership_id,
            display_name: display_name.into(),
            ephemeral,
            outbound: OutboundQueue::new(),
            udp_source: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_udp_source(&self, addr: Option<SocketAddr>) {
        let mut guard = self.udp_source.lock().expect("ClientHandle mutex poisoned");
        *guard = addr;
    }

    pub fn udp_source(&self) -> Option<SocketAddr> {
        let guard = self.udp_source.lock().expect("ClientHandle mutex poisoned");
        *guard
    }
}

#[derive(Debug)]
pub struct ClientRegistry {
    by_membership: Mutex<HashMap<u64, ClientHandle>>,
    by_udp_source: Mutex<HashMap<SocketAddr, u64>>,
    #[allow(dead_code)]
    next_ephemeral_id: AtomicU64,
}

impl ClientRegistry {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            by_membership: Mutex::new(HashMap::new()),
            by_udp_source: Mutex::new(HashMap::new()),
            next_ephemeral_id: AtomicU64::new(1000),
        })
    }

    pub fn register(&self, membership_id: u64, display_name: &str, peer_addr: SocketAddr) -> ClientHandle {
        let stale_udp = {
            let by_membership = self.by_membership.lock().expect("Registry mutex poisoned");
            by_membership.get(&membership_id).and_then(|h| h.udp_source())
        };
        if let Some(prev) = stale_udp {
            if prev != peer_addr {
                self.by_udp_source.lock().expect("Registry mutex poisoned").remove(&prev);
            }
        }

        let handle = ClientHandle {
            membership_id,
            display_name: display_name.to_string(),
            ephemeral: true,
            outbound: OutboundQueue::new(),
            udp_source: Arc::new(Mutex::new(Some(peer_addr))),
        };

        self.by_membership.lock().expect("Registry mutex poisoned").insert(membership_id, handle.clone());
        self.by_udp_source.lock().expect("Registry mutex poisoned").insert(peer_addr, membership_id);
        handle
    }

    pub fn upsert(&self, handle: ClientHandle) -> ClientHandle {
        let membership_id = handle.membership_id;
        if let Some(addr) = handle.udp_source() {
            let mut by_udp = self.by_udp_source.lock().expect("Registry mutex poisoned");
            let stale: Vec<SocketAddr> = by_udp
                .iter()
                .filter_map(|(k, v)| if *v == membership_id && *k != addr { Some(*k) } else { None })
                .collect();
            for k in stale { by_udp.remove(&k); }
            by_udp.insert(addr, membership_id);
        }
        self.by_membership.lock().expect("Registry mutex poisoned").insert(membership_id, handle.clone());
        handle
    }

    pub fn unregister(&self, membership_id: u64) {
        let removed = self.by_membership.lock().expect("Registry mutex poisoned").remove(&membership_id);
        if let Some(handle) = removed {
            if let Some(addr) = handle.udp_source() {
                let mut by_udp = self.by_udp_source.lock().expect("Registry mutex poisoned");
                if matches!(by_udp.get(&addr).copied(), Some(id) if id == membership_id) {
                    by_udp.remove(&addr);
                }
            }
        }
    }

    pub fn get(&self, membership_id: u64) -> Option<ClientHandle> {
        self.by_membership.lock().expect("Registry mutex poisoned").get(&membership_id).cloned()
    }

    pub fn lookup_by_udp(&self, source: SocketAddr) -> Option<ClientHandle> {
        let membership_id = {
            let by_udp = self.by_udp_source.lock().expect("Registry mutex poisoned");
            by_udp.get(&source).copied()?
        };
        self.by_membership.lock().expect("Registry mutex poisoned").get(&membership_id).cloned()
    }

    pub fn bind_udp_source(&self, membership_id: u64, source: SocketAddr) -> bool {
        let handle = {
            let by_membership = self.by_membership.lock().expect("Registry mutex poisoned");
            match by_membership.get(&membership_id) {
                Some(h) => h.clone(),
                None => return false,
            }
        };

        let mut by_udp = self.by_udp_source.lock().expect("Registry mutex poisoned");
        if let Some(prev) = handle.udp_source() {
            if prev != source {
                if matches!(by_udp.get(&prev).copied(), Some(id) if id == membership_id) {
                    by_udp.remove(&prev);
                }
            }
        }
        by_udp.retain(|_, v| *v != membership_id);
        by_udp.insert(source, membership_id);
        drop(by_udp);
        handle.set_udp_source(Some(source));
        true
    }

    pub fn client_count(&self) -> usize {
        self.by_membership.lock().expect("Registry mutex poisoned").len()
    }

    pub fn all_membership_ids(&self) -> Vec<u64> {
        self.by_membership.lock().expect("Registry mutex poisoned").keys().copied().collect()
    }
}

impl Default for ClientRegistry {
    fn default() -> Self {
        Self {
            by_membership: Mutex::new(HashMap::new()),
            by_udp_source: Mutex::new(HashMap::new()),
            next_ephemeral_id: AtomicU64::new(1000),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    fn sock(a: u8, b: u8, c: u8, d: u8, port: u16) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(a, b, c, d)), port)
    }

    #[test]
    fn register_records_both_indices() {
        let reg = ClientRegistry::new();
        let peer = sock(127, 0, 0, 1, 5000);
        let handle = reg.register(42, "Guardian", peer);
        assert_eq!(handle.membership_id, 42);
        assert_eq!(handle.display_name, "Guardian");
        assert_eq!(reg.client_count(), 1);
        assert_eq!(reg.lookup_by_udp(peer).map(|h| h.membership_id), Some(42));
    }

    #[test]
    fn unregister_drops_udp_mapping() {
        let reg = ClientRegistry::new();
        let peer = sock(10, 0, 0, 1, 7000);
        let h = reg.register(7, "Ephemeral", peer);
        reg.unregister(h.membership_id);
        assert!(reg.lookup_by_udp(peer).is_none());
        assert_eq!(reg.client_count(), 0);
    }
}
