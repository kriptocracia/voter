use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::crypto::token::VotingToken;
use crate::error::Result;

/// Local persistent state (registrations and voting tokens).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppState {
    #[serde(default)]
    pub registrations: HashMap<String, VoterRegistration>,
    #[serde(default)]
    pub tokens: HashMap<String, VotingToken>,
}

/// Registration status for a specific election.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterRegistration {
    pub registered: bool,
    pub registered_at: String,
}

impl AppState {
    /// Load state from disk. Returns default state if file doesn't exist.
    pub fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            let contents = std::fs::read_to_string(path)?;
            let state: AppState = serde_json::from_str(&contents)?;
            Ok(state)
        } else {
            Ok(AppState::default())
        }
    }

    /// Save state to disk, creating parent directories if needed.
    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Check if the voter is registered for an election.
    pub fn is_registered(&self, election_id: &str) -> bool {
        self.registrations
            .get(election_id)
            .is_some_and(|r| r.registered)
    }

    /// Get a token for an election, if one exists and is not consumed.
    pub fn get_active_token(&self, election_id: &str) -> Option<&VotingToken> {
        self.tokens.get(election_id).filter(|t| !t.consumed)
    }

    /// Check if the voter has already voted in an election.
    pub fn has_voted(&self, election_id: &str) -> bool {
        self.tokens.get(election_id).is_some_and(|t| t.consumed)
    }
}
