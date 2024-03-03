use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use sha2::{Digest, Sha512};

use crate::Operations;

#[derive(Debug, Clone)]
pub struct EllipticCurveOperations {
    g: RistrettoPoint,
    h: RistrettoPoint,
}

impl EllipticCurveOperations {
    pub fn new(g: RistrettoPoint, h: RistrettoPoint) -> Self {
        EllipticCurveOperations { g, h }
    }

    fn generate_random_scalar(&self) -> Scalar {
        let mut rng = rand::thread_rng();
        Scalar::random(&mut rng)
    }
}

impl Operations<RistrettoPoint, Scalar> for EllipticCurveOperations {
    fn compute_x(&self, password: String) -> Scalar {
        let password_hash = Sha512::digest(password.as_bytes());
        let mut x_bytes = [0u8; 32];
        x_bytes.copy_from_slice(&password_hash[..32]);
        let x = Scalar::from_bytes_mod_order(x_bytes);
        x
    }

    fn generate_x(&self) -> Scalar {
        self.generate_random_scalar()
    }

    fn compute_y1(&self, x: Scalar) -> RistrettoPoint {
        self.g * x
    }

    fn compute_y2(&self, x: Scalar) -> RistrettoPoint {
        self.h * x
    }

    fn generate_k(&self) -> Scalar {
        self.generate_random_scalar()
    }

    fn compute_r1(&self, k: Scalar) -> RistrettoPoint {
        self.g * k
    }

    fn compute_r2(&self, k: Scalar) -> RistrettoPoint {
        self.h * k
    }

    fn generate_c(&self) -> Scalar {
        self.generate_random_scalar()
    }

    fn compute_s(&self, x: Scalar, k: Scalar, c: Scalar) -> Scalar {
        k + c * x
    }

    fn compute_vr1(&self, y1: RistrettoPoint, c: Scalar, s: Scalar) -> RistrettoPoint {
        (self.g * s) - (y1 * c)
    }

    fn compute_vr2(&self, y2: RistrettoPoint, c: Scalar, s: Scalar) -> RistrettoPoint {
        (self.h * s) - (y2 * c)
    }
}
