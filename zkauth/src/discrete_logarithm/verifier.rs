use num_bigint::{BigInt, BigUint, RandBigInt, Sign, ToBigInt};
use num_primes::Generator;
use num_traits::One;

use super::operations::DiscreteLogarithmOperations;
use crate::{Operations, Verifier};

pub struct DiscreteLogarithmVerifier {
    operations: DiscreteLogarithmOperations,
}

impl DiscreteLogarithmVerifier {
    pub fn new(p: BigInt, q: BigInt, g: BigInt, h: BigInt) -> Self {
        DiscreteLogarithmVerifier {
            operations: DiscreteLogarithmOperations::new(p, q, g, h),
        }
    }
}

impl Verifier for DiscreteLogarithmVerifier {
    fn generate_challenge_c(&self) -> BigInt {
        self.operations.generate_c()
    }

    fn compute_verification_r1r2(
        &self,
        y1: BigInt,
        y2: BigInt,
        c: BigInt,
        s: BigInt,
    ) -> (BigInt, BigInt) {
        let r1 = self.operations.compute_r1_prime(y1, c.clone(), s.clone());
        let r2 = self.operations.compute_r2_prime(y2, c, s);
        (r1, r2)
    }
}

pub fn default_parameters() -> (BigInt, BigInt, BigInt, BigInt) {
    // Based on https://github.com/twilker/cp-zkp/blob/main/src/lib/chaum_pedersen/algorithm.rs#L11-L15
    let p = "42765216643065397982265462252423826320512529931694366715111734768493812630447"
        .parse::<BigInt>()
        .unwrap();
    let q = "21382608321532698991132731126211913160256264965847183357555867384246906315223"
        .parse::<BigInt>()
        .unwrap();
    let g = 4.to_bigint().unwrap();
    let h = 9.to_bigint().unwrap();
    (p, q, g, h)
}

pub fn generate_parameters(prime_bits: usize) -> (BigInt, BigInt, BigInt, BigInt) {
    // Based on https://github.com/neongazer/zkp-auth-py/blob/main/zkp_auth/sigma_protocols/utils.py
    let p = BigInt::from_biguint(
        Sign::Plus,
        BigUint::from_bytes_be(&Generator::safe_prime(prime_bits).to_bytes_be()),
    );
    let one: BigInt = One::one();
    let two = &one + &one;
    let q = (&p - one) / two;

    let g1 = generate_g(p.clone(), q.clone());
    let mut g2 = generate_g(p.clone(), q.clone());

    while g1 == g2 {
        g2 = generate_g(p.clone(), q.clone());
    }
    let (g, h) = if g1 < g2 { (g1, g2) } else { (g2, g1) };

    (p, q, g, h)
}

fn generate_g(p: BigInt, q: BigInt) -> BigInt {
    let one = One::one();
    let mut rng = rand::thread_rng();
    let mut g = rng.gen_bigint_range(&one, &p);
    while g.modpow(&q, &p) != one {
        g = rng.gen_bigint_range(&one, &p);
    }
    g
}
