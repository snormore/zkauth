use num_bigint::{BigInt, RandBigInt, Sign};
use num_primes::Generator;
use num_traits::One;
use std::panic::{self, AssertUnwindSafe};

#[derive(Debug, Clone)]
/// Configuration for the discrete logarithm protocol.
pub struct DiscreteLogarithmConfiguration {
    /// The prime number.
    pub p: BigInt,

    /// Prime order of the groups that g and h generate.
    pub q: BigInt,

    /// A group generator of prime order q.
    pub g: BigInt,

    /// A group generator of prime order q.
    pub h: BigInt,
}

/// Configuration for the discrete logarithm protocol.
impl DiscreteLogarithmConfiguration {
    /// Generates a configuration with the specified number of bits for the prime number.
    pub fn generate(prime_bits: usize) -> DiscreteLogarithmConfiguration {
        let p = generate_prime_p(prime_bits);
        Self::generate_from_prime(p)
    }

    /// Generates a configuration from a prime number.
    pub fn generate_from_prime(p: BigInt) -> DiscreteLogarithmConfiguration {
        // Based on https://github.com/neongazer/zkp-auth-py/blob/main/zkp_auth/sigma_protocols/utils.py
        let one: BigInt = One::one();
        let two = &one + &one;
        let q = (&p - one) / two;

        let g1 = generate_generator_g(p.clone(), q.clone());
        let mut g2 = generate_generator_g(p.clone(), q.clone());

        while g1 == g2 {
            g2 = generate_generator_g(p.clone(), q.clone());
        }
        let (g, h) = if g1 < g2 { (g1, g2) } else { (g2, g1) };
        DiscreteLogarithmConfiguration { p, q, g, h }
    }
}

/// Generates a prime number with the specified number of bits.
fn generate_prime_p(bits: usize) -> BigInt {
    let prime = loop {
        // There are times when the prime generation panics, so we catch the panic and retry.
        // The panic is coming from this line: https://github.com/rust-num/num-bigint/blob/num-bigint-0.4.4/src/bigrand.rs#L123
        // When it happens, the output looks like this:
        //   thread 'service::test::get_configuration::succeeds_with_discrete_logarithm_config' panicked at /Users/snormore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/num-bigint-0.2.6/src/bigrand.rs:95:9:
        //   assertion failed: *lbound < *ubound
        let result = panic::catch_unwind(AssertUnwindSafe(|| Generator::safe_prime(bits)));

        match result {
            Ok(prime) => break prime,
            Err(_) => {
                log::warn!("Panic occurred during prime generation, retrying...");
            }
        }
    };
    BigInt::from_bytes_le(Sign::Plus, &prime.to_bytes_le())
}

/// Generates a generator for the specified prime number and its corresponding q.
fn generate_generator_g(p: BigInt, q: BigInt) -> BigInt {
    let one = One::one();
    let mut rng = rand::thread_rng();
    let mut g = rng.gen_bigint_range(&one, &p);
    while g.modpow(&q, &p) != one {
        g = rng.gen_bigint_range(&one, &p);
    }
    g
}
