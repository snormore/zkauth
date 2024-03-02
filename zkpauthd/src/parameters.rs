use num_bigint::{BigInt, BigUint, RandBigInt, Sign, ToBigInt};
use num_primes::Generator;
use num_traits::One;

#[derive(Debug)]
pub struct Parameters {
    pub p: BigInt,
    pub q: BigInt,
    pub g: BigInt,
    pub h: BigInt,
}

pub fn default_parameters() -> Parameters {
    // Based on https://github.com/twilker/cp-zkp/blob/main/src/lib/chaum_pedersen/algorithm.rs#L11-L15
    Parameters {
        p: "42765216643065397982265462252423826320512529931694366715111734768493812630447"
            .parse::<BigInt>()
            .unwrap(),
        q: "21382608321532698991132731126211913160256264965847183357555867384246906315223"
            .parse::<BigInt>()
            .unwrap(),
        g: 4.to_bigint().unwrap(),
        h: 9.to_bigint().unwrap(),
    }
}

pub fn generate_parameters(prime_bits: usize) -> Parameters {
    // Based on https://github.com/neongazer/zkp-auth-py/blob/main/zkp_auth/sigma_protocols/utils.py
    let p = BigInt::from_biguint(
        Sign::Plus,
        BigUint::from_bytes_be(&Generator::safe_prime(prime_bits).to_bytes_be()),
    );
    let q = (&p - 1.to_bigint().unwrap()) / 2.to_bigint().unwrap();

    let g1 = generate_g(p.clone(), q.clone());
    let mut g2 = generate_g(p.clone(), q.clone());

    while g1 == g2 {
        g2 = generate_g(p.clone(), q.clone());
    }
    let (g, h) = if g1 < g2 { (g1, g2) } else { (g2, g1) };

    Parameters { p, q, g, h }
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

#[cfg(test)]
mod default_parameters {
    use super::*;
    use anyhow::Result;
    use num_traits::Zero;

    #[test]
    fn succeeds() -> Result<()> {
        let params = default_parameters();
        assert_ne!(params.p, Zero::zero());
        assert_ne!(params.p, params.q);
        assert_eq!(params.g, 4.to_bigint().unwrap());
        assert_eq!(params.h, 9.to_bigint().unwrap());
        Ok(())
    }
}

#[cfg(test)]
mod generate_parameters {
    use super::*;
    use anyhow::Result;
    use num_traits::Zero;

    #[test]
    fn succeeds() -> Result<()> {
        let params = generate_parameters(16);
        assert_ne!(params.p, Zero::zero());
        assert_ne!(params.p, params.q);
        assert_ne!(params.g, Zero::zero());
        assert_ne!(params.g, params.h);
        Ok(())
    }
}
