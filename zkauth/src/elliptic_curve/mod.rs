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
