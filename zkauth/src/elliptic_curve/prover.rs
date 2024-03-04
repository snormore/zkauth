use curve25519_dalek::{RistrettoPoint, Scalar};
use num_bigint::BigInt;
use sha2::{Digest, Sha512};

use super::{
    bigint_to_scalar, configuration::EllipticCurveConfiguration, generate_random_scalar,
    ristretto_point_to_bigint, scalar_to_bigint,
};
use crate::Prover;

#[derive(Debug)]
pub struct EllipticCurveProver {
    config: EllipticCurveConfiguration,
}

impl EllipticCurveProver {
    pub fn new(config: EllipticCurveConfiguration) -> Self {
        EllipticCurveProver { config }
    }

    fn compute_x(&self, password: String) -> Scalar {
        let password_hash = Sha512::digest(password.as_bytes());
        let mut x_bytes = [0u8; 32];
        x_bytes.copy_from_slice(&password_hash[..32]);
        let x = Scalar::from_bytes_mod_order(x_bytes);
        x
    }

    fn generate_x(&self) -> Scalar {
        generate_random_scalar()
    }

    fn compute_y1(&self, x: Scalar) -> RistrettoPoint {
        self.config.g * x
    }

    fn compute_y2(&self, x: Scalar) -> RistrettoPoint {
        self.config.h * x
    }

    fn generate_k(&self) -> Scalar {
        generate_random_scalar()
    }

    fn compute_r1(&self, k: Scalar) -> RistrettoPoint {
        self.config.g * k
    }

    fn compute_r2(&self, k: Scalar) -> RistrettoPoint {
        self.config.h * k
    }

    fn compute_s(&self, x: Scalar, k: Scalar, c: Scalar) -> Scalar {
        k + c * x
    }
}

impl Prover for EllipticCurveProver {
    fn generate_registration_x(&self) -> BigInt {
        let x = self.generate_x();
        scalar_to_bigint(x)
    }

    fn compute_registration_x(&self, password: String) -> BigInt {
        let x = self.compute_x(password);
        scalar_to_bigint(x)
    }

    fn compute_registration_y1y2(&self, x: BigInt) -> (BigInt, BigInt) {
        let x = bigint_to_scalar(x);
        let y1 = self.compute_y1(x);
        let y2 = self.compute_y2(x);
        (ristretto_point_to_bigint(y1), ristretto_point_to_bigint(y2))
    }

    fn generate_challenge_k(&self) -> BigInt {
        let c = self.generate_k();
        scalar_to_bigint(c)
    }

    fn compute_challenge_commitment_r1r2(&self, k: BigInt) -> (BigInt, BigInt) {
        let k = bigint_to_scalar(k);
        let r1 = self.compute_r1(k);
        let r2 = self.compute_r2(k);
        (ristretto_point_to_bigint(r1), ristretto_point_to_bigint(r2))
    }

    fn compute_challenge_response_s(&self, x: BigInt, k: BigInt, c: BigInt) -> BigInt {
        let x = bigint_to_scalar(x);
        let k = bigint_to_scalar(k);
        let c = bigint_to_scalar(c);
        let s = self.compute_s(x, k, c);
        scalar_to_bigint(s)
    }
}
