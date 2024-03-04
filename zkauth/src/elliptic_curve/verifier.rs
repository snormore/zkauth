use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::RistrettoPoint;
use curve25519_dalek::Scalar;
use num_bigint::BigInt;
use sha2::{Digest, Sha512};

use super::operations::EllipticCurveOperations;
use super::scalar_to_bigint;
use super::{bigint_to_ristretto_point, bigint_to_scalar, ristretto_point_to_bigint};
use crate::Operations;
use crate::Verifier;

pub struct EllipticCurveVerifier {
    operations: EllipticCurveOperations,
}

impl EllipticCurveVerifier {
    pub fn new(g: RistrettoPoint, h: RistrettoPoint) -> Self {
        EllipticCurveVerifier {
            operations: EllipticCurveOperations::new(g, h),
        }
    }
}

impl Verifier for EllipticCurveVerifier {
    fn generate_challenge_c(&self) -> BigInt {
        let c = self.operations.generate_c();
        scalar_to_bigint(c)
    }

    fn compute_verification_r1r2(
        &self,
        y1: BigInt,
        y2: BigInt,
        c: BigInt,
        s: BigInt,
    ) -> (BigInt, BigInt) {
        let y1 = bigint_to_ristretto_point(y1);
        let y2 = bigint_to_ristretto_point(y2);
        let c = bigint_to_scalar(c);
        let s = bigint_to_scalar(s);
        let r1 = self.operations.compute_r1_prime(y1, c.clone(), s.clone());
        let r2 = self.operations.compute_r2_prime(y2, c, s);
        (ristretto_point_to_bigint(r1), ristretto_point_to_bigint(r2))
    }
}

pub fn generate_parameters() -> (RistrettoPoint, RistrettoPoint) {
    let g = RISTRETTO_BASEPOINT_POINT;

    let h_value = "Unique value for H";
    let mut hasher = Sha512::new();
    hasher.update(h_value.as_bytes());
    let h_result = hasher.finalize();
    let h_bigint: [u8; 64] = *h_result.as_ref();
    let h = RistrettoPoint::from_uniform_bytes(&h_bigint);

    let mut rng = rand::thread_rng();
    let secret = Scalar::random(&mut rng);
    let point_g = g * secret;
    let point_h = h * secret;

    (point_g, point_h)
}
