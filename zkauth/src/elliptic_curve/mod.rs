use curve25519_dalek::{ristretto::CompressedRistretto, RistrettoPoint, Scalar};
use num_bigint::{BigInt, Sign};

pub mod prover;
pub mod verifier;

mod operations;

fn bigint_to_scalar(v: BigInt) -> Scalar {
    let (_, mut bytes) = v.to_bytes_le();
    bytes.resize(32, 0);
    Scalar::from_canonical_bytes(bytes.try_into().unwrap()).unwrap()
}

fn scalar_to_bigint(v: Scalar) -> BigInt {
    let v = v.to_bytes();
    BigInt::from_bytes_le(Sign::Plus, &v)
}

pub fn bigint_to_ristretto_point(v: BigInt) -> RistrettoPoint {
    let (_, mut bytes) = v.to_bytes_le();
    bytes.resize(32, 0);
    let compressed = CompressedRistretto::from_slice(&bytes).unwrap();
    compressed.decompress().unwrap()
}

pub fn ristretto_point_to_bigint(v: RistrettoPoint) -> BigInt {
    let v = v.compress().to_bytes();
    BigInt::from_bytes_le(Sign::Plus, &v)
}
