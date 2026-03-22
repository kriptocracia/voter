//! Integration tests for voter registration with the EC.
//! Requires a running EC daemon — run with `cargo test -- --ignored`.

#[tokio::test]
#[ignore = "requires running EC daemon"]
async fn register_with_valid_token() {
    // TODO: Connect to EC, register with valid token, assert success
    todo!("requires running EC daemon");
}

#[tokio::test]
#[ignore = "requires running EC daemon"]
async fn register_with_invalid_token() {
    // TODO: Connect to EC, register with invalid token, assert INVALID_TOKEN error
    todo!("requires running EC daemon");
}

#[tokio::test]
#[ignore = "requires running EC daemon"]
async fn register_twice_fails() {
    // TODO: Register once, register again, assert ALREADY_REGISTERED error
    todo!("requires running EC daemon");
}
