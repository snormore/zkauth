use std::fmt::Debug;

use num_bigint::BigInt;

pub mod discrete_logarithm;
pub mod elliptic_curve;

pub trait Prover: Sync + Send + Debug {
    fn generate_registration_x(&self) -> BigInt;
    fn compute_registration_x(&self, password: String) -> BigInt;
    fn compute_registration_y1y2(&self, x: BigInt) -> (BigInt, BigInt);
    fn generate_challenge_k(&self) -> BigInt;
    fn compute_challenge_commitment_r1r2(&self, k: BigInt) -> (BigInt, BigInt);
    fn compute_challenge_response_s(&self, x: BigInt, k: BigInt, c: BigInt) -> BigInt;
}

pub trait Verifier: Sync + Send {
    fn generate_challenge_c(&self) -> BigInt;
    fn compute_verification_r1r2(
        &self,
        y1: BigInt,
        y2: BigInt,
        c: BigInt,
        s: BigInt,
    ) -> (BigInt, BigInt);
}
