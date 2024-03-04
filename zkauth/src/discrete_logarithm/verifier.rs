use num_bigint::BigInt;
use num_traits::One;

use super::{configuration::DiscreteLogarithmConfiguration, generate_random_scalar};
use crate::{Element, Scalar, Verifier};

pub struct DiscreteLogarithmVerifier {
    config: DiscreteLogarithmConfiguration,
}

impl DiscreteLogarithmVerifier {
    pub fn new(config: DiscreteLogarithmConfiguration) -> Self {
        DiscreteLogarithmVerifier { config }
    }

    fn generate_c(&self) -> BigInt {
        generate_random_scalar()
    }

    fn compute_r1_prime(&self, y1: BigInt, c: BigInt, s: BigInt) -> BigInt {
        let one: BigInt = One::one();
        (self.config.g.modpow(&s, &self.config.p) * y1.modpow(&c, &self.config.p))
            .modpow(&one, &self.config.p)
    }

    fn compute_r2_prime(&self, y2: BigInt, c: BigInt, s: BigInt) -> BigInt {
        let one: BigInt = One::one();
        (self.config.h.modpow(&s, &self.config.p) * y2.modpow(&c, &self.config.p))
            .modpow(&one, &self.config.p)
    }
}

impl Verifier for DiscreteLogarithmVerifier {
    fn generate_challenge_c(&self) -> Scalar {
        self.generate_c().into()
    }

    fn compute_verification_r1r2(
        &self,
        y1: Element,
        y2: Element,
        c: Scalar,
        s: Scalar,
    ) -> (Element, Element) {
        let c: BigInt = c.into();
        let s: BigInt = s.into();
        let r1 = self.compute_r1_prime(y1.into(), c.clone(), s.clone());
        let r2 = self.compute_r2_prime(y2.into(), c, s);
        (r1.into(), r2.into())
    }
}
