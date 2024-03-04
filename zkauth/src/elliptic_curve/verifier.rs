use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::RistrettoPoint;
use curve25519_dalek::Scalar;
use num_bigint::BigInt;
use sha2::{Digest, Sha512};

use super::configuration::EllipticCurveConfiguration;
use super::generate_random_scalar;
use super::{
    bigint_to_ristretto_point, bigint_to_scalar, ristretto_point_to_bigint, scalar_to_bigint,
};
use crate::Verifier;

pub struct EllipticCurveVerifier {
    config: EllipticCurveConfiguration,
}

impl EllipticCurveVerifier {
    pub fn new(config: EllipticCurveConfiguration) -> Self {
        EllipticCurveVerifier { config }
    }

    fn generate_c(&self) -> Scalar {
        generate_random_scalar()
    }

    fn compute_r1_prime(&self, y1: RistrettoPoint, c: Scalar, s: Scalar) -> RistrettoPoint {
        (self.config.g * s) - (y1 * c)
    }

    fn compute_r2_prime(&self, y2: RistrettoPoint, c: Scalar, s: Scalar) -> RistrettoPoint {
        (self.config.h * s) - (y2 * c)
    }
}

impl Verifier for EllipticCurveVerifier {
    fn generate_challenge_c(&self) -> BigInt {
        let c = self.generate_c();
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
        let r1 = self.compute_r1_prime(y1, c.clone(), s.clone());
        let r2 = self.compute_r2_prime(y2, c, s);
        (ristretto_point_to_bigint(r1), ristretto_point_to_bigint(r2))
    }
}
