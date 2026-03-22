# Contributing

## Getting Started

1. Fork and clone the repository
2. Install the Rust stable toolchain
3. Run `cargo build` to verify everything compiles
4. Run `cargo test` to verify tests pass

## Development Workflow

### Before Submitting

All of these must pass:

```bash
cargo fmt --check        # Code formatting
cargo clippy -- -D warnings  # Lints (zero warnings)
cargo test               # All tests
```

### Code Style

- Follow standard Rust conventions and `rustfmt` defaults
- No `unsafe` code — the project denies it at the compiler level
- No custom cryptography — use audited libraries only
- Fail secure: reject invalid inputs rather than attempting recovery

### Privacy Requirements

Any changes to the voting protocol or cryptographic operations must preserve:

- **Unlinkability**: the voter's identity keypair must never appear in the same message as their vote
- **Token blindness**: the EC must never see the unblinded nonce or final token before it is used
- **No telemetry**: the client must not make network calls except to configured Nostr relays

### Testing

- Add unit tests for new functionality
- Integration tests requiring an EC daemon should be gated behind `#[ignore]` and the `RUN_EC_INTEGRATION` environment variable
- Use `tempfile` for tests that write to disk

## Pull Requests

- Keep PRs focused on a single change
- Reference any related issues
- Describe what changed and why in the PR description
