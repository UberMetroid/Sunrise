// File: Thanatonaut/src/server/handlers/account.rs
// Title: Account Summary Handler
// Plain English: Returns the serialized AccountState JSON as the response payload.

use crate::error::Result;
use crate::protocol::bap_frame::BapFrame;
use crate::protocol::envelope::{ServiceResponseEnvelope, ServiceStatusCode};
use crate::server::session_handler::SessionHandler;

pub fn handle_account_summary(
    request: &BapFrame,
    session: &mut SessionHandler,
) -> Result<BapFrame> {
    let json_data = serde_json::to_vec(&session.account).unwrap_or_default();
    let response_envelope = ServiceResponseEnvelope::new(
        request.opcode,
        request.transaction_id,
        ServiceStatusCode::Success,
        4,
        json_data,
    );
    let payload = response_envelope.encode()?;
    Ok(BapFrame::new(request.transaction_id, request.opcode, payload))
}