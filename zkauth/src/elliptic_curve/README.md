# Elliptic Curve Chaum-Pedersen Proof

Adapting the Chaum-Pedersen protocol to elliptic curves involves leveraging the elliptic curve discrete logarithm problem (ECDLP) instead of the classical discrete logarithm problem in a cyclic group. The fundamental principles remain similar, but the operations are adapted to the properties and operations of elliptic curves.

Here's how the steps adapt:

1. **Setup**: Instead of agreeing on a prime $p$ and a generator $g$ of a cyclic group, the prover and verifier agree on an elliptic curve $E$ defined over a finite field and a base point $G$ on $E$ of prime order $q$. The prover knows a secret scalar $x$, which corresponds to the discrete logarithm (with respect to base point $G$) of two points $Y_1 = xG$ and $Y_2 = xH$ on the elliptic curve, where $H$ is another point on the curve. The prover intends to demonstrate that $\log_G(Y_1) = \log_H(Y_2) = x$ without revealing $x$.
2. **Commitment**: The prover picks a random scalar $k$ from the set $1, ..., q-1$ and computes two commitment points $R_1 = kG$ and $R_2 = kH$ on the elliptic curve. These commitments $R_1$ and $R_2$ are then sent to the verifier.
3. **Challenge**: The verifier generates a random challenge scalar $c$ and sends it to the prover. This challenge is again a random scalar from the set $1, ..., q-1$.
4. **Response**: Upon receiving $c$, the prover calculates the response scalar $s = k + cx \mod q$ and sends $s$ back to the verifier.
5. **Verification**: The verifier receives $s$ and validates the prover’s claims by checking if $sG = R_1 + cY_1$ and $sH = R_2 + cY_2$ on the elliptic curve, or equivalently if $R_1 = sG - cY_1$ and $R_2 = sH - cY_2$. If both equations hold, the prover's claim is accepted; otherwise, it is rejected.

Adapting the protocol to elliptic curves maintains the privacy and security characteristics of the original Chaum-Pedersen protocol while leveraging the added security benefits and efficiency of elliptic curve cryptography, which typically allows for shorter key sizes compared to traditional discrete logarithm-based systems for a comparable level of security. The main changes involve moving from multiplicative group operations to additive elliptic curve group operations and from working with integers modulo a prime to working with points on an elliptic curve.
