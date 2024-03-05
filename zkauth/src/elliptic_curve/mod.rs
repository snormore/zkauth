//! Adapting the Chaum-Pedersen protocol to elliptic curves involves leveraging the elliptic curve
//! discrete logarithm problem (ECDLP) instead of the classical discrete logarithm problem in a
//! cyclic group. The fundamental principles remain similar, but the operations are adapted to the
//! properties and operations of elliptic curves.
//!
//! Here's how the steps adapt:
//!
//! 1. **Setup**: Instead of agreeing on a prime `p` and a generator `g` of a cyclic group, the
//! prover and verifier agree on an elliptic curve `E` defined over a finite field and a base point
//! `G` on `E` of prime order `q`. The prover knows a secret scalar `x`, which corresponds to the
//! discrete logarithm (with respect to base point `G`) of two points `Y_1 = xG` and `Y_2 = xH` on
//! the elliptic curve, where `H` is another point on the curve. The prover intends to demonstrate
//! that `\log_G(Y_1) = \log_H(Y_2) = x` without revealing `x`.
//! 2. **Commitment**: The prover picks a random scalar `k` from the set `1, ..., q-1` and computes
//! two commitment points `R_1 = kG` and `R_2 = kH` on the elliptic curve. These commitments `R_1`
//! and `R_2` are then sent to the verifier.
//! 3. **Challenge**: The verifier generates a random challenge scalar `c` and sends it to the
//! prover. This challenge is again a random scalar from the set `1, ..., q-1`.
//! 4. **Response**: Upon receiving `c`, the prover calculates the response scalar `s = k + cx \mod
//! q` and sends `s` back to the verifier.
//! 5. **Verification**: The verifier receives `s` and validates the prover’s claims by checking if
//! `sG = R_1 + cY_1` and `sH = R_2 + cY_2` on the elliptic curve, or equivalently if
//! `R_1 = sG - cY_1` and `R_2 = sH - cY_2`. If both equations hold, the prover's claim is
//! accepted; otherwise, it is rejected.
//!
//! Adapting the protocol to elliptic curves maintains the privacy and security characteristics of
//! the original Chaum-Pedersen protocol while leveraging the added security benefits and
//! efficiency of elliptic curve cryptography, which typically allows for shorter key sizes
//! compared to traditional discrete logarithm-based systems for a comparable level of security.
//! The main changes involve moving from multiplicative group operations to additive elliptic curve
//! group operations and from working with integers modulo a prime to working with points on an
//! elliptic curve.

use curve25519_dalek::{ristretto::CompressedRistretto, RistrettoPoint, Scalar as DalekScalar};
use num_bigint::{BigInt, Sign};

use crate::{ConversionError, Element, Scalar};

/// The configuration module.
pub mod configuration;

/// The prover module.
pub mod prover;

/// The verifier module.
pub mod verifier;

#[cfg(test)]
mod test;

/// Generates a random scalar.
fn generate_random_scalar() -> DalekScalar {
    let mut rng = rand::thread_rng();
    DalekScalar::random(&mut rng)
}

/// Converts a dalek scalar to a BigInt scalar.
impl TryFrom<Scalar> for DalekScalar {
    type Error = ConversionError;

    fn try_from(value: Scalar) -> Result<Self, Self::Error> {
        let (_, mut bytes) = value.0.to_bytes_le();
        bytes.resize(32, 0);
        let bytes: [u8; 32] = bytes.try_into().map_err(|_| ConversionError)?;
        let scalar = DalekScalar::from_canonical_bytes(bytes);
        if scalar.is_some().into() {
            Ok(scalar.unwrap())
        } else {
            Err(ConversionError)
        }
    }
}

/// Converts a BigInt scalar to a dalek scalar.
impl From<DalekScalar> for Scalar {
    fn from(value: DalekScalar) -> Self {
        let v = value.to_bytes();
        Scalar(BigInt::from_bytes_le(Sign::Plus, &v))
    }
}

/// Converts a ristretto point to a BigInt element.
impl TryFrom<Element> for RistrettoPoint {
    type Error = ConversionError;

    fn try_from(value: Element) -> Result<Self, Self::Error> {
        let (_, mut bytes) = value.0.to_bytes_le();
        bytes.resize(32, 0);
        let compressed = CompressedRistretto::from_slice(&bytes).map_err(|_| ConversionError)?;
        compressed.decompress().ok_or(ConversionError)
    }
}

/// Converts a BigInt element to a ristretto point.
impl From<RistrettoPoint> for Element {
    fn from(value: RistrettoPoint) -> Self {
        let v = value.compress().to_bytes();
        Element(BigInt::from_bytes_le(Sign::Plus, &v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
    use num_bigint::{BigUint, RandomBits};
    use num_traits::{One, Zero};
    use rand::Rng;

    fn generate_random_bigint() -> BigInt {
        let mut rng = rand::thread_rng();
        let num: BigUint = rng.sample(RandomBits::new(32));
        let signed_num: BigInt = num.clone().into();
        signed_num
    }

    #[test]
    fn test_generate_random_scalar() {
        let scalar = generate_random_scalar();
        let scalar: Scalar = scalar.into();
        assert!(scalar > Scalar::zero());
    }

    #[test]
    fn try_from_scalar_to_dalek_scalar() {
        let value: Scalar = generate_random_bigint().into();
        let scalar: DalekScalar = value.clone().try_into().unwrap();
        let scalar: Scalar = scalar.into();
        assert_eq!(value, scalar);
    }

    #[test]
    fn try_from_element_to_ristretto_point() {
        let value: Element = RISTRETTO_BASEPOINT_POINT.into();
        let element: RistrettoPoint = value.clone().try_into().unwrap();
        let element: Element = element.into();
        assert_eq!(value, element);
    }

    #[test]
    fn try_from_element_to_ristretto_point_error() {
        let value: Element = BigInt::one().into();
        let element: Result<RistrettoPoint, ConversionError> = value.clone().try_into();
        assert!(element.is_err());
    }
}
