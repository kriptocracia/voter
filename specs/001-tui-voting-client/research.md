# Research: TUI Voting Client

**Feature Branch**: `001-tui-voting-client`
**Date**: 2026-03-21

## R1: Nostr SDK — NIP-59 Gift Wrap API

**Decision**: Use `nostr-sdk 0.44.1` with the `nip59` feature flag.

**Rationale**: The EC daemon already uses this exact version. The SDK
exposes `Client::gift_wrap()` for sending and `Client::unwrap_gift_wrap()`
for receiving, which map directly to the EC protocol.

**Key API surface**:
- `client.gift_wrap(receiver_pubkey, rumor_event, [])` — sends a
  Gift Wrap message (Kind 1059)
- `client.unwrap_gift_wrap(&event)` — returns `GiftWrap { sender, rumor }`
  where `rumor.content` contains the JSON payload
- Subscribe to election events via `Filter::new().kinds([Kind::Custom(35_000), Kind::Custom(35_001)])`
- Subscribe to Gift Wrap replies via `Filter::new().kind(Kind::GiftWrap).pubkey(my_pubkey)`
- Process with `client.handle_notifications()` callback

**Alternatives considered**: Raw `nostr` crate without SDK — rejected
because it would require reimplementing relay pool management and
subscription handling.

## R2: Blind RSA Signatures — blind-rsa-signatures 0.17.1

**Decision**: Use `blind-rsa-signatures 0.17.1` with `Sha384 + PSS + Randomized`
to match the EC's implementation.

**Rationale**: The EC uses this exact crate and parameterization. Using
the same types ensures wire-compatible signatures.

**Key types**:
```
PublicKey<Sha384, PSS, Randomized>  — election public key (from Kind 35000)
BlindingResult                      — holds blind message + secret for finalize
BlindSignature                      — EC's response to request-token
Signature                           — finalized (unblinded) signature
MessageRandomizer                   — 32-byte randomizer (part of token)
```

**Client-side flow**:
1. `pk.blind(&mut rng, msg)` → `BlindingResult` (send `.blind_msg` to EC)
2. Receive `BlindSignature` from EC
3. `pk.finalize(&blind_sig, &blinding_result, msg)` → `Signature`
4. Store `base64(signature_bytes ++ msg_randomizer_32bytes)` as the token
5. Verify locally: `pk.verify(&sig, Some(randomizer), msg)`

**Token format**: The EC expects `base64(signature_bytes ++ 32-byte randomizer)`.
The message being signed is `h_n = SHA-256(nonce)` where `nonce` is a
random 32-byte value.

**Alternatives considered**: None — must match the EC exactly.

## R3: TUI Architecture — ratatui + crossterm + tokio

**Decision**: Elm Architecture (TEA) pattern with channel-based action
dispatch.

**Rationale**: This is the recommended pattern for async ratatui apps.
All events (keyboard, network, timers) funnel through a single
`tokio::sync::mpsc` channel, keeping the update/view cycle simple and
predictable.

**Pattern**:
- `Action` enum: `Tick`, `Quit`, `KeyPress(KeyCode)`, `NetworkResponse(...)`
- Spawned task reads `crossterm::event::EventStream` and sends `Action`s
- Spawned tasks for network calls send results as `Action::NetworkResponse`
- Main loop: `terminal.draw(|f| app.view(f))` then `action_rx.recv().await`
- `app.update(action)` mutates state, optionally spawns new async tasks

**Dependencies**:
- `ratatui = "0.30"`
- `crossterm = { version = "0.29", features = ["event-stream"] }`

**Alternatives considered**: `tui-realm` (too heavy), raw crossterm
without ratatui (too low-level).

## R4: Identity Storage — Encrypted Keypair Persistence

**Decision**: Use the `age` crate for passphrase-based encryption of
the Nostr secret key.

**Rationale**: `age` uses scrypt for KDF internally, is well-audited,
and wraps encryption concerns in a single API. Avoids rolling custom
crypto per constitution principle II (Cryptographic Integrity).

**Storage path**: `~/.config/voter/identity.age` (encrypted) or
`~/.config/voter/identity.json` (unencrypted, if no password set).

**Format (unencrypted)**:
```json
{
  "secret_key": "<hex-encoded 32-byte secret key>"
}
```

**Encrypted format**: Standard `age` file format (binary, scrypt recipient).

**Alternatives considered**: `chacha20poly1305` + `argon2` (more manual),
NIP-49 encrypted key format (less established tooling in Rust).

## R5: EC Protocol Message Formats

**Decision**: Use the exact JSON message formats defined by the EC.

**Inbound (voter → EC) actions**:
- `register`: `{ "action": "register", "election_id": "...", "registration_token": "..." }`
- `request-token`: `{ "action": "request-token", "election_id": "...", "blinded_nonce": "<base64>" }`
- `cast-vote`: `{ "action": "cast-vote", "election_id": "...", "candidate_ids": [...], "h_n": "<hex>", "token": "<base64>" }`

**Outbound (EC → voter) responses**:
- Success: `{ "status": "ok", "action": "<action-confirmed>" }`
- Token issued: `{ "status": "ok", "action": "token-issued", "blind_signature": "<base64>" }`
- Error: `{ "status": "error", "code": "<ERROR_CODE>", "message": "..." }`

**Error codes**: `ELECTION_NOT_FOUND`, `ELECTION_CLOSED`, `INVALID_TOKEN`,
`ALREADY_REGISTERED`, `NOT_AUTHORIZED`, `ALREADY_ISSUED`,
`NONCE_ALREADY_USED`, `INVALID_CANDIDATE`, `BALLOT_INVALID`,
`UNKNOWN_RULES`, `INTERNAL_ERROR`, `INVALID_MESSAGE`.

**Public Nostr events**:
- Kind 35000: Election announcement (addressable, d-tagged by election_id).
  Content: `{ election_id, name, start_time, end_time, status, rules_id, rsa_pub_key, candidates }`.
- Kind 35001: Election results (addressable, d-tagged by election_id).
  Content: `{ election_id, elected, tally, count_sheet }`.

**Alternatives considered**: None — protocol is defined by the EC.
