use serde::{Deserialize, Serialize};

/// An election parsed from a Kind 35000 Nostr event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Election {
    pub election_id: String,
    pub name: String,
    pub start_time: String,
    pub end_time: String,
    pub status: ElectionStatus,
    pub rules_id: String,
    /// Base64 DER-encoded RSA public key for blind signing.
    pub rsa_pub_key: String,
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ElectionStatus {
    Open,
    InProgress,
    Finished,
    Cancelled,
}

impl std::fmt::Display for ElectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "Open"),
            Self::InProgress => write!(f, "In Progress"),
            Self::Finished => write!(f, "Finished"),
            Self::Cancelled => write!(f, "Cancelled"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub id: u32,
    pub name: String,
}

/// Election results parsed from a Kind 35001 Nostr event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectionResults {
    pub election_id: String,
    pub elected: Vec<u32>,
    pub tally: Vec<TallyEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TallyEntry {
    pub candidate_id: u32,
    pub votes: u64,
}
