//! Integration tests for blind RSA roundtrip with the EC.
//! Requires a running EC daemon — run with `RUN_EC_INTEGRATION=1 cargo test -- --ignored`.

#[tokio::test]
#[ignore = "requires running EC daemon"]
async fn blind_rsa_full_roundtrip_with_ec() {
    if std::env::var("RUN_EC_INTEGRATION").is_err() {
        eprintln!("skipped: set RUN_EC_INTEGRATION=1 to run");
        return;
    }

    // TODO: Register, request token via blind RSA, finalize, verify signature
    eprintln!("blind_rsa_full_roundtrip_with_ec: test scaffold — implement with live EC");
}
