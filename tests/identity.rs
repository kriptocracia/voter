use voter::identity;

#[test]
fn generate_and_save_plaintext() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("identity.json");

    let keys = identity::generate_keypair();
    identity::save_identity(&keys, None, &path).unwrap();

    assert!(path.exists());
    let loaded = identity::load_identity(None, &path).unwrap();
    assert_eq!(
        identity::export_public_key(&keys),
        identity::export_public_key(&loaded)
    );
}

#[test]
fn encrypt_decrypt_with_password() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("identity.json");

    let keys = identity::generate_keypair();
    identity::save_identity(&keys, Some("testpass"), &path).unwrap();

    let encrypted_path = path.with_extension("age");
    assert!(encrypted_path.exists());

    let loaded = identity::load_identity(Some("testpass"), &path).unwrap();
    assert_eq!(
        identity::export_public_key(&keys),
        identity::export_public_key(&loaded)
    );
}

#[test]
fn wrong_password_fails() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("identity.json");

    let keys = identity::generate_keypair();
    identity::save_identity(&keys, Some("correct"), &path).unwrap();

    let result = identity::load_identity(Some("wrong"), &path);
    assert!(result.is_err());
}

#[test]
fn identity_exists_plaintext() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("identity.json");

    assert!(!identity::identity_exists(&path));

    let keys = identity::generate_keypair();
    identity::save_identity(&keys, None, &path).unwrap();

    assert!(identity::identity_exists(&path));
    assert!(!identity::identity_is_encrypted(&path));
}

#[test]
fn identity_exists_encrypted() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("identity.json");

    let keys = identity::generate_keypair();
    identity::save_identity(&keys, Some("pass"), &path).unwrap();

    assert!(identity::identity_exists(&path));
    assert!(identity::identity_is_encrypted(&path));
}

#[test]
fn load_nonexistent_fails() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("nonexistent.json");

    let result = identity::load_identity(None, &path);
    assert!(result.is_err());
}

#[test]
fn export_public_key_is_hex() {
    let keys = identity::generate_keypair();
    let pubkey = identity::export_public_key(&keys);
    assert_eq!(pubkey.len(), 64);
    assert!(pubkey.chars().all(|c| c.is_ascii_hexdigit()));
}
