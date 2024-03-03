use bytes::Bytes;
use curve25519_dalek::{ristretto::CompressedRistretto, RistrettoPoint, Scalar};

pub mod prover;
pub mod verifier;

mod operations;

fn bytes_to_scalar(v: Bytes) -> Scalar {
    // if x.len() != 32 {}
    // TODO: make sure length is 32
    // TODO: fix these hard unwraps
    let v = &v[..];
    let v = v.try_into().unwrap();
    let v = Scalar::from_canonical_bytes(v).unwrap();
    v
}

fn scalar_to_bytes(v: Scalar) -> Bytes {
    let v = v.to_bytes();
    Bytes::copy_from_slice(&v)
}

fn bytes_to_ristretto_point(v: Bytes) -> RistrettoPoint {
    // if x.len() != 32 {}
    // TODO: make sure length is 32
    // TODO: fix these hard unwraps
    let v: [u8; 32] = v.as_ref().try_into().unwrap();
    let v = CompressedRistretto(v).decompress().unwrap();
    v
}

fn ristretto_point_to_bytes(v: RistrettoPoint) -> Bytes {
    let v = v.compress().to_bytes();
    Bytes::copy_from_slice(&v)
}
