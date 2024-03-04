use curve25519_dalek::{RistrettoPoint, Scalar as DalekScalar};

use super::configuration::EllipticCurveConfiguration;
use super::generate_random_scalar;
use crate::Verifier;
use crate::{Element, Scalar};

pub struct EllipticCurveVerifier {
    config: EllipticCurveConfiguration,
}

impl EllipticCurveVerifier {
    pub fn new(config: EllipticCurveConfiguration) -> Self {
        EllipticCurveVerifier { config }
    }

    fn generate_c(&self) -> DalekScalar {
        generate_random_scalar()
    }

    fn compute_r1_prime(
        &self,
        y1: RistrettoPoint,
        c: DalekScalar,
        s: DalekScalar,
    ) -> RistrettoPoint {
        (self.config.g * s) - (y1 * c)
    }

    fn compute_r2_prime(
        &self,
        y2: RistrettoPoint,
        c: DalekScalar,
        s: DalekScalar,
    ) -> RistrettoPoint {
        (self.config.h * s) - (y2 * c)
    }
}

impl Verifier for EllipticCurveVerifier {
    fn generate_challenge_c(&self) -> Scalar {
        let c = self.generate_c();
        c.into()
    }

    fn compute_verification_r1r2(
        &self,
        y1: Element,
        y2: Element,
        c: Scalar,
        s: Scalar,
    ) -> (Element, Element) {
        let y1: RistrettoPoint = y1.into();
        let y2: RistrettoPoint = y2.into();
        let c: DalekScalar = c.into();
        let s: DalekScalar = s.into();
        let r1 = self.compute_r1_prime(y1, c, s);
        let r2 = self.compute_r2_prime(y2, c, s);
        (r1.into(), r2.into())
    }
}
