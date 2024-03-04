use num_bigint::{BigInt, BigUint};
use num_traits::Zero;
use sha2::{Digest, Sha512};

use super::{configuration::DiscreteLogarithmConfiguration, generate_random_scalar};
use crate::Prover;

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
        let signed_x: BigInt = x.clone().into();
        signed_x
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
    fn generate_registration_x(&self) -> BigInt {
        self.generate_x()
    }

    fn compute_registration_x(&self, password: String) -> BigInt {
        self.compute_x(password)
    }

    fn compute_registration_y1y2(&self, x: BigInt) -> (BigInt, BigInt) {
        let y1 = self.compute_y1(x.clone());
        let y2 = self.compute_y2(x);
        (y1, y2)
    }

    fn generate_challenge_k(&self) -> BigInt {
        self.generate_k()
    }

    fn compute_challenge_commitment_r1r2(&self, k: BigInt) -> (BigInt, BigInt) {
        let r1 = self.compute_r1(k.clone());
        let r2 = self.compute_r2(k);
        (r1, r2)
    }

    fn compute_challenge_response_s(&self, x: BigInt, k: BigInt, c: BigInt) -> BigInt {
        self.compute_s(x, k, c)
    }
}
