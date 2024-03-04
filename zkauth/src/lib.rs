use anyhow::Result;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::fmt;
use std::fmt::Debug;
use std::ops::{Add, Mul};
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

impl Add for Scalar {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Scalar(self.0 + other.0)
    }
}

impl<'a> Add for &'a Scalar {
    type Output = Scalar;

    fn add(self, other: Self) -> Self::Output {
        Scalar(&self.0 + &other.0)
    }
}

impl Mul for Scalar {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Scalar(self.0 * other.0)
    }
}

impl<'a> Mul for &'a Scalar {
    type Output = Scalar;

    fn mul(self, other: Self) -> Self::Output {
        Scalar(&self.0 * &other.0)
    }
}

impl Add for Element {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Element(self.0 + other.0)
    }
}

impl<'a> Add for &'a Element {
    type Output = Element;

    fn add(self, other: Self) -> Self::Output {
        Element(&self.0 + &other.0)
    }
}

impl Mul for Element {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Element(self.0 * other.0)
    }
}

impl<'a> Mul for &'a Element {
    type Output = Element;

    fn mul(self, other: Self) -> Self::Output {
        Element(&self.0 * &other.0)
    }
}

impl Zero for Scalar {
    fn zero() -> Self {
        Scalar(BigInt::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl One for Scalar {
    fn one() -> Self {
        Scalar(BigInt::one())
    }
}

impl Zero for Element {
    fn zero() -> Self {
        Element(BigInt::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl One for Element {
    fn one() -> Self {
        Element(BigInt::one())
    }
}
