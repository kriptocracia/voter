//! Integration tests for blind RSA roundtrip with the EC.
//! Requires a running EC daemon — run with `cargo test -- --ignored`.

#[tokio::test]
#[ignore = "requires running EC daemon"]
async fn blind_rsa_full_roundtrip_with_ec() {
    // TODO: Register, request token via blind RSA, finalize, verify signature
    todo!("requires running EC daemon");
}
