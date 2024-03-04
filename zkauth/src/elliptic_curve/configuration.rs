use curve25519_dalek::{constants::RISTRETTO_BASEPOINT_POINT, RistrettoPoint, Scalar};
use sha2::{Digest, Sha512};

#[derive(Debug, Clone)]
/// Configuration for the discrete logarithm protocol.
pub struct EllipticCurveConfiguration {
    pub g: RistrettoPoint,
    pub h: RistrettoPoint,
}

impl EllipticCurveConfiguration {
    pub fn generate(prime_bits: usize) -> EllipticCurveConfiguration {
        let g = RISTRETTO_BASEPOINT_POINT;

        let h_value = "Unique value for H";
        let mut hasher = Sha512::new();
        hasher.update(h_value.as_bytes());
        let h_result = hasher.finalize();
        let h_bigint: [u8; 64] = *h_result.as_ref();
        let h = RistrettoPoint::from_uniform_bytes(&h_bigint);

        let mut rng = rand::thread_rng();
        let secret = Scalar::random(&mut rng);
        let point_g = g * secret;
        let point_h = h * secret;

        EllipticCurveConfiguration {
            g: point_g,
            h: point_h,
        }
    }
}
