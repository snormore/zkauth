use anyhow::Result;
use num_bigint::BigInt;
use num_traits::One;

use super::{configuration::DiscreteLogarithmConfiguration, generate_random_scalar};
use crate::{Element, Scalar, Verifier};

/// The discrete logarithm verifier.
#[derive(Clone)]
pub struct DiscreteLogarithmVerifier {
    config: DiscreteLogarithmConfiguration,
}

/// Implementation of the discrete logarithm verifier.
impl DiscreteLogarithmVerifier {
    /// Creates a new discrete logarithm verifier.
    pub fn new(config: DiscreteLogarithmConfiguration) -> Self {
        DiscreteLogarithmVerifier { config }
    }

    /// Generates a random c value.
    fn generate_c(&self) -> BigInt {
        generate_random_scalar()
    }

    /// Computes r1' from the given y1, c, and s using the g and p configuration values.
    fn compute_r1_prime(&self, y1: BigInt, c: BigInt, s: BigInt) -> BigInt {
        let one: BigInt = One::one();
        (self.config.g.modpow(&s, &self.config.p) * y1.modpow(&c, &self.config.p))
            .modpow(&one, &self.config.p)
    }

    /// Computes r2' from the given y2, c, and s using the h and p configuration values.
    fn compute_r2_prime(&self, y2: BigInt, c: BigInt, s: BigInt) -> BigInt {
        let one: BigInt = One::one();
        (self.config.h.modpow(&s, &self.config.p) * y2.modpow(&c, &self.config.p))
            .modpow(&one, &self.config.p)
    }
}

/// Implementation of the verifier trait for the discrete logarithm verifier.
impl Verifier for DiscreteLogarithmVerifier {
    /// Generates a challenge c value.
    fn generate_challenge_c(&self) -> Scalar {
        self.generate_c().into()
    }

    /// Computes verification r1' and r2' values from the given y1, y2, c, and s.
    fn compute_verification_r1r2(
        &self,
        y1: Element,
        y2: Element,
        c: Scalar,
        s: Scalar,
    ) -> Result<(Element, Element)> {
        let c: BigInt = c.into();
        let s: BigInt = s.into();
        let r1 = self.compute_r1_prime(y1.into(), c.clone(), s.clone());
        let r2 = self.compute_r2_prime(y2.into(), c, s);
        Ok((r1.into(), r2.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discrete_logarithm::test::test_verifier;
    use num_traits::Zero;

    #[test]
    fn test_generate_challenge_c() {
        let verifier = test_verifier();
        let c = verifier.generate_challenge_c();
        assert!(c > Scalar::zero());
    }
}
