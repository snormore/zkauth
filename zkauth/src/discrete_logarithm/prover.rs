use bytes::Bytes;
use num_bigint::BigInt;

use super::{bigint_to_bytes, bytes_to_bigint, operations::DiscreteLogarithmOperations};
use crate::{Operations, Prover};

#[derive(Debug)]
pub struct DiscreteLogarithmProver {
    operations: DiscreteLogarithmOperations,
}

impl DiscreteLogarithmProver {
    pub fn new(p: BigInt, q: BigInt, g: BigInt, h: BigInt) -> Self {
        DiscreteLogarithmProver {
            operations: DiscreteLogarithmOperations::new(p, q, g, h),
        }
    }
}

impl Prover for DiscreteLogarithmProver {
    fn generate_registration_x(&self) -> Bytes {
        let x = self.operations.generate_x();
        bigint_to_bytes(x)
    }

    fn compute_registration_x(&self, password: String) -> Bytes {
        let x = self.operations.compute_x(password);
        bigint_to_bytes(x)
    }

    fn compute_registration_y1y2(&self, x: Bytes) -> (Bytes, Bytes) {
        let x = bytes_to_bigint(x);
        // TODO: fix this clone
        let y1 = self.operations.compute_y1(x.clone());
        let y2 = self.operations.compute_y2(x);
        (bigint_to_bytes(y1), bigint_to_bytes(y2))
    }

    fn generate_challenge_k(&self) -> Bytes {
        let c = self.operations.generate_k();
        bigint_to_bytes(c)
    }

    fn compute_challenge_commitment_r1r2(&self, k: Bytes) -> (Bytes, Bytes) {
        let k = bytes_to_bigint(k);
        // TODO: fix this clone
        let r1 = self.operations.compute_r1(k.clone());
        let r2 = self.operations.compute_r2(k);
        (bigint_to_bytes(r1), bigint_to_bytes(r2))
    }

    fn compute_challenge_response_s(&self, x: Bytes, k: Bytes, c: Bytes) -> Bytes {
        let x = bytes_to_bigint(x);
        let k = bytes_to_bigint(k);
        let c = bytes_to_bigint(c);
        let s = self.operations.compute_s(x, k, c);
        bigint_to_bytes(s)
    }
}
