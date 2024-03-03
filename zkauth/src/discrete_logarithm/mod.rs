use bytes::Bytes;
use num_bigint::{BigInt, Sign};

pub mod prover;
pub mod verifier;

mod operations;

fn bytes_to_bigint(v: Bytes) -> BigInt {
    BigInt::from_bytes_be(Sign::Plus, &v)
}

fn bigint_to_bytes(v: BigInt) -> Bytes {
    let (_, vec) = v.to_bytes_be();
    Bytes::from(vec)
}
