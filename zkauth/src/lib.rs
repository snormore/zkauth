use bytes::Bytes;
use std::fmt::Debug;

pub mod discrete_logarithm;
pub mod elliptic_curve;

pub trait Prover: Sync + Send + Debug {
    fn generate_registration_x(&self) -> Bytes;
    fn compute_registration_x(&self, password: String) -> Bytes;
    fn compute_registration_y1y2(&self, x: Bytes) -> (Bytes, Bytes);
    fn generate_challenge_k(&self) -> Bytes;
    fn compute_challenge_commitment_r1r2(&self, k: Bytes) -> (Bytes, Bytes);
    fn compute_challenge_response_s(&self, x: Bytes, k: Bytes, c: Bytes) -> Bytes;
}

pub trait Verifier: Sync + Send {
    fn generate_challenge_c(&self) -> Bytes;
    fn compute_verification_r1r2(&self, y1: Bytes, y2: Bytes, c: Bytes, s: Bytes)
        -> (Bytes, Bytes);
}

pub trait Operations<Element, Scalar> {
    // TODO: rename Element to something better

    fn compute_x(&self, password: String) -> Scalar;
    fn generate_x(&self) -> Scalar;

    fn compute_y1(&self, x: Scalar) -> Element;
    fn compute_y2(&self, x: Scalar) -> Element;

    fn generate_k(&self) -> Scalar;

    fn compute_r1(&self, k: Scalar) -> Element;
    fn compute_r2(&self, k: Scalar) -> Element;

    fn generate_c(&self) -> Scalar;

    fn compute_s(&self, x: Scalar, k: Scalar, c: Scalar) -> Scalar;

    fn compute_vr1(&self, y1: Element, c: Scalar, s: Scalar) -> Element;
    fn compute_vr2(&self, y2: Element, c: Scalar, s: Scalar) -> Element;
}
