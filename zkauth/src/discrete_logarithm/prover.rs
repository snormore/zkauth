use num_bigint::{BigInt, BigUint};
use num_traits::Zero;
use sha2::{Digest, Sha512};

use super::{configuration::DiscreteLogarithmConfiguration, generate_random_scalar};
use crate::{Element, Prover, Scalar};

#[derive(Debug)]
pub struct DiscreteLogarithmProver {
    config: DiscreteLogarithmConfiguration,
}

impl DiscreteLogarithmProver {
    pub fn new(config: DiscreteLogarithmConfiguration) -> Self {
        DiscreteLogarithmProver { config }
    }

    fn compute_x(&self, password: String) -> BigInt {
        let x = BigUint::from_bytes_be(&Sha512::digest(password.as_bytes()));
        x.into()
    }

    fn generate_x(&self) -> BigInt {
        generate_random_scalar()
    }

    fn compute_y1(&self, x: BigInt) -> BigInt {
        self.config.g.modpow(&x, &self.config.p)
    }

    fn compute_y2(&self, x: BigInt) -> BigInt {
        self.config.h.modpow(&x, &self.config.p)
    }

    fn generate_k(&self) -> BigInt {
        generate_random_scalar()
    }

    fn compute_r1(&self, k: BigInt) -> BigInt {
        self.config.g.modpow(&k, &self.config.p)
    }

    fn compute_r2(&self, k: BigInt) -> BigInt {
        self.config.h.modpow(&k, &self.config.p)
    }

    fn compute_s(&self, x: BigInt, k: BigInt, c: BigInt) -> BigInt {
        let mut s = (k - c * x) % &self.config.q;
        if s < Zero::zero() {
            s += &self.config.q;
        }
        s
    }
}

impl Prover for DiscreteLogarithmProver {
    fn generate_registration_x(&self) -> Scalar {
        self.generate_x().into()
    }

    fn compute_registration_x(&self, password: String) -> Scalar {
        self.compute_x(password).into()
    }

    fn compute_registration_y1y2(&self, x: Scalar) -> (Element, Element) {
        let x: BigInt = x.into();
        let y1 = self.compute_y1(x.clone());
        let y2 = self.compute_y2(x);
        (y1.into(), y2.into())
    }

    fn generate_challenge_k(&self) -> Scalar {
        self.generate_k().into()
    }

    fn compute_challenge_commitment_r1r2(&self, k: Scalar) -> (Element, Element) {
        let k: BigInt = k.into();
        let r1 = self.compute_r1(k.clone());
        let r2 = self.compute_r2(k);
        (r1.into(), r2.into())
    }

    fn compute_challenge_response_s(&self, x: Scalar, k: Scalar, c: Scalar) -> Scalar {
        self.compute_s(x.into(), k.into(), c.into()).into()
    }
}
