use voter::crypto::blind_rsa;

#[test]
fn blind_sign_finalize_verify_roundtrip() {
    let (pk, sk) = blind_rsa::generate_test_keypair();

    let nonce = b"test-nonce-32-bytes-padded-ok!!!";
    let h_n = blind_rsa::compute_h_n(nonce);
    let h_n_bytes = h_n.as_bytes();

    // Blind
    let (blinding_result, blind_msg_b64) = blind_rsa::blind_nonce(&pk, h_n_bytes).unwrap();
    assert!(!blind_msg_b64.is_empty());

    // Server signs the blind message
    let blind_msg_bytes =
        base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &blind_msg_b64).unwrap();
    let blind_sig: blind_rsa_signatures::BlindSignature = sk.blind_sign(&blind_msg_bytes).unwrap();
    let blind_sig_b64 =
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &blind_sig.0);

    // Finalize (unblind)
    let (sig, msg_randomizer) =
        blind_rsa::finalize_token(&pk, &blind_sig_b64, &blinding_result, h_n_bytes).unwrap();

    // Verify
    blind_rsa::verify_token(&pk, &sig, msg_randomizer, h_n_bytes).unwrap();
}

#[test]
fn tampered_signature_fails_verification() {
    let (pk, sk) = blind_rsa::generate_test_keypair();

    let nonce = b"another-nonce-32-bytes-padded!!";
    let h_n = blind_rsa::compute_h_n(nonce);
    let h_n_bytes = h_n.as_bytes();

    let (blinding_result, blind_msg_b64) = blind_rsa::blind_nonce(&pk, h_n_bytes).unwrap();

    let blind_msg_bytes =
        base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &blind_msg_b64).unwrap();
    let blind_sig: blind_rsa_signatures::BlindSignature = sk.blind_sign(&blind_msg_bytes).unwrap();
    let blind_sig_b64 =
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &blind_sig.0);

    let (mut sig, msg_randomizer) =
        blind_rsa::finalize_token(&pk, &blind_sig_b64, &blinding_result, h_n_bytes).unwrap();

    // Tamper with the signature
    if let Some(byte) = sig.0.first_mut() {
        *byte ^= 0xFF;
    }

    let result = blind_rsa::verify_token(&pk, &sig, msg_randomizer, h_n_bytes);
    assert!(result.is_err());
}

#[test]
fn encode_decode_token_roundtrip() {
    let (pk, sk) = blind_rsa::generate_test_keypair();

    let nonce = b"roundtrip-nonce-32-bytes-pad!!!";
    let h_n = blind_rsa::compute_h_n(nonce);
    let h_n_bytes = h_n.as_bytes();

    let (blinding_result, blind_msg_b64) = blind_rsa::blind_nonce(&pk, h_n_bytes).unwrap();

    let blind_msg_bytes =
        base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &blind_msg_b64).unwrap();
    let blind_sig: blind_rsa_signatures::BlindSignature = sk.blind_sign(&blind_msg_bytes).unwrap();
    let blind_sig_b64 =
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &blind_sig.0);

    let (sig, msg_randomizer) =
        blind_rsa::finalize_token(&pk, &blind_sig_b64, &blinding_result, h_n_bytes).unwrap();

    // Encode
    let token_b64 = blind_rsa::encode_token(&sig, msg_randomizer);
    assert!(!token_b64.is_empty());

    // Decode
    let (sig_bytes, randomizer) = blind_rsa::decode_token(&token_b64).unwrap();
    assert_eq!(sig_bytes, sig.0);
    if msg_randomizer.is_some() {
        assert!(randomizer.is_some());
    }
}

#[test]
fn compute_h_n_is_deterministic() {
    let nonce = b"deterministic-test";
    let h1 = blind_rsa::compute_h_n(nonce);
    let h2 = blind_rsa::compute_h_n(nonce);
    assert_eq!(h1, h2);
    assert_eq!(h1.len(), 64); // SHA-256 hex = 64 chars
}

#[test]
fn compute_h_n_different_inputs_differ() {
    let h1 = blind_rsa::compute_h_n(b"input-a");
    let h2 = blind_rsa::compute_h_n(b"input-b");
    assert_ne!(h1, h2);
}
