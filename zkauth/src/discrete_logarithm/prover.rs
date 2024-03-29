use anyhow::Result;
use num_bigint::{BigInt, BigUint};
use num_traits::Zero;
use sha2::{Digest, Sha512};

use super::{configuration::DiscreteLogarithmConfiguration, generate_random_scalar};
use crate::{Element, Prover, Scalar};

/// The discrete logarithm prover.
#[derive(Debug)]
pub struct DiscreteLogarithmProver {
    config: DiscreteLogarithmConfiguration,
}

/// Implementation of the discrete logarithm prover.
impl DiscreteLogarithmProver {
    /// Creates a new discrete logarithm prover.
    pub fn new(config: DiscreteLogarithmConfiguration) -> Self {
        DiscreteLogarithmProver { config }
    }

    /// Computes x from the given password.
    fn compute_x(&self, password: String) -> BigInt {
        let x = BigUint::from_bytes_be(&Sha512::digest(password.as_bytes()));
        x.into()
    }

    /// Generates a random x value.
    fn generate_x(&self) -> BigInt {
        generate_random_scalar()
    }

    /// Computes y1 from the given x using the g and p configuration values.
    fn compute_y1(&self, x: BigInt) -> BigInt {
        self.config.g.modpow(&x, &self.config.p)
    }

    /// Computes y2 from the given x using the h and p configuration values.
    fn compute_y2(&self, x: BigInt) -> BigInt {
        self.config.h.modpow(&x, &self.config.p)
    }

    /// Generates a random k value.
    fn generate_k(&self) -> BigInt {
        generate_random_scalar()
    }

    /// Computes r1 from the given k using the g and p configuration values.
    fn compute_r1(&self, k: BigInt) -> BigInt {
        self.config.g.modpow(&k, &self.config.p)
    }

    /// Computes r2 from the given k using the h and p configuration values.
    fn compute_r2(&self, k: BigInt) -> BigInt {
        self.config.h.modpow(&k, &self.config.p)
    }

    /// Computes s from the given x, k, and c using the q configuration value.
    fn compute_s(&self, x: BigInt, k: BigInt, c: BigInt) -> BigInt {
        let mut s = (k - c * x) % &self.config.q;
        if s < Zero::zero() {
            s += &self.config.q;
        }
        s
    }
}

/// Prover implementation for the discrete logarithm protocol.
impl Prover for DiscreteLogarithmProver {
    /// Generates a registration x value.
    fn generate_registration_x(&self) -> Scalar {
        self.generate_x().into()
    }

    /// Computes a registration x value from the given password.
    fn compute_registration_x(&self, password: String) -> Scalar {
        self.compute_x(password).into()
    }

    /// Computes a registration y1 and y2 value from the given x.
    fn compute_registration_y1y2(&self, x: Scalar) -> Result<(Element, Element)> {
        let x: BigInt = x.into();
        let y1 = self.compute_y1(x.clone());
        let y2 = self.compute_y2(x);
        Ok((y1.into(), y2.into()))
    }

    /// Generates a challenge k value.
    fn generate_challenge_k(&self) -> Scalar {
        self.generate_k().into()
    }

    /// Computes a challenge commitment r1 and r2 value from the given k.
    fn compute_challenge_commitment_r1r2(&self, k: Scalar) -> Result<(Element, Element)> {
        let k: BigInt = k.into();
        let r1 = self.compute_r1(k.clone());
        let r2 = self.compute_r2(k);
        Ok((r1.into(), r2.into()))
    }

    /// Computes a challenge response s value from the given x, k, and c.
    fn compute_challenge_response_s(&self, x: Scalar, k: Scalar, c: Scalar) -> Result<Scalar> {
        Ok(self.compute_s(x.into(), k.into(), c.into()).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discrete_logarithm::test::test_prover;

    #[test]
    fn generate_registration_x() {
        let prover = test_prover();
        let x1 = prover.generate_registration_x();
        assert!(x1 > Scalar::zero());
        let x2 = prover.generate_registration_x();
        assert!(x2 > Scalar::zero());
        assert_ne!(x1, x2);
    }

    #[test]
    fn compute_registration_x_with_password() {
        let prover = test_prover();
        let x1 = prover.compute_registration_x("password".to_string());
        assert!(x1 > Scalar::zero());
        let x2 = prover.compute_registration_x("password".to_string());
        assert!(x2 > Scalar::zero());
        assert_eq!(x1, x2);
        let x3 = prover.compute_registration_x("password2".to_string());
        assert!(x3 > Scalar::zero());
        assert_ne!(x1, x3);
    }

    #[test]
    fn compute_registration_x_with_empty_password() {
        let prover = test_prover();
        let x = prover.compute_registration_x("".to_string());
        assert!(x > Scalar::zero());
    }

    #[test]
    fn compute_registration_y1y2() {
        let prover = test_prover();
        let x = prover.generate_x();
        let (y1, y2) = prover.compute_registration_y1y2(x.into()).unwrap();
        assert!(y1 > Element::zero());
        assert!(y2 > Element::zero());
        assert_ne!(y1, y2);
    }

    #[test]
    fn generate_challenge_k() {
        let prover = test_prover();
        let k1 = prover.generate_challenge_k();
        assert!(k1 > Scalar::zero());
        let k2 = prover.generate_challenge_k();
        assert!(k2 > Scalar::zero());
        assert_ne!(k1, k2);
    }

    #[test]
    fn compute_challenge_commitment_r1r2() {
        let prover = test_prover();
        let k = prover.generate_k();
        let (r1, r2) = prover.compute_challenge_commitment_r1r2(k.into()).unwrap();
        assert!(r1 > Element::zero());
        assert!(r2 > Element::zero());
        assert_ne!(r1, r2);
    }

    #[test]
    fn compute_challenge_response_s() {
        let prover = test_prover();
        let x = prover.generate_x();
        let k = prover.generate_k();
        let c = generate_random_scalar();
        let s = prover
            .compute_challenge_response_s(x.into(), k.into(), c.into())
            .unwrap();
        assert!(s >= Scalar::zero() && s < prover.config.q.into());
    }
}
