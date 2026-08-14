// File: linux/src/server/handlers/signon.rs
// Title: Signon & BindUdp Handlers
// Plain English: Handles TCP Signon (Steam ID parsing) and BindUdp (UDP source binding).

use crate::error::{Result, SunriseError};
use crate::protocol::bap_frame::BapFrame;
use crate::protocol::envelope::{ServiceResponseEnvelope, ServiceStatusCode};
use crate::server::session_handler::SessionHandler;

#[derive(Debug, serde::Deserialize)]
pub struct SignonPayload {
    #[serde(default)]
    pub steam_id: Option<u64>,
    #[serde(default)]
    pub display_name: Option<String>,
}

pub fn handle_signon(request: &BapFrame, session: &mut SessionHandler) -> Result<BapFrame> {
    let parsed: Option<SignonPayload> = if request.payload.is_empty() {
        None
    } else {
        serde_json::from_slice(&request.payload).ok()
    };

    if let Some(p) = parsed {
        if let Some(steam) = p.steam_id {
            session.steam_id = Some(steam);
            if let Some(name) = p.display_name {
                session.account.display_name = name;
            }
            session.ephemeral = false;
            session.refresh_account_from_steam();
        }
    }

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

pub fn handle_bind_udp(request: &BapFrame, session: &mut SessionHandler) -> Result<BapFrame> {
    #[derive(Debug, serde::Deserialize)]
    struct BindPayload {
        #[serde(default)]
        udp_port: Option<u16>,
    }

    let parsed: Option<BindPayload> = if request.payload.is_empty() {
        None
    } else {
        serde_json::from_slice(&request.payload).ok()
    };

    let udp_port = match parsed.and_then(|p| p.udp_port) {
        Some(p) if p > 0 => p,
        _ => {
            let envelope = ServiceResponseEnvelope::new(
                request.opcode,
                request.transaction_id,
                ServiceStatusCode::MalformedRequest,
                1,
                Vec::new(),
            );
            return Ok(BapFrame::new(
                request.transaction_id,
                request.opcode,
                envelope.encode()?,
            ));
        }
    };

    let ip = session.peer_addr.ip();
    let full_addr = std::net::SocketAddr::new(ip, udp_port);
    session.registry_handle.set_udp_source(Some(full_addr));

    let envelope = ServiceResponseEnvelope::new(
        request.opcode,
        request.transaction_id,
        ServiceStatusCode::Success,
        1,
        serde_json::to_vec(&serde_json::json!({
            "udp_source": full_addr.to_string(),
        }))
        .unwrap_or_default(),
    );
    Ok(BapFrame::new(
        request.transaction_id,
        request.opcode,
        envelope.encode()?,
    ))
}

impl SessionHandler {
    pub fn refresh_account_from_steam(&mut self) {
        if let Some(steam_id) = self.steam_id {
            let store = self.store.clone();
            let account = store.load_or_create_account_by_steam(steam_id);
            self.account = account;
        }
    }
}

#[allow(dead_code)]
pub(crate) fn _suppress_unused_error() -> Result<()> {
    Err(SunriseError::ConnectionClosed)
}