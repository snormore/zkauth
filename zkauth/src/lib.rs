use anyhow::Result;
use num_bigint::BigInt;
use std::fmt;
use std::fmt::Debug;
use std::str::FromStr;

pub mod discrete_logarithm;
pub mod elliptic_curve;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Scalar(pub BigInt);

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Element(pub BigInt);

#[derive(Debug)]
pub struct ConversionError;

pub trait Prover: Sync + Send + Debug {
    fn generate_registration_x(&self) -> Scalar;
    fn compute_registration_x(&self, password: String) -> Scalar;
    fn compute_registration_y1y2(&self, x: Scalar) -> Result<(Element, Element)>;
    fn generate_challenge_k(&self) -> Scalar;
    fn compute_challenge_commitment_r1r2(&self, k: Scalar) -> Result<(Element, Element)>;
    fn compute_challenge_response_s(&self, x: Scalar, k: Scalar, c: Scalar) -> Result<Scalar>;
}

pub trait Verifier: Sync + Send {
    fn generate_challenge_c(&self) -> Scalar;
    fn compute_verification_r1r2(
        &self,
        y1: Element,
        y2: Element,
        c: Scalar,
        s: Scalar,
    ) -> Result<(Element, Element)>;
}

impl From<BigInt> for Scalar {
    fn from(value: BigInt) -> Self {
        Scalar(value)
    }
}

impl From<Scalar> for BigInt {
    fn from(value: Scalar) -> Self {
        value.0
    }
}

impl From<BigInt> for Element {
    fn from(value: BigInt) -> Self {
        Element(value)
    }
}

impl From<Element> for BigInt {
    fn from(value: Element) -> Self {
        value.0
    }
}

impl FromStr for Scalar {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigInt::from_str(s).map(Scalar).map_err(|_| ConversionError)
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Element {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigInt::from_str(s)
            .map(Element)
            .map_err(|_| ConversionError)
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
