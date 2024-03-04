use anyhow::{Error, Result};
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
    ) -> Result<(Element, Element)> {
        let y1: RistrettoPoint = y1
            .try_into()
            .map_err(|_| Error::msg("Failed to convert element y1"))?;
        let y2: RistrettoPoint = y2
            .try_into()
            .map_err(|_| Error::msg("Failed to convert element y2"))?;
        let c: DalekScalar = c
            .try_into()
            .map_err(|_| Error::msg("Failed to convert scalar c"))?;
        let s: DalekScalar = s
            .try_into()
            .map_err(|_| Error::msg("Failed to convert scalar s"))?;
        let r1 = self.compute_r1_prime(y1, c, s);
        let r2 = self.compute_r2_prime(y2, c, s);
        Ok((r1.into(), r2.into()))
    }
}
