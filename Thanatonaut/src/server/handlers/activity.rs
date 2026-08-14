// File: Thanatonaut/src/server/handlers/activity.rs
// Title: Activity & Fireteam Handlers
// Plain English: QueueZ, ActivityMatchmaking, ActivityJoin/Leave, FireteamBroadcast fanout.

use crate::error::Result;
use crate::protocol::bap_frame::BapFrame;
use crate::protocol::envelope::{ServiceResponseEnvelope, ServiceStatusCode};
use crate::protocol::opcode::Opcode;
use crate::server::session_handler::SessionHandler;
use crate::state::activity_director::ActivityDirector;

#[derive(Debug, serde::Deserialize)]
pub struct ActivityPayload {
    #[serde(default)]
    pub activity_hash: Option<u32>,
    #[serde(default)]
    pub destination_hash: Option<u32>,
}

pub fn handle_queuez(request: &BapFrame, _session: &mut SessionHandler) -> Result<BapFrame> {
    let payload = vec![0x08, 0x01, 0x10, 0x00];
    Ok(BapFrame::new(request.transaction_id, Opcode::QueueZ, payload))
}

pub fn handle_activity_matchmaking(
    request: &BapFrame,
    session: &mut SessionHandler,
) -> Result<BapFrame> {
    let dest =
        ActivityDirector::lookup_destination(session.active_destination_hash);
    let dest_bytes = dest
        .and_then(|d| serde_json::to_vec(&d).ok())
        .unwrap_or_default();

    let response_envelope = ServiceResponseEnvelope::new(
        request.opcode,
        request.transaction_id,
        ServiceStatusCode::Success,
        1,
        dest_bytes,
    );
    let payload = response_envelope.encode()?;
    Ok(BapFrame::new(
        request.transaction_id,
        request.opcode,
        payload,
    ))
}

pub fn handle_activity_join(
    request: &BapFrame,
    session: &mut SessionHandler,
) -> Result<BapFrame> {
    let parsed: Option<ActivityPayload> = serde_json::from_slice(&request.payload).ok();
    let (activity_hash, destination_hash) = match parsed {
        Some(p) => (
            p.activity_hash.unwrap_or(session.active_destination_hash),
            p.destination_hash.unwrap_or(session.active_destination_hash),
        ),
        None => (
            session.active_destination_hash,
            session.active_destination_hash,
        ),
    };

    session.fireteam.join(
        activity_hash,
        destination_hash,
        session.account.membership_id,
    );
    session.active_destination_hash = destination_hash;

    let response_envelope = ServiceResponseEnvelope::new(
        request.opcode,
        request.transaction_id,
        ServiceStatusCode::Success,
        1,
        serde_json::to_vec(&serde_json::json!({
            "activity_hash": activity_hash,
            "destination_hash": destination_hash,
            "membership_id": session.account.membership_id,
        }))
        .unwrap_or_default(),
    );
    let payload = response_envelope.encode()?;
    Ok(BapFrame::new(request.transaction_id, request.opcode, payload))
}

pub fn handle_activity_leave(
    request: &BapFrame,
    session: &mut SessionHandler,
) -> Result<BapFrame> {
    let parsed: Option<ActivityPayload> = serde_json::from_slice(&request.payload).ok();
    let activity_hash = parsed
        .and_then(|p| p.activity_hash)
        .unwrap_or(session.active_destination_hash);

    session
        .fireteam
        .leave(activity_hash, session.account.membership_id);

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

pub fn handle_fireteam_broadcast(
    request: &BapFrame,
    session: &mut SessionHandler,
) -> Result<BapFrame> {
    let parsed: Option<ActivityPayload> = serde_json::from_slice(&request.payload).ok();
    let activity_hash = parsed
        .and_then(|p| p.activity_hash)
        .unwrap_or(session.active_destination_hash);

    session.fireteam.broadcast_to_others(
        activity_hash,
        session.account.membership_id,
        request.clone(),
    );

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