use crate::{Prover, Verifier};
use anyhow::Result;
use num_traits::One;

use super::{
    configuration::DiscreteLogarithmConfiguration, prover::DiscreteLogarithmProver,
    verifier::DiscreteLogarithmVerifier,
};

pub fn test_prover_and_verifier() -> (DiscreteLogarithmProver, DiscreteLogarithmVerifier) {
    let config = DiscreteLogarithmConfiguration::generate(16);
    let prover = DiscreteLogarithmProver::new(config.clone());
    let verifier = DiscreteLogarithmVerifier::new(config);
    (prover, verifier)
}

pub fn test_prover() -> DiscreteLogarithmProver {
    let (prover, _) = test_prover_and_verifier();
    prover
}

pub fn test_verifier() -> DiscreteLogarithmVerifier {
    let (_, verifier) = test_prover_and_verifier();
    verifier
}

#[test]
fn verify_succeeds_with_given_password() -> Result<()> {
    let (prover, verifier) = test_prover_and_verifier();

    let x = prover.compute_registration_x("password".to_string());
    let (y1, y2) = prover.compute_registration_y1y2(x.clone()).unwrap();
    let c = verifier.generate_challenge_c();
    let k = prover.generate_challenge_k();
    let (r1, r2) = prover.compute_challenge_commitment_r1r2(k.clone()).unwrap();
    let s = prover
        .compute_challenge_response_s(x, k, c.clone())
        .unwrap();
    let (r1_prime, r2_prime) = verifier.compute_verification_r1r2(y1, y2, c, s).unwrap();
    assert_eq!(r1_prime, r1);
    assert_eq!(r2_prime, r2);

    Ok(())
}

#[test]
fn verify_succeeds_with_generated_x() -> Result<()> {
    let (prover, verifier) = test_prover_and_verifier();

    let x = prover.generate_registration_x();
    let (y1, y2) = prover.compute_registration_y1y2(x.clone()).unwrap();
    let c = verifier.generate_challenge_c();
    let k = prover.generate_challenge_k();
    let (r1, r2) = prover.compute_challenge_commitment_r1r2(k.clone()).unwrap();
    let s = prover
        .compute_challenge_response_s(x, k, c.clone())
        .unwrap();
    let (r1_prime, r2_prime) = verifier.compute_verification_r1r2(y1, y2, c, s).unwrap();
    assert_eq!(r1_prime, r1);
    assert_eq!(r2_prime, r2);

    Ok(())
}

#[test]
fn verify_fails_with_incorrect_y1() -> Result<()> {
    let (prover, verifier) = test_prover_and_verifier();

    let x = prover.compute_registration_x("password".to_string());
    let (mut y1, y2) = prover.compute_registration_y1y2(x.clone()).unwrap();
    y1 = y1 + One::one();
    let c = verifier.generate_challenge_c();
    let k = prover.generate_challenge_k();
    let (r1, r2) = prover.compute_challenge_commitment_r1r2(k.clone()).unwrap();
    let s = prover
        .compute_challenge_response_s(x, k, c.clone())
        .unwrap();
    let (r1_prime, r2_prime) = verifier.compute_verification_r1r2(y1, y2, c, s).unwrap();
    assert_ne!(r1_prime, r1);
    assert_eq!(r2_prime, r2);

    Ok(())
}

#[test]
fn verify_fails_with_incorrect_y2() -> Result<()> {
    let (prover, verifier) = test_prover_and_verifier();

    let x = prover.compute_registration_x("password".to_string());
    let (y1, mut y2) = prover.compute_registration_y1y2(x.clone()).unwrap();
    y2 = y2 + One::one();
    let c = verifier.generate_challenge_c();
    let k = prover.generate_challenge_k();
    let (r1, r2) = prover.compute_challenge_commitment_r1r2(k.clone()).unwrap();
    let s = prover
        .compute_challenge_response_s(x, k, c.clone())
        .unwrap();
    let (r1_prime, r2_prime) = verifier.compute_verification_r1r2(y1, y2, c, s).unwrap();
    assert_eq!(r1_prime, r1);
    assert_ne!(r2_prime, r2);

    Ok(())
}
