use curve25519_dalek::{ristretto::CompressedRistretto, RistrettoPoint, Scalar as DalekScalar};
use num_bigint::{BigInt, Sign};

use crate::{ConversionError, Element, Scalar};

pub mod configuration;
pub mod prover;
pub mod verifier;

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
