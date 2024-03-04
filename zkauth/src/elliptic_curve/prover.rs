use anyhow::{Error, Result};
use curve25519_dalek::{RistrettoPoint, Scalar as DalekScalar};
use sha2::{Digest, Sha512};

use super::{configuration::EllipticCurveConfiguration, generate_random_scalar};
use crate::{Element, Prover, Scalar};

#[derive(Debug)]
pub struct EllipticCurveProver {
    config: EllipticCurveConfiguration,
}

impl EllipticCurveProver {
    pub fn new(config: EllipticCurveConfiguration) -> Self {
        EllipticCurveProver { config }
    }

    fn compute_x(&self, password: String) -> DalekScalar {
        let password_hash = Sha512::digest(password.as_bytes());
        let mut x_bytes = [0u8; 32];
        x_bytes.copy_from_slice(&password_hash[..32]);
        DalekScalar::from_bytes_mod_order(x_bytes)
    }

    fn generate_x(&self) -> DalekScalar {
        generate_random_scalar()
    }

    fn compute_y1(&self, x: DalekScalar) -> RistrettoPoint {
        self.config.g * x
    }

    fn compute_y2(&self, x: DalekScalar) -> RistrettoPoint {
        self.config.h * x
    }

    fn generate_k(&self) -> DalekScalar {
        generate_random_scalar()
    }

    fn compute_r1(&self, k: DalekScalar) -> RistrettoPoint {
        self.config.g * k
    }

    fn compute_r2(&self, k: DalekScalar) -> RistrettoPoint {
        self.config.h * k
    }

    fn compute_s(&self, x: DalekScalar, k: DalekScalar, c: DalekScalar) -> DalekScalar {
        k + c * x
    }
}

impl Prover for EllipticCurveProver {
    fn generate_registration_x(&self) -> Scalar {
        let x = self.generate_x();
        x.into()
    }

    fn compute_registration_x(&self, password: String) -> Scalar {
        let x = self.compute_x(password);
        x.into()
    }

    fn compute_registration_y1y2(&self, x: Scalar) -> Result<(Element, Element)> {
        let x: DalekScalar = x
            .try_into()
            .map_err(|_| Error::msg("Failed to convert scalar x"))?;
        let y1 = self.compute_y1(x);
        let y2 = self.compute_y2(x);
        Ok((y1.into(), y2.into()))
    }

    fn generate_challenge_k(&self) -> Scalar {
        let c = self.generate_k();
        c.into()
    }

    fn compute_challenge_commitment_r1r2(&self, k: Scalar) -> Result<(Element, Element)> {
        let k: DalekScalar = k
            .try_into()
            .map_err(|_| Error::msg("Failed to convert scalar k"))?;
        let r1 = self.compute_r1(k);
        let r2 = self.compute_r2(k);
        Ok((r1.into(), r2.into()))
    }

    fn compute_challenge_response_s(&self, x: Scalar, k: Scalar, c: Scalar) -> Result<Scalar> {
        let x = x
            .try_into()
            .map_err(|_| Error::msg("Failed to convert scalar x"))?;
        let k = k
            .try_into()
            .map_err(|_| Error::msg("Failed to convert scalar k"))?;
        let c = c
            .try_into()
            .map_err(|_| Error::msg("Failed to convert scalar c"))?;
        let s = self.compute_s(x, k, c);
        Ok(s.into())
    }
}
