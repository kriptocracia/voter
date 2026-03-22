# Tasks: TUI Voting Client

**Input**: Design documents from `/specs/001-tui-voting-client/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: Included — constitution Quality Standards require unit tests on critical paths and integration tests for the full voting flow.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup

**Purpose**: Project initialization and basic structure

- [x] T001 Initialize Rust project with `cargo init --name voter` and configure Cargo.toml with dependencies: ratatui 0.30, crossterm 0.29 (event-stream feature), nostr-sdk 0.44.1 (nip59 feature), blind-rsa-signatures 0.17.1, tokio (full features), age 0.11, serde + serde_json, toml 0.8, sha2 0.10, base64 0.22, secrecy 0.10, directories 5, tracing 0.1, tracing-subscriber 0.3, futures 0.3
- [x] T002 [P] Create project directory structure per plan: src/{crypto/, nostr/, ui/widgets/}, tests/{integration/, unit/}
- [x] T003 [P] Create src/error.rs with project-wide error enum covering config, identity, crypto, nostr, and UI error variants
- [x] T004 [P] Configure clippy (clippy.toml or Cargo.toml lint section) with `-D warnings` and rustfmt.toml for formatting

**Checkpoint**: Project compiles (`cargo check` passes), directory structure in place

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 Implement src/config.rs — parse voter.toml (AppConfig struct with relay list, identity path, theme), default config generation, XDG path resolution via directories crate
- [x] T006 [P] Implement src/app.rs — App struct with AppState enum (per screen), Action enum (Tick, Quit, KeyPress, NetworkResponse, ElectionUpdate), update() dispatch skeleton, action_tx channel
- [x] T007 [P] Implement src/main.rs — terminal setup (enable raw mode, alternate screen), create tokio runtime, spawn crossterm EventStream reader into action channel, main draw/update loop, cleanup on exit
- [x] T008 [P] Implement src/nostr/messages.rs — serde-tagged message types for all EC protocol messages: RegisterRequest, RequestTokenRequest, CastVoteRequest, EcResponse, EcError with error code enum
- [x] T009 [P] Implement src/nostr/events.rs — parse Kind 35000 (Election struct with candidates, status, rules, RSA pub key) and Kind 35001 (ElectionResults struct with tally)
- [x] T010 Implement src/nostr/client.rs — NostrClient wrapper: connect to relays from config, subscribe to Kind 35000/35001 and Kind GiftWrap, send_gift_wrap() helper, unwrap incoming Gift Wrap to EcResponse, feed events into action channel
- [x] T011 [P] Implement src/state.rs — local state persistence: load/save state.json (registrations map, tokens map), encrypt with age when password is set, VoterRegistration and VotingToken structs per data-model.md

### Unit Tests for Foundational

- [x] T012 [P] Unit test for config parsing in tests/config.rs — default config, custom config, missing file creates default
- [x] T013 [P] Unit test for message serialization in tests/messages.rs — round-trip serialize/deserialize for all EC message types, error code parsing

**Checkpoint**: Foundation ready — app launches, connects to relays, receives election events, displays nothing yet (skeleton UI)

---

## Phase 3: User Story 1 — Identity Setup (Priority: P1) MVP

**Goal**: First-time voter creates or imports a Nostr keypair, optionally encrypted with a password

**Independent Test**: Launch with no identity file → complete setup → verify identity file created → app proceeds to election list

- [x] T014 [P] [US1] Implement src/identity.rs — generate_keypair() using nostr-sdk Keys::generate(), import_keypair(hex_secret) → Keys, save_identity(keys, optional password, path) using age encryption or plaintext JSON, load_identity(optional password, path) → Keys, zero secret from memory via secrecy crate
- [x] T015 [US1] Implement src/ui/welcome.rs — Welcome screen: two options (Generate / Import), password input field (optional), render with ratatui, handle KeyPress actions, dispatch IdentityCreated action on completion
- [x] T016 [US1] Implement src/ui/password.rs — Password prompt screen: secure text input (masked), submit dispatches IdentityUnlocked or shows error on wrong password, retry logic
- [x] T017 [US1] Wire identity flow into src/app.rs — on launch check if identity exists: if not → Welcome screen; if encrypted → Password screen; if plaintext → load and transition to ElectionList state
- [x] T018 [P] [US1] Unit test for identity in tests/identity.rs — generate round-trip, encrypt/decrypt with password, wrong password fails, plaintext save/load

**Checkpoint**: App launches, identity setup works end-to-end, transitions to election list (empty)

---

## Phase 4: User Story 2 — Election Discovery and Registration (Priority: P1)

**Goal**: Voter browses elections from Kind 35000 events and registers with a token

**Independent Test**: With identity, connect to relay, see election list, select election, enter token, verify registration success

- [x] T019 [US2] Implement src/ui/election_list.rs — scrollable list of elections (name, status, time, rules, candidate count), vim-style navigation (j/k/arrows), Enter to select, 's' for settings, 'q' to quit, status badges (Open/InProgress/Finished)
- [x] T020 [US2] Implement src/ui/election_detail.rs — display election name, status, candidates, rules, time bounds; registration token input field (shown if not registered); "Request Token" button (shown if registered, no token); "Vote" button (shown if has token); registration status indicator
- [x] T021 [US2] Implement registration flow in src/app.rs — on token submit: build RegisterRequest, send via NostrClient::send_gift_wrap(), handle EcResponse (ok → save VoterRegistration to state, error → display error with retry)
- [x] T022 [US2] Wire election updates into app — ElectionUpdate action feeds parsed Kind 35000 data into app state, election list auto-refreshes, election detail updates status in real-time
- [x] T023 [P] [US2] Integration test stubs in tests/registration.rs — #[ignore] tests for register with valid/invalid token and duplicate registration (requires running EC daemon)

**Checkpoint**: Election list populates from relays, registration flow works, election detail shows registered status

---

## Phase 5: User Story 3 — Anonymous Token Request (Priority: P1)

**Goal**: Registered voter obtains an anonymous voting token via blind RSA protocol

**Independent Test**: As registered voter, request token → verify token stored locally → verify token validates against election RSA public key

- [x] T024 [P] [US3] Implement src/crypto/blind_rsa.rs — blind_nonce(rsa_pub_key, nonce) → BlindingResult, finalize_token(rsa_pub_key, blind_sig, blinding_result, nonce) → (Signature, MessageRandomizer), verify_token(rsa_pub_key, signature, randomizer, h_n) → bool, encode_token(signature, randomizer) → base64 string, decode_token(base64) → (Signature, MessageRandomizer)
- [x] T025 [P] [US3] Implement src/crypto/token.rs — generate_nonce() → 32 random bytes, compute_h_n(nonce) → SHA-256 hex, VotingToken struct with nonce/h_n/signature/randomizer/consumed fields, serialize/deserialize for state persistence
- [x] T026 [US3] Implement token request flow in src/app.rs — on "Request Token" press: generate nonce, blind h_n, build RequestTokenRequest, send via Gift Wrap, handle response (ok → finalize + verify + save VotingToken to state, error → display), progress indicator updates per step
- [x] T027 [US3] Add token request progress to src/ui/election_detail.rs — show step-by-step progress (Generating nonce → Blinding → Sending → Receiving → Unblinding → Done), disable button during request
- [x] T028 [P] [US3] Unit test in tests/blind_rsa.rs — generate keypair, blind/sign/finalize roundtrip, verify succeeds, tampered signature fails, encode/decode token roundtrip (NOTE: this test generates its own RSA keypair for isolated testing)
- [x] T029 [P] [US3] Integration test stub in tests/integration_blind_rsa.rs — #[ignore] test for full blind RSA roundtrip with EC (requires running EC daemon)

**Checkpoint**: Token request works, token stored locally, can verify token is valid

---

## Phase 6: User Story 4 — Cast Anonymous Vote (Priority: P1)

**Goal**: Voter casts an anonymous ballot from a throwaway keypair

**Independent Test**: With valid token, select candidate(s), confirm, submit → vote accepted, token consumed, throwaway key discarded

- [x] T030 [US4] Implement src/ui/vote.rs — ballot screen with two modes: Plurality (radio-button candidate selection, j/k navigate, Enter/Space select) and STV (ranked list, Enter to add rank, number keys for position, 'd' to remove), Submit button, Esc to go back
- [x] T031 [US4] Implement src/ui/widgets/confirm_dialog.rs — modal overlay showing election name + selected candidates, "Confirm" and "Go Back" buttons, Enter to confirm, Esc to go back
- [x] T032 [US4] Implement vote casting flow in src/app.rs — on confirm: generate throwaway Keys, build CastVoteRequest (election_id, candidate_ids, h_n from token, encoded token), create separate NostrClient connection with throwaway keys, send via Gift Wrap, handle response (ok → mark token consumed + save state, error → preserve token + display error), zeroize throwaway secret key
- [x] T033 [US4] Update src/ui/election_detail.rs — show "Voted" badge when token is consumed, hide "Vote" button, show "Request Token" only if no token exists
- [x] T034 [P] [US4] Integration test stub in tests/full_flow.rs — #[ignore] test for end-to-end voting flow (requires running EC daemon)

**Checkpoint**: Full voting flow works end-to-end. This is the MVP.

---

## Phase 7: User Story 5 — View Election Results (Priority: P2)

**Goal**: Voter views published results after election ends

**Independent Test**: After election finishes, navigate to it, verify results displayed with candidate names and tallies

- [x] T035 [US5] Implement src/ui/results.rs — results display screen: election name, candidate names with vote tallies (sorted by votes), winner highlighted, Esc to go back to election list
- [x] T036 [US5] Wire Kind 35001 events into app — when results event arrives, store parsed ElectionResults in app state; when election status = Finished and results available, show "View Results" in election detail
- [x] T037 [US5] Update src/ui/election_detail.rs — add "View Results" button when election is Finished and results are available

**Checkpoint**: Results display works for finished elections

---

## Phase 8: User Story 6 — Settings and Configuration (Priority: P3)

**Goal**: Voter configures relays, manages identity, customizes theme

**Independent Test**: Open settings, modify relay list, save, verify new relays are used

- [x] T038 [US6] Implement src/ui/settings.rs — settings screen: editable relay list (add/remove URLs), theme toggle (dark/light), identity export option, save button that writes voter.toml and reconnects relays
- [x] T039 [US6] Implement identity export in src/identity.rs — export_public_key() → hex string, export_secret_key(password) → hex string (requires password if encrypted), display in settings or save to file
- [x] T040 [US6] Wire settings into app — 's' key from election list opens settings, Esc returns, on save: reload config, reconnect NostrClient with updated relay list, apply theme change immediately

**Checkpoint**: Settings fully functional, relay changes take effect

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T041 [P] Implement src/ui/help.rs — help overlay toggled by '?' key, shows all keyboard shortcuts per screen, renders as semi-transparent overlay on current screen
- [x] T042 [P] Implement src/ui/widgets/status_bar.rs — bottom status bar showing relay connection status (connected/disconnected/reconnecting), current screen name, key hints
- [x] T043 Add graceful network error handling — Nostr client wired into main loop with NostrAction dispatch, connection status in status bar, exponential backoff reconnection, local state preserved
- [x] T044 Add terminal resize handling in src/main.rs — respond to crossterm Resize events, trigger full redraw via Action::Resize
- [x] T045 [P] Add tracing instrumentation — structured logging via tracing crate with env-filter for debug builds, ensure no sensitive data (tokens, nonces, keys) is ever logged per constitution Must Not rules
- [x] T046 Quickstart.md validation — build succeeds, app launches, identity generation works, election list screen displays

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **US1 Identity (Phase 3)**: Depends on Foundational — BLOCKS US2, US3, US4 (need identity)
- **US2 Elections (Phase 4)**: Depends on US1 (need identity to register)
- **US3 Token (Phase 5)**: Depends on US2 (need registration)
- **US4 Vote (Phase 6)**: Depends on US3 (need token)
- **US5 Results (Phase 7)**: Depends on Foundational only (can start after Phase 2 in parallel with US1-US4)
- **US6 Settings (Phase 8)**: Depends on Foundational only (can start after Phase 2 in parallel)
- **Polish (Phase 9)**: Depends on US4 completion (MVP must work first)

### Within Each User Story

- Models/types before services
- Services before UI
- UI before integration wiring
- Tests can run in parallel with each other

### Parallel Opportunities

```bash
# Phase 1 — all setup tasks T002-T004 in parallel after T001
# Phase 2 — T006, T007, T008, T009, T011 in parallel; T010 after T008+T009
# Phase 3 (US1) — T014 and T018 in parallel; T015 and T016 in parallel after T014
# Phase 5 (US3) — T024, T025, T028, T029 in parallel
# Phase 7 (US5) and Phase 8 (US6) can run in parallel with the core flow
# Phase 9 — T041, T042, T045 in parallel
```

---

## Implementation Strategy

### MVP First (Through User Story 4)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: US1 — Identity Setup
4. Complete Phase 4: US2 — Election Discovery + Registration
5. Complete Phase 5: US3 — Anonymous Token Request
6. Complete Phase 6: US4 — Cast Anonymous Vote
7. **STOP and VALIDATE**: Full voting flow works end-to-end with EC daemon

### Incremental Delivery

8. Add Phase 7: US5 — Results viewing
9. Add Phase 8: US6 — Settings
10. Add Phase 9: Polish (help overlay, status bar, error handling, resize)

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- US1→US2→US3→US4 is a sequential chain (each builds on previous)
- US5 and US6 are independent and can be done in any order after foundational
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
