use num_bigint::{BigInt, BigUint, RandomBits};
use rand::Rng;

pub mod configuration;
pub mod prover;
pub mod verifier;

fn generate_random_scalar() -> BigInt {
    let mut rng = rand::thread_rng();
    let num: BigUint = rng.sample(RandomBits::new(32));
    let signed_num: BigInt = num.clone().into();
    signed_num
}
