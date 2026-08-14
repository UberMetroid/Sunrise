// File: Thanatonaut/src/server/handlers/inventory.rs
// Title: Inventory & Equipment Handlers
// Plain English: CharacterInventory, EquipItem, TransferItem, SetItemLockState, SetSocketSelection.

use crate::error::Result;
use crate::protocol::bap_frame::BapFrame;
use crate::protocol::envelope::{ServiceResponseEnvelope, ServiceStatusCode};
use crate::server::session_handler::SessionHandler;

pub fn handle_character_inventory(
    request: &BapFrame,
    session: &mut SessionHandler,
) -> Result<BapFrame> {
    let json_data = serde_json::to_vec(&session.account.characters).unwrap_or_default();
    let envelope = ServiceResponseEnvelope::new(
        request.opcode,
        request.transaction_id,
        ServiceStatusCode::Success,
        4,
        json_data,
    );
    let payload = envelope.encode()?;
    Ok(BapFrame::new(request.transaction_id, request.opcode, payload))
}

pub fn handle_equip_item(
    request: &BapFrame,
    session: &mut SessionHandler,
) -> Result<BapFrame> {
    if let Some(character) = session.account.characters.first_mut() {
        character.recalculate_light();
    }
    let _ = session.store.save_account(&session.account);
    success_empty(request)
}

pub fn handle_transfer_item(
    request: &BapFrame,
    session: &mut SessionHandler,
) -> Result<BapFrame> {
    let _ = session.store.save_account(&session.account);
    success_empty(request)
}

pub fn handle_set_item_lock_state(
    request: &BapFrame,
    session: &mut SessionHandler,
) -> Result<BapFrame> {
    let _ = session.store.save_account(&session.account);
    success_empty(request)
}

pub fn handle_set_socket_selection(
    request: &BapFrame,
    session: &mut SessionHandler,
) -> Result<BapFrame> {
    let _ = session.store.save_account(&session.account);
    success_empty(request)
}

fn success_empty(request: &BapFrame) -> Result<BapFrame> {
    let envelope = ServiceResponseEnvelope::new(
        request.opcode,
        request.transaction_id,
        ServiceStatusCode::Success,
        1,
        Vec::new(),
    );
    let payload = envelope.encode()?;
    Ok(BapFrame::new(request.transaction_id, request.opcode, payload))
}