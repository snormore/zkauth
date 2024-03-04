use num_bigint::BigInt;

use super::operations::DiscreteLogarithmOperations;
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
    fn generate_registration_x(&self) -> BigInt {
        self.operations.generate_x()
    }

    fn compute_registration_x(&self, password: String) -> BigInt {
        self.operations.compute_x(password)
    }

    fn compute_registration_y1y2(&self, x: BigInt) -> (BigInt, BigInt) {
        let y1 = self.operations.compute_y1(x.clone());
        let y2 = self.operations.compute_y2(x);
        (y1, y2)
    }

    fn generate_challenge_k(&self) -> BigInt {
        self.operations.generate_k()
    }

    fn compute_challenge_commitment_r1r2(&self, k: BigInt) -> (BigInt, BigInt) {
        let r1 = self.operations.compute_r1(k.clone());
        let r2 = self.operations.compute_r2(k);
        (r1, r2)
    }

    fn compute_challenge_response_s(&self, x: BigInt, k: BigInt, c: BigInt) -> BigInt {
        self.operations.compute_s(x, k, c)
    }
}
