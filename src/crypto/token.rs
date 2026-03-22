#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::crypto::blind_rsa;
use crate::error::{Result, VoterError};

/// A voting token obtained via the blind RSA signing protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingToken {
    /// The random 32-byte nonce (base64-encoded). Never sent to EC.
    pub nonce_b64: String,
    /// SHA-256(nonce) as hex. Sent as h_n in cast-vote.
    pub h_n: String,
    /// The finalized (unblinded) RSA signature (base64-encoded).
    pub signature_b64: String,
    /// The 32-byte message randomizer (base64-encoded), if present.
    pub randomizer_b64: Option<String>,
    /// Whether this token has been used to cast a vote.
    pub consumed: bool,
}

/// Generate a cryptographically random 32-byte nonce.
pub fn generate_nonce() -> Result<[u8; 32]> {
    let mut nonce = [0u8; 32];
    getrandom::fill(&mut nonce)
        .map_err(|e| VoterError::Crypto(format!("random nonce generation failed: {e}")))?;
    Ok(nonce)
}

/// Compute the nonce hash (h_n) as a hex-encoded SHA-256 digest.
pub fn compute_h_n(nonce: &[u8]) -> String {
    blind_rsa::compute_h_n_hex(nonce)
}
