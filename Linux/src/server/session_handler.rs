// File: linux/src/server/session_handler.rs
// Title: BAP Session Dispatcher
// Plain English: Dispatches incoming frames to appropriate handlers and builds responses.

use crate::error::Result;
use crate::protocol::bap_frame::BapFrame;
use crate::protocol::envelope::{ServiceResponseEnvelope, ServiceStatusCode};
use crate::protocol::opcode::Opcode;
use crate::state::account::AccountState;

pub struct SessionHandler {
    pub account: AccountState,
}

impl SessionHandler {
    pub fn new(membership_id: u64, display_name: impl Into<String>) -> Self {
        Self {
            account: AccountState::new(membership_id, display_name),
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
                // QueueZ status response (instant admission)
                let payload = vec![0x08, 0x01, 0x10, 0x00];
                Ok(BapFrame::new(request.transaction_id, Opcode::QueueZ, payload))
            }
            Opcode::AccountSummary => {
                let response_envelope = ServiceResponseEnvelope::new(
                    Opcode::AccountSummary,
                    request.transaction_id,
                    ServiceStatusCode::Success,
                    4,
                    vec![0xAA, 0xBB, 0xCC, 0xDD],
                );
                let payload = response_envelope.encode()?;
                Ok(BapFrame::new(request.transaction_id, Opcode::AccountSummary, payload))
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
