//! End-to-end integration test: register → request token → cast vote.
//! Requires a running EC daemon — run with `cargo test -- --ignored`.

#[tokio::test]
#[ignore = "requires running EC daemon"]
async fn full_voting_flow() {
    // TODO: Register → request blind token → cast anonymous vote → verify accepted
    todo!("requires running EC daemon");
}
