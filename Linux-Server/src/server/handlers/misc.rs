// File: Linux-Server/src/server/handlers/misc.rs
// Title: Default Opcode Catch-All Handler
// Plain English: Returns a success envelope with empty payload for any unmapped opcode.

use crate::error::Result;
use crate::protocol::bap_frame::BapFrame;
use crate::protocol::envelope::{ServiceResponseEnvelope, ServiceStatusCode};
use crate::server::session_handler::SessionHandler;

pub fn handle_default(
    request: &BapFrame,
    _session: &mut SessionHandler,
) -> Result<BapFrame> {
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