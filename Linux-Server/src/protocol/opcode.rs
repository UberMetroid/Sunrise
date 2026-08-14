// File: Linux-Server/src/protocol/opcode.rs
// Title: BAP Opcode Registry
// Plain English: Maps numerical operation codes to readable service names.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    Signon,
    SignonExtended,
    QueueZ,
    AccountSummary,
    CharacterInventory,
    EquipItem,
    TransferItem,
    SetItemLockState,
    SetSocketSelection,
    ActivityMatchmaking,
    ActivityJoin,
    ActivityLeave,
    FireteamBroadcast,
    BindUdp,
    Unknown(u16),
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        match value {
            0x0101 => Self::Signon,
            0x0102 => Self::SignonExtended,
            0x0103 => Self::QueueZ,
            0x0205 => Self::AccountSummary,
            0x0206 => Self::CharacterInventory,
            0x0501 => Self::EquipItem,
            0x0503 => Self::TransferItem,
            0x0504 => Self::SetItemLockState,
            0x0505 => Self::SetSocketSelection,
            0x0601 => Self::ActivityMatchmaking,
            0x0602 => Self::ActivityJoin,
            0x0603 => Self::ActivityLeave,
            0x0604 => Self::FireteamBroadcast,
            0x0701 => Self::BindUdp,
            other => Self::Unknown(other),
        }
    }
}

impl From<Opcode> for u16 {
    fn from(op: Opcode) -> Self {
        match op {
            Opcode::Signon => 0x0101,
            Opcode::SignonExtended => 0x0102,
            Opcode::QueueZ => 0x0103,
            Opcode::AccountSummary => 0x0205,
            Opcode::CharacterInventory => 0x0206,
            Opcode::EquipItem => 0x0501,
            Opcode::TransferItem => 0x0503,
            Opcode::SetItemLockState => 0x0504,
            Opcode::SetSocketSelection => 0x0505,
            Opcode::ActivityMatchmaking => 0x0601,
            Opcode::ActivityJoin => 0x0602,
            Opcode::ActivityLeave => 0x0603,
            Opcode::FireteamBroadcast => 0x0604,
            Opcode::BindUdp => 0x0701,
            Opcode::Unknown(val) => val,
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Signon => write!(f, "Signon (0x0101)"),
            Self::SignonExtended => write!(f, "SignonExtended (0x0102)"),
            Self::QueueZ => write!(f, "QueueZ (0x0103)"),
            Self::AccountSummary => write!(f, "AccountSummary (0x0205)"),
            Self::CharacterInventory => write!(f, "CharacterInventory (0x0206)"),
            Self::EquipItem => write!(f, "EquipItem (0x0501)"),
            Self::TransferItem => write!(f, "TransferItem (0x0503)"),
            Self::SetItemLockState => write!(f, "SetItemLockState (0x0504)"),
            Self::SetSocketSelection => write!(f, "SetSocketSelection (0x0505)"),
            Self::ActivityMatchmaking => write!(f, "ActivityMatchmaking (0x0601)"),
            Self::ActivityJoin => write!(f, "ActivityJoin (0x0602)"),
            Self::ActivityLeave => write!(f, "ActivityLeave (0x0603)"),
            Self::FireteamBroadcast => write!(f, "FireteamBroadcast (0x0604)"),
            Self::BindUdp => write!(f, "BindUdp (0x0701)"),
            Self::Unknown(val) => write!(f, "UnknownOpcode (0x{:04X})", val),
        }
    }
}