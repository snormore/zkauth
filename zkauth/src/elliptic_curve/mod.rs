use curve25519_dalek::{ristretto::CompressedRistretto, RistrettoPoint, Scalar as DalekScalar};
use num_bigint::{BigInt, Sign};

use crate::{Element, Scalar};

pub mod configuration;
pub mod prover;
pub mod verifier;

fn generate_random_scalar() -> DalekScalar {
    let mut rng = rand::thread_rng();
    DalekScalar::random(&mut rng)
}

impl From<Scalar> for DalekScalar {
    fn from(value: Scalar) -> Self {
        let (_, mut bytes) = value.0.to_bytes_le();
        bytes.resize(32, 0);
        // TODO: fix these hard unwraps
        DalekScalar::from_canonical_bytes(bytes.try_into().unwrap()).unwrap()
    }
}

impl From<DalekScalar> for Scalar {
    fn from(value: DalekScalar) -> Self {
        let v = value.to_bytes();
        Scalar(BigInt::from_bytes_le(Sign::Plus, &v))
    }
}

impl From<Element> for RistrettoPoint {
    fn from(value: Element) -> Self {
        let (_, mut bytes) = value.0.to_bytes_le();
        bytes.resize(32, 0);
        // TODO: fix these hard unwraps
        let compressed = CompressedRistretto::from_slice(&bytes).unwrap();
        compressed.decompress().unwrap()
    }
}

impl From<RistrettoPoint> for Element {
    fn from(value: RistrettoPoint) -> Self {
        let v = value.compress().to_bytes();
        Element(BigInt::from_bytes_le(Sign::Plus, &v))
    }
}
