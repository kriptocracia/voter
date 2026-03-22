#[cfg(test)]
pub fn generate_test_keypair() -> (
    blind_rsa_signatures::PublicKey<
        blind_rsa_signatures::Sha384,
        blind_rsa_signatures::PSS,
        blind_rsa_signatures::Randomized,
    >,
    blind_rsa_signatures::SecretKey<
        blind_rsa_signatures::Sha384,
        blind_rsa_signatures::PSS,
        blind_rsa_signatures::Randomized,
    >,
) {
    let kp = blind_rsa_signatures::KeyPair::<
        blind_rsa_signatures::Sha384,
        blind_rsa_signatures::PSS,
        blind_rsa_signatures::Randomized,
    >::generate(&mut blind_rsa_signatures::DefaultRng, 2048)
    .expect("keygen");
    (kp.pk, kp.sk)
}
