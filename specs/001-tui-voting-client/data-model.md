# Data Model: TUI Voting Client

**Feature Branch**: `001-tui-voting-client`
**Date**: 2026-03-21

## Entities

### VoterIdentity

The voter's persistent Nostr keypair. One per client installation.

| Field        | Type       | Description                              |
|--------------|------------|------------------------------------------|
| public_key   | 32 bytes   | Nostr public key (hex-encoded on disk)   |
| secret_key   | 32 bytes   | Nostr secret key (encrypted or plaintext)|
| encrypted    | boolean    | Whether the key file is password-encrypted|
| created_at   | timestamp  | When the identity was created            |

**Storage**: `~/.config/voter/identity.age` (encrypted) or
`~/.config/voter/identity.json` (plaintext).

**Constraints**:
- Secret key MUST be zeroed from memory after use (secrecy crate).
- If `encrypted = true`, secret key is wrapped with `age` passphrase encryption.

---

### Election

An electoral event discovered from Kind 35000 Nostr events.

| Field        | Type       | Description                              |
|--------------|------------|------------------------------------------|
| election_id  | string     | Unique identifier (from EC)              |
| name         | string     | Human-readable election name             |
| status       | enum       | Open, InProgress, Finished, Cancelled    |
| start_time   | timestamp  | When voting opens                        |
| end_time     | timestamp  | When voting closes                       |
| rules_id     | string     | Voting rules identifier (plurality, stv) |
| rsa_pub_key  | bytes      | Election's RSA public key (DER-encoded)  |
| candidates   | list       | List of Candidate entities               |

**Source**: Parsed from Kind 35000 Nostr event content.

**State transitions**: `Open → InProgress → Finished` (or `Cancelled`
at any point). Transitions are driven by the EC scheduler and
reflected in updated Kind 35000 events.

---

### Candidate

A candidate within an election.

| Field        | Type       | Description                              |
|--------------|------------|------------------------------------------|
| candidate_id | integer    | Unique ID within the election            |
| name         | string     | Candidate display name                   |

**Relationship**: Belongs to one Election.

---

### VoterRegistration

Tracks the voter's registration status per election. Local state only.

| Field        | Type       | Description                              |
|--------------|------------|------------------------------------------|
| election_id  | string     | References Election                      |
| registered   | boolean    | Whether registration was successful      |
| registered_at| timestamp  | When registration was confirmed          |

**Storage**: Local state file `~/.config/voter/state.json`.

---

### VotingToken

An anonymous credential obtained via blind RSA signing. One per election.

| Field        | Type       | Description                              |
|--------------|------------|------------------------------------------|
| election_id  | string     | References Election                      |
| nonce        | 32 bytes   | Random nonce (secret, never sent to EC)  |
| h_n          | 32 bytes   | SHA-256(nonce) — used in vote message    |
| signature    | bytes      | Unblinded RSA signature                  |
| randomizer   | 32 bytes   | Message randomizer from blind signing    |
| consumed     | boolean    | Whether the token has been used to vote  |

**Storage**: Local state file `~/.config/voter/state.json`.
Token bytes encrypted if password is set.

**Constraints**:
- Nonce MUST be cryptographically random (32 bytes from OS CSPRNG).
- Nonce MUST NEVER be transmitted to the EC (only h_n is sent).
- Token is encoded as `base64(signature ++ randomizer)` for the
  cast-vote message.
- Once `consumed = true`, the token cannot be reused.

---

### Ballot

The voter's candidate selection, constructed at vote time.

| Field         | Type       | Description                              |
|---------------|------------|------------------------------------------|
| election_id   | string     | References Election                      |
| candidate_ids | list<int>  | Single element (plurality) or ranked (STV)|

**Constraints**:
- Plurality: exactly one candidate_id.
- STV: ordered list of candidate_ids, no duplicates.
- Candidate IDs must exist in the election's candidate list.

**Not persisted** — constructed in memory and discarded after submission.

---

### AppConfig

User configuration loaded from TOML file.

| Field        | Type       | Description                              |
|--------------|------------|------------------------------------------|
| relays       | list<url>  | Nostr relay WebSocket URLs               |
| identity_path| path       | Path to identity file                    |
| theme        | enum       | dark, light                              |

**Storage**: `~/.config/voter/voter.toml`.

**Defaults**:
```toml
[nostr]
relays = ["wss://relay.mostro.network", "wss://nos.lol"]

[identity]
path = "~/.config/voter/identity.json"

[ui]
theme = "dark"
```

## Relationships

```text
VoterIdentity (1) ──────── (*) VoterRegistration
                                    │
Election (1) ──── (*) Candidate     │
    │                               │
    └──────── (1) VoterRegistration ┘
    │
    └──────── (0..1) VotingToken ──── (0..1) Ballot (transient)
```

## State File Format

`~/.config/voter/state.json`:
```json
{
  "registrations": {
    "<election_id>": {
      "registered": true,
      "registered_at": "2026-03-21T10:00:00Z"
    }
  },
  "tokens": {
    "<election_id>": {
      "nonce": "<base64>",
      "h_n": "<hex>",
      "signature": "<base64>",
      "randomizer": "<base64>",
      "consumed": false
    }
  }
}
```

If a password is set, this file is encrypted with `age` using the
same passphrase as the identity file.
