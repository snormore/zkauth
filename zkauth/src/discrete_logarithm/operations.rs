use num_bigint::{BigInt, BigUint, RandomBits};
use num_traits::{One, Zero};
use rand::Rng;
use sha2::{Digest, Sha512};

use crate::Operations;

#[derive(Debug, Clone)]
pub struct DiscreteLogarithmOperations {
    p: BigInt,
    q: BigInt,
    g: BigInt,
    h: BigInt,
}

impl DiscreteLogarithmOperations {
    pub fn new(p: BigInt, q: BigInt, g: BigInt, h: BigInt) -> Self {
        DiscreteLogarithmOperations { p, q, g, h }
    }

    fn generate_random_scalar(&self) -> BigInt {
        let mut rng = rand::thread_rng();
        let num: BigUint = rng.sample(RandomBits::new(32));
        let signed_num: BigInt = num.clone().into();
        signed_num
    }
}

impl Operations<BigInt, BigInt> for DiscreteLogarithmOperations {
    fn compute_x(&self, password: String) -> BigInt {
        let x = BigUint::from_bytes_be(&Sha512::digest(password.as_bytes()));
        let signed_x: BigInt = x.clone().into();
        signed_x
    }

    fn generate_x(&self) -> BigInt {
        self.generate_random_scalar()
    }

    fn compute_y1(&self, x: BigInt) -> BigInt {
        self.g.modpow(&x, &self.p)
    }

    fn compute_y2(&self, x: BigInt) -> BigInt {
        self.h.modpow(&x, &self.p)
    }

    fn generate_k(&self) -> BigInt {
        self.generate_random_scalar()
    }

    fn compute_r1(&self, k: BigInt) -> BigInt {
        self.g.modpow(&k, &self.p)
    }

    fn compute_r2(&self, k: BigInt) -> BigInt {
        self.h.modpow(&k, &self.p)
    }

    fn generate_c(&self) -> BigInt {
        self.generate_random_scalar()
    }

    fn compute_s(&self, x: BigInt, k: BigInt, c: BigInt) -> BigInt {
        let mut s = (k - c * x) % &self.q;
        if s < Zero::zero() {
            s += &self.q;
        }
        s
    }

    // TODO: rename to compute_r1_prime?
    fn compute_vr1(&self, y1: BigInt, c: BigInt, s: BigInt) -> BigInt {
        let one: BigInt = One::one();
        (self.g.modpow(&s, &self.p) * y1.modpow(&c, &self.p)).modpow(&one, &self.p)
    }

    fn compute_vr2(&self, y2: BigInt, c: BigInt, s: BigInt) -> BigInt {
        let one: BigInt = One::one();
        (self.h.modpow(&s, &self.p) * y2.modpow(&c, &self.p)).modpow(&one, &self.p)
    }
}
