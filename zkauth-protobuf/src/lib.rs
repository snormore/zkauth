//! This module contains the generated protobuf code for the zkauth protocol and grpc service.

use v1::{configuration, Configuration};
use zkauth::{
    discrete_logarithm::configuration::DiscreteLogarithmConfiguration,
    elliptic_curve::configuration::EllipticCurveConfiguration, Element, Scalar,
};

include!("gen/mod.rs");

#[derive(Debug)]
pub struct ConversionError;

impl From<DiscreteLogarithmConfiguration> for Configuration {
    fn from(value: DiscreteLogarithmConfiguration) -> Self {
        let p: Scalar = value.p.into();
        let q: Scalar = value.q.into();
        let g: Element = value.g.into();
        let h: Element = value.h.into();
        Configuration {
            flavor: Some(configuration::Flavor::DiscreteLogarithm(
                configuration::DiscreteLogarithm {
                    p: p.to_string(),
                    q: q.to_string(),
                    g: g.to_string(),
                    h: h.to_string(),
                },
            )),
        }
    }
}

impl From<EllipticCurveConfiguration> for Configuration {
    fn from(value: EllipticCurveConfiguration) -> Self {
        let g: Element = value.g.into();
        let h: Element = value.h.into();
        Configuration {
            flavor: Some(configuration::Flavor::EllipticCurve(
                configuration::EllipticCurve {
                    g: g.to_string(),
                    h: h.to_string(),
                },
            )),
        }
    }
}

impl TryFrom<configuration::DiscreteLogarithm> for DiscreteLogarithmConfiguration {
    type Error = ConversionError;

    fn try_from(config: configuration::DiscreteLogarithm) -> Result<Self, Self::Error> {
        Ok(DiscreteLogarithmConfiguration {
            p: config.p.parse().map_err(|_| ConversionError)?,
            q: config.q.parse().map_err(|_| ConversionError)?,
            g: config.g.parse().map_err(|_| ConversionError)?,
            h: config.h.parse().map_err(|_| ConversionError)?,
        })
    }
}

impl TryFrom<configuration::EllipticCurve> for EllipticCurveConfiguration {
    type Error = ConversionError;

    fn try_from(config: configuration::EllipticCurve) -> Result<Self, Self::Error> {
        Ok(EllipticCurveConfiguration {
            g: config
                .g
                .parse::<Element>()
                .map_err(|_| ConversionError)?
                .try_into()
                .map_err(|_| ConversionError)?,
            h: config
                .h
                .parse::<Element>()
                .map_err(|_| ConversionError)?
                .try_into()
                .map_err(|_| ConversionError)?,
        })
    }
}
