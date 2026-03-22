use serde::{Deserialize, Serialize};

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
