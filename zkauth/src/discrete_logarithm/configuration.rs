use num_bigint::{BigInt, BigUint, RandBigInt, Sign};
use num_primes::Generator;
use num_traits::One;

#[derive(Debug, Clone)]
/// Configuration for the discrete logarithm protocol.
pub struct DiscreteLogarithmConfiguration {
    pub p: BigInt,
    pub q: BigInt,
    pub g: BigInt,
    pub h: BigInt,
}

impl DiscreteLogarithmConfiguration {
    pub fn generate(prime_bits: usize) -> DiscreteLogarithmConfiguration {
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
        DiscreteLogarithmConfiguration { p, q, g, h }
    }
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
