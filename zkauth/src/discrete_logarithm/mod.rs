//! The classic Chaum-Pedersen protocol is a cryptographic technique mainly used for proving that
//! two discrete logarithms are equal and that they correspond to the same base without revealing
//! the actual values. This protocol is commonly utilized in privacy-preserving cryptographic
//! systems such as electronic voting schemes and zero-knowledge proof constructions.
//!
//! Here are the steps of the Chaum-Pedersen protocol:
//!
//! 1. **Setup**: The prover and verifier agree on a prime `p` and a generator `g` of a cyclic
//! group `G` of order `q`, where `q` is a large prime factor of `p-1`. The prover knows a secret
//! `x`, which is the discrete logarithm of both `y_1 = g^x \mod p` and `y_2 = h^x \mod p` to the
//! bases `g` and `h`, respectively. Note that `h` is another element of `G`, and the equality of
//! logarithms `\log_g(y_1) = \log_h(y_2) = x` is what the prover intends to prove without
//! revealing `x`.
//! 2. **Commitment**: The prover selects a random value `k` from the group `G` and computes two
//! commitments `r_1 = g^k \mod p` and `r_2 = h^k \mod p`. The prover then sends the commitments
//! `r_1` and `r_2` to the verifier.
//! 3. **Challenge**: The verifier sends a random challenge `c` to the prover. This challenge is
//! typically a random number selected from a range that ensures security, such as the order of
//! the group `q`.
//! 4. **Response**: Upon receiving the challenge `c`, the prover computes the response
//! `s = k - c \cdot x \mod q` and sends `s` to the verifier.
//! 5. **Verification**: The verifier checks the validity of the prover's response by ensuring
//! that both `r_1 = g^s \cdot y_1^c \mod p` and `r_2 = h^s \cdot y_2^c \mod p` hold true. If
//! both equations are satisfied, the verifier accepts the proof; otherwise, the proof is rejected.
//!
//! The protocol ensures that the prover knows the discrete logarithm `x` without revealing it. The
//! security of the protocol relies on the difficulty of computing discrete logarithms in the group
//! `G`.

use num_bigint::{BigInt, BigUint, RandomBits};
use rand::Rng;

/// The configuration module.
pub mod configuration;

/// The prover module.
pub mod prover;

/// The verifier module.
pub mod verifier;

#[cfg(test)]
mod test;

/// Generates a random non-negative scalar.
fn generate_random_scalar() -> BigInt {
    let mut rng = rand::thread_rng();
    let num: BigUint = rng.sample(RandomBits::new(32));
    let signed_num: BigInt = num.clone().into();
    signed_num
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::Zero;

    #[test]
    fn test_generate_random_scalar() {
        let scalar = generate_random_scalar();
        assert!(scalar >= BigInt::zero());
    }
}
