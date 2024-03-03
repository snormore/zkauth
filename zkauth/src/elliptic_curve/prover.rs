use bytes::Bytes;
use curve25519_dalek::RistrettoPoint;

use super::operations::EllipticCurveOperations;
use super::{bytes_to_scalar, ristretto_point_to_bytes, scalar_to_bytes};
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
    fn generate_registration_x(&self) -> Bytes {
        let x = self.operations.generate_x();
        scalar_to_bytes(x)
    }

    fn compute_registration_x(&self, password: String) -> Bytes {
        let x = self.operations.compute_x(password);
        scalar_to_bytes(x)
    }

    fn compute_registration_y1y2(&self, x: Bytes) -> (Bytes, Bytes) {
        let x = bytes_to_scalar(x);
        let y1 = self.operations.compute_y1(x);
        let y2 = self.operations.compute_y2(x);
        (ristretto_point_to_bytes(y1), ristretto_point_to_bytes(y2))
    }

    fn generate_challenge_k(&self) -> Bytes {
        let c = self.operations.generate_k();
        scalar_to_bytes(c)
    }

    fn compute_challenge_commitment_r1r2(&self, k: Bytes) -> (Bytes, Bytes) {
        let k = bytes_to_scalar(k);
        let r1 = self.operations.compute_r1(k);
        let r2 = self.operations.compute_r2(k);
        (ristretto_point_to_bytes(r1), ristretto_point_to_bytes(r2))
    }

    fn compute_challenge_response_s(&self, x: Bytes, k: Bytes, c: Bytes) -> Bytes {
        let x = bytes_to_scalar(x);
        let k = bytes_to_scalar(k);
        let c = bytes_to_scalar(c);
        let s = self.operations.compute_s(x, k, c);
        scalar_to_bytes(s)
    }
}
