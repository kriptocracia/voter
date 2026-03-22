//! Integration tests for voter registration with the EC.
//! Requires a running EC daemon — run with `RUN_EC_INTEGRATION=1 cargo test -- --ignored`.

fn skip_if_no_ec() -> bool {
    if std::env::var("RUN_EC_INTEGRATION").is_err() {
        eprintln!("skipped: set RUN_EC_INTEGRATION=1 to run");
        return true;
    }
    false
}

#[tokio::test]
#[ignore = "requires running EC daemon"]
async fn register_with_valid_token() {
    if skip_if_no_ec() {
        return;
    }

    // TODO: Connect to EC, register with valid token, assert success
    eprintln!("register_with_valid_token: test scaffold — implement with live EC");
}

#[tokio::test]
#[ignore = "requires running EC daemon"]
async fn register_with_invalid_token() {
    if skip_if_no_ec() {
        return;
    }

    // TODO: Connect to EC, register with invalid token, assert EcErrorCode::InvalidToken
    eprintln!("register_with_invalid_token: test scaffold — implement with live EC");
}

#[tokio::test]
#[ignore = "requires running EC daemon"]
async fn register_twice_fails() {
    if skip_if_no_ec() {
        return;
    }

    // TODO: Register once, register again, assert EcErrorCode::AlreadyRegistered
    eprintln!("register_twice_fails: test scaffold — implement with live EC");
}
