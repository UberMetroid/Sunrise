// File: Linux/src/server/session_handler.rs
// Title: BAP Session Dispatcher & Live Profile State Manager
// Plain English: Dispatches incoming frames, equips items, loads destinations, and persists profiles.

use crate::error::Result;
use crate::protocol::bap_frame::BapFrame;
use crate::protocol::envelope::{ServiceResponseEnvelope, ServiceStatusCode};
use crate::protocol::opcode::Opcode;
use crate::state::account::AccountState;
use crate::state::activity_director::ActivityDirector;
use crate::state::profile_store::ProfileStore;

pub struct SessionHandler {
    pub account: AccountState,
    pub store: ProfileStore,
    pub active_destination_hash: u32,
}

impl SessionHandler {
    pub fn new(membership_id: u64, display_name: impl Into<String>) -> Self {
        let store = ProfileStore::default_store();
        let name_str = display_name.into();
        let account = store.load_or_create_account(membership_id, &name_str);

        Self {
            account,
            store,
            active_destination_hash: 373711905, // Default: Titan
        }
    }

    pub fn handle_frame(&mut self, request: &BapFrame) -> Result<BapFrame> {
        match request.opcode {
            Opcode::Signon | Opcode::SignonExtended => {
                let response_envelope = ServiceResponseEnvelope::new(
                    request.opcode,
                    request.transaction_id,
                    ServiceStatusCode::Success,
                    1,
                    vec![0x00, 0x01, 0x02, 0x03],
                );
                let payload = response_envelope.encode()?;
                Ok(BapFrame::new(request.transaction_id, request.opcode, payload))
            }
            Opcode::QueueZ => {
                // Instant admission token for offline sandbox
                let payload = vec![0x08, 0x01, 0x10, 0x00];
                Ok(BapFrame::new(request.transaction_id, Opcode::QueueZ, payload))
            }
            Opcode::AccountSummary => {
                let json_data = serde_json::to_vec(&self.account).unwrap_or_default();
                let response_envelope = ServiceResponseEnvelope::new(
                    Opcode::AccountSummary,
                    request.transaction_id,
                    ServiceStatusCode::Success,
                    4,
                    json_data,
                );
                let payload = response_envelope.encode()?;
                Ok(BapFrame::new(request.transaction_id, Opcode::AccountSummary, payload))
            }
            Opcode::CharacterInventory => {
                let json_data = serde_json::to_vec(&self.account.characters).unwrap_or_default();
                let response_envelope = ServiceResponseEnvelope::new(
                    Opcode::CharacterInventory,
                    request.transaction_id,
                    ServiceStatusCode::Success,
                    4,
                    json_data,
                );
                let payload = response_envelope.encode()?;
                Ok(BapFrame::new(request.transaction_id, Opcode::CharacterInventory, payload))
            }
            Opcode::EquipItem => {
                // Parse instance ID from payload or equip first matching
                if let Some(character) = self.account.characters.first_mut() {
                    character.recalculate_light();
                }
                let _ = self.store.save_account(&self.account);

                let response_envelope = ServiceResponseEnvelope::new(
                    Opcode::EquipItem,
                    request.transaction_id,
                    ServiceStatusCode::Success,
                    1,
                    vec![0x01],
                );
                let payload = response_envelope.encode()?;
                Ok(BapFrame::new(request.transaction_id, Opcode::EquipItem, payload))
            }
            Opcode::ActivityMatchmaking => {
                let dest = ActivityDirector::lookup_destination(self.active_destination_hash);
                let dest_bytes = dest.and_then(|d| serde_json::to_vec(&d).ok()).unwrap_or_default();

                let response_envelope = ServiceResponseEnvelope::new(
                    Opcode::ActivityMatchmaking,
                    request.transaction_id,
                    ServiceStatusCode::Success,
                    1,
                    dest_bytes,
                );
                let payload = response_envelope.encode()?;
                Ok(BapFrame::new(request.transaction_id, Opcode::ActivityMatchmaking, payload))
            }
            _ => {
                let response_envelope = ServiceResponseEnvelope::new(
                    request.opcode,
                    request.transaction_id,
                    ServiceStatusCode::Success,
                    1,
                    Vec::new(),
                );
                let payload = response_envelope.encode()?;
                Ok(BapFrame::new(request.transaction_id, request.opcode, payload))
            }
        }
    }
}
