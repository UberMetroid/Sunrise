// File: linux-server/src/server/session_handler.rs
// Title: BAP Session Dispatcher & Live Profile State Manager
// Plain English: Dispatches incoming frames, equips items, loads destinations, and persists profiles.

use std::net::SocketAddr;
use std::sync::Arc;

use crate::error::Result;
use crate::protocol::bap_frame::BapFrame;
use crate::protocol::opcode::Opcode;
use crate::server::client_registry::{ClientHandle, ClientRegistry};
use crate::server::fireteam::Fireteam;
use crate::state::account::AccountState;
use crate::state::profile_store::ProfileStore;

pub struct SessionHandler {
    pub account: AccountState,
    pub store: ProfileStore,
    pub active_destination_hash: u32,
    pub steam_id: Option<u64>,
    pub ephemeral: bool,
    pub peer_addr: SocketAddr,
    pub registry_handle: ClientHandle,
    pub registry: Arc<ClientRegistry>,
    pub fireteam: Arc<Fireteam>,
}

impl SessionHandler {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        membership_id: u64,
        display_name: impl Into<String>,
        peer_addr: SocketAddr,
        registry: Arc<ClientRegistry>,
        fireteam: Arc<Fireteam>,
    ) -> Self {
        let store = ProfileStore::default_store();
        let name_str = display_name.into();
        let account = store.load_or_create_account(membership_id, &name_str);
        let handle = registry.register(membership_id, &name_str, peer_addr);

        Self {
            account,
            store,
            active_destination_hash: 373711905,
            steam_id: None,
            ephemeral: true,
            peer_addr,
            registry_handle: handle,
            registry,
            fireteam,
        }
    }

    pub fn handle_frame(&mut self, request: &BapFrame) -> Result<BapFrame> {
        match request.opcode {
            Opcode::Signon | Opcode::SignonExtended => {
                crate::server::handlers::signon::handle_signon(request, self)
            }
            Opcode::BindUdp => {
                crate::server::handlers::signon::handle_bind_udp(request, self)
            }
            Opcode::AccountSummary => {
                crate::server::handlers::account::handle_account_summary(request, self)
            }
            Opcode::CharacterInventory => {
                crate::server::handlers::inventory::handle_character_inventory(request, self)
            }
            Opcode::EquipItem => {
                crate::server::handlers::inventory::handle_equip_item(request, self)
            }
            Opcode::TransferItem => {
                crate::server::handlers::inventory::handle_transfer_item(request, self)
            }
            Opcode::SetItemLockState => {
                crate::server::handlers::inventory::handle_set_item_lock_state(request, self)
            }
            Opcode::SetSocketSelection => {
                crate::server::handlers::inventory::handle_set_socket_selection(request, self)
            }
            Opcode::QueueZ => {
                crate::server::handlers::activity::handle_queuez(request, self)
            }
            Opcode::ActivityMatchmaking => {
                crate::server::handlers::activity::handle_activity_matchmaking(request, self)
            }
            Opcode::ActivityJoin => {
                crate::server::handlers::activity::handle_activity_join(request, self)
            }
            Opcode::ActivityLeave => {
                crate::server::handlers::activity::handle_activity_leave(request, self)
            }
            Opcode::FireteamBroadcast => {
                crate::server::handlers::activity::handle_fireteam_broadcast(request, self)
            }
            _ => crate::server::handlers::misc::handle_default(request, self),
        }
    }
}