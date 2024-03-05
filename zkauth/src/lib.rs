// Enforce documentation for all public items in the crate.
#![warn(missing_docs)]

//! This crate provides a set of tools for zero-knowledge proof authentication using
//! Chaum-Pedersen zero-knowledge proofs. It implements two flavors of the Chaum-Pedersen
//! cryptographic proofs; discrete logarithms and elliptive curves. These mechanisms allow a prover
//! to demonstrate knowledge of a secret corresponding to a public value without revealing the
//! secret itself.

use anyhow::Result;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::fmt;
use std::fmt::Debug;
use std::ops::{Add, Mul};
use std::str::FromStr;

/// The discrete logarithm module.
pub mod discrete_logarithm;

/// The elliptic curve module.
pub mod elliptic_curve;

/// A scalar value.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Scalar(pub BigInt);

/// An element value.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Element(pub BigInt);

/// An error for when a conversion fails.
#[derive(Debug)]
pub struct ConversionError;

/// A trait for implementing a prover for a zero-knowledge proof.
pub trait Prover: Sync + Send + Debug {
    /// Generates a registration x value.
    fn generate_registration_x(&self) -> Scalar;

    /// Computes a registration x value from the given password.
    fn compute_registration_x(&self, password: String) -> Scalar;

    /// Computes a registration y1 and y2 value from the given x.
    fn compute_registration_y1y2(&self, x: Scalar) -> Result<(Element, Element)>;

    /// Generates a challenge k value.
    fn generate_challenge_k(&self) -> Scalar;

    /// Computes a challenge commitment r1 and r2 value from the given k.
    fn compute_challenge_commitment_r1r2(&self, k: Scalar) -> Result<(Element, Element)>;

    /// Computes a challenge response s value from the given x, k, and c.
    fn compute_challenge_response_s(&self, x: Scalar, k: Scalar, c: Scalar) -> Result<Scalar>;
}

/// A trait for implementing a verifier for a zero-knowledge proof.
pub trait Verifier: Sync + Send {
    /// Generates a challenge c value.
    fn generate_challenge_c(&self) -> Scalar;

    /// Computes a verification r1 and r2 value from the given y1, y2, c, and s.
    fn compute_verification_r1r2(
        &self,
        y1: Element,
        y2: Element,
        c: Scalar,
        s: Scalar,
    ) -> Result<(Element, Element)>;
}

/// Converts a BigInt to a Scalar.
impl From<BigInt> for Scalar {
    fn from(value: BigInt) -> Self {
        Scalar(value)
    }
}

/// Converts a Scalar to a BigInt.
impl From<Scalar> for BigInt {
    fn from(value: Scalar) -> Self {
        value.0
    }
}

/// Converts a BigInt to an Element.
impl From<BigInt> for Element {
    fn from(value: BigInt) -> Self {
        Element(value)
    }
}

/// Converts an Element to a BigInt.
impl From<Element> for BigInt {
    fn from(value: Element) -> Self {
        value.0
    }
}

/// Converts a string to a Scalar.
impl FromStr for Scalar {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigInt::from_str(s).map(Scalar).map_err(|_| ConversionError)
    }
}

/// Converts a string to an Element.
impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Converts a string to an Element.
impl FromStr for Element {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigInt::from_str(s)
            .map(Element)
            .map_err(|_| ConversionError)
    }
}

/// Converts an Element to a string.
impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Implements the Add trait for Scalar.
impl Add for Scalar {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Scalar(self.0 + other.0)
    }
}

/// Implements the Add trait for &Scalar.
impl<'a> Add for &'a Scalar {
    type Output = Scalar;

    fn add(self, other: Self) -> Self::Output {
        Scalar(&self.0 + &other.0)
    }
}

/// Implements the Mul trait for Scalar.
impl Mul for Scalar {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Scalar(self.0 * other.0)
    }
}

/// Implements the Mul trait for &Scalar.
impl<'a> Mul for &'a Scalar {
    type Output = Scalar;

    fn mul(self, other: Self) -> Self::Output {
        Scalar(&self.0 * &other.0)
    }
}

/// Implements the Add trait for Element.
impl Add for Element {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Element(self.0 + other.0)
    }
}

/// Implements the Add trait for &Element.
impl<'a> Add for &'a Element {
    type Output = Element;

    fn add(self, other: Self) -> Self::Output {
        Element(&self.0 + &other.0)
    }
}

/// Implements the Mul trait for Element.
impl Mul for Element {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Element(self.0 * other.0)
    }
}

/// Implements the Mul trait for &Element.
impl<'a> Mul for &'a Element {
    type Output = Element;

    fn mul(self, other: Self) -> Self::Output {
        Element(&self.0 * &other.0)
    }
}

/// Implements the Zero trait for Scalar.
impl Zero for Scalar {
    fn zero() -> Self {
        Scalar(BigInt::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

/// Implements the One trait for Scalar.
impl One for Scalar {
    fn one() -> Self {
        Scalar(BigInt::one())
    }
}

/// Implements the Zero trait for Element.
impl Zero for Element {
    fn zero() -> Self {
        Element(BigInt::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

/// Implements the One trait for Element.
impl One for Element {
    fn one() -> Self {
        Element(BigInt::one())
    }
}
