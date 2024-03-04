use curve25519_dalek::RistrettoPoint;
use num_bigint::BigInt;

use super::operations::EllipticCurveOperations;
use super::{bigint_to_scalar, ristretto_point_to_bigint, scalar_to_bigint};
use crate::{Operations, Prover};

#[derive(Debug)]
pub struct EllipticCurveProver {
    operations: EllipticCurveOperations,
}

impl EllipticCurveProver {
    pub fn new(g: RistrettoPoint, h: RistrettoPoint) -> Self {
        EllipticCurveProver {
            operations: EllipticCurveOperations::new(g, h),
        }
    }
}

impl Prover for EllipticCurveProver {
    fn generate_registration_x(&self) -> BigInt {
        let x = self.operations.generate_x();
        scalar_to_bigint(x)
    }

    fn compute_registration_x(&self, password: String) -> BigInt {
        let x = self.operations.compute_x(password);
        scalar_to_bigint(x)
    }

    fn compute_registration_y1y2(&self, x: BigInt) -> (BigInt, BigInt) {
        let x = bigint_to_scalar(x);
        let y1 = self.operations.compute_y1(x);
        let y2 = self.operations.compute_y2(x);
        (ristretto_point_to_bigint(y1), ristretto_point_to_bigint(y2))
    }

    fn generate_challenge_k(&self) -> BigInt {
        let c = self.operations.generate_k();
        scalar_to_bigint(c)
    }

    fn compute_challenge_commitment_r1r2(&self, k: BigInt) -> (BigInt, BigInt) {
        let k = bigint_to_scalar(k);
        let r1 = self.operations.compute_r1(k);
        let r2 = self.operations.compute_r2(k);
        (ristretto_point_to_bigint(r1), ristretto_point_to_bigint(r2))
    }

    fn compute_challenge_response_s(&self, x: BigInt, k: BigInt, c: BigInt) -> BigInt {
        let x = bigint_to_scalar(x);
        let k = bigint_to_scalar(k);
        let c = bigint_to_scalar(c);
        let s = self.operations.compute_s(x, k, c);
        scalar_to_bigint(s)
    }
}
