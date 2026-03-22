//! End-to-end integration test: register → request token → cast vote.
//! Requires a running EC daemon — run with `RUN_EC_INTEGRATION=1 cargo test -- --ignored`.

#[tokio::test]
#[ignore = "requires running EC daemon"]
async fn full_voting_flow() {
    if std::env::var("RUN_EC_INTEGRATION").is_err() {
        eprintln!("skipped: set RUN_EC_INTEGRATION=1 to run");
        return;
    }

    // TODO: Register → request blind token → cast anonymous vote → verify accepted
    eprintln!("full_voting_flow: test scaffold — implement with live EC");
}
