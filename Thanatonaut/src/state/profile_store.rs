// File: Thanatonaut/src/state/profile_store.rs
// Title: Persistent Player Profile File Storage
// Plain English: Saves and loads Guardian profiles, characters, and gear inventories to disk.

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Result, SunriseError};
use crate::installer::steam_locator::get_home_dir;
use crate::state::account::{AccountState, CharacterClass};
use crate::state::starter_loadout::StarterLoadoutFactory;

#[derive(Clone)]
pub struct ProfileStore {
    storage_path: PathBuf,
}

pub const STEAM_ID_NAMESPACE: u64 = 0x53555253_53554E52; // "SUNRSUNR" — deterministic hash namespace

impl ProfileStore {
    pub fn default_store() -> Self {
        let path = if Path::new("/data").exists() {
            PathBuf::from("/data/profiles.json")
        } else {
            get_home_dir().join(".config").join("thanatonaut").join("profiles.json")
        };
        Self { storage_path: path }
    }

    pub fn with_path(path: impl Into<PathBuf>) -> Self {
        Self { storage_path: path.into() }
    }

    pub fn hash_steam_id_to_membership(steam_id: u64) -> u64 {
        // Deterministic splitmix64-style hash so the same Steam ID always
        // maps to the same membership_id across reboots.
        let mut z = steam_id.wrapping_add(STEAM_ID_NAMESPACE);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    pub fn load_or_create_account_by_steam(&self, steam_id: u64) -> AccountState {
        let membership_id = Self::hash_steam_id_to_membership(steam_id);
        let display_name = format!("Guardian-{:x}", steam_id & 0xFFFF);

        if let Ok(account) = self.load_account(membership_id) {
            return account;
        }

        let mut new_account = AccountState::new(membership_id, &display_name);
        new_account.add_character(StarterLoadoutFactory::create_default_character(
            1001, CharacterClass::Titan,
        ));
        new_account.add_character(StarterLoadoutFactory::create_default_character(
            1002, CharacterClass::Hunter,
        ));
        new_account.add_character(StarterLoadoutFactory::create_default_character(
            1003, CharacterClass::Warlock,
        ));

        let _ = self.save_account(&new_account);
        new_account
    }

    pub fn load_or_create_account(&self, membership_id: u64, name: &str) -> AccountState {
        if let Ok(account) = self.load_account(membership_id) {
            return account;
        }

        let mut new_account = AccountState::new(membership_id, name);
        new_account.add_character(StarterLoadoutFactory::create_default_character(1001, CharacterClass::Titan));
        new_account.add_character(StarterLoadoutFactory::create_default_character(1002, CharacterClass::Hunter));
        new_account.add_character(StarterLoadoutFactory::create_default_character(1003, CharacterClass::Warlock));

        let _ = self.save_account(&new_account);
        new_account
    }

    pub fn load_account(&self, membership_id: u64) -> Result<AccountState> {
        if !self.storage_path.exists() {
            return Err(SunriseError::FileNotFound(self.storage_path.display().to_string()));
        }

        let content = fs::read_to_string(&self.storage_path)
            .map_err(|e| SunriseError::IoError(e.to_string()))?;
        let accounts: Vec<AccountState> = serde_json::from_str(&content)
            .map_err(|e| SunriseError::InvalidJson(e.to_string()))?;

        accounts.into_iter()
            .find(|a| a.membership_id == membership_id)
            .ok_or_else(|| SunriseError::FileNotFound(format!("Account {} not found", membership_id)))
    }

    pub fn save_account(&self, account: &AccountState) -> Result<()> {
        if let Some(parent) = self.storage_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let mut accounts: Vec<AccountState> = if self.storage_path.exists() {
            let content = fs::read_to_string(&self.storage_path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Vec::new()
        };

        accounts.retain(|a| a.membership_id != account.membership_id);
        accounts.push(account.clone());

        let json = serde_json::to_string_pretty(&accounts)
            .map_err(|e| SunriseError::InvalidJson(e.to_string()))?;
        fs::write(&self.storage_path, json)
            .map_err(|e| SunriseError::IoError(e.to_string()))?;

        Ok(())
    }
}