use num_bigint::{BigInt, BigUint, RandomBits};
use rand::Rng;

/// The configuration module.
pub mod configuration;

/// The prover module.
pub mod prover;

/// The verifier module.
pub mod verifier;

/// Generates a random scalar.
fn generate_random_scalar() -> BigInt {
    let mut rng = rand::thread_rng();
    let num: BigUint = rng.sample(RandomBits::new(32));
    let signed_num: BigInt = num.clone().into();
    signed_num
}
