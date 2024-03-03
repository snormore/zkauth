use bytes::Bytes;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::RistrettoPoint;
use curve25519_dalek::Scalar;
use rand::rngs::OsRng;
use sha2::{Digest, Sha512};

use super::bytes_to_ristretto_point;
use super::bytes_to_scalar;
use super::operations::EllipticCurveOperations;
use super::ristretto_point_to_bytes;
use super::scalar_to_bytes;
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
    fn generate_challenge_c(&self) -> Bytes {
        let c = self.operations.generate_c();
        scalar_to_bytes(c)
    }

    fn compute_verification_r1r2(
        &self,
        y1: Bytes,
        y2: Bytes,
        c: Bytes,
        s: Bytes,
    ) -> (Bytes, Bytes) {
        let y1 = bytes_to_ristretto_point(y1);
        let y2 = bytes_to_ristretto_point(y2);
        let c = bytes_to_scalar(c);
        let s = bytes_to_scalar(s);
        let r1 = self.operations.compute_vr1(y1, c.clone(), s.clone());
        let r2 = self.operations.compute_vr2(y2, c, s);
        (ristretto_point_to_bytes(r1), ristretto_point_to_bytes(r2))
    }
}

pub fn generate_parameters() -> (RistrettoPoint, RistrettoPoint) {
    // TODO: clean this up
    let g = RISTRETTO_BASEPOINT_POINT;

    let h_value = "Unique value for H";
    let mut hasher = Sha512::new();
    hasher.update(h_value.as_bytes());
    let h_result = hasher.finalize();
    let h_bytes: [u8; 64] = *h_result.as_ref();
    let h = RistrettoPoint::from_uniform_bytes(&h_bytes);

    // TODO: use threadrng here instead
    let mut rng = OsRng;
    let secret = Scalar::random(&mut rng);
    let point_g = g * secret;
    let point_h = h * secret;

    (point_g, point_h)
}
