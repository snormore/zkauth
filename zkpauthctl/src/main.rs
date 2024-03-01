use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use env_logger::Env;
use num_bigint::{BigInt, BigUint, RandomBits};
use num_traits::Zero;
use rand::Rng;
use zkpauthpb::v1::{
    auth_client::AuthClient, AuthenticationAnswerRequest, AuthenticationChallengeRequest,
    GetPublicParametersRequest, RegisterRequest,
};

const RANDOM_NONCE_LENGTH_BITS: u64 = 32;
const RANDOM_SECRET_LENGTH_BITS: u64 = 256;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,

    /// Specifies the address of the gRPC server to connect to.
    #[arg(short, long, default_value = "http://127.0.0.1:50001")]
    address: String,
}

impl Options {
    fn init_logger(&self) {
        if self.verbose.is_present() {
            env_logger::Builder::new()
                .filter_level(self.verbose.log_level_filter())
                .init();
        } else {
            env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Options::parse();
    opts.init_logger();

    let mut rng = rand::thread_rng();

    let mut client = AuthClient::connect(opts.address).await?;

    let params = client
        .get_public_parameters(GetPublicParametersRequest {})
        .await?
        .into_inner();
    let p = params.p.parse::<BigInt>().unwrap();
    let q = params.q.parse::<BigInt>().unwrap();
    let g = params.g.parse::<BigInt>().unwrap();
    let h = params.h.parse::<BigInt>().unwrap();

    // Registration.

    // Generate random secret number x.
    // Should not be negative because it's used as an exponent.
    let x: BigUint = rng.sample(RandomBits::new(RANDOM_SECRET_LENGTH_BITS));
    let signed_x: BigInt = x.clone().into();

    // Compute y1 and y2 for registration.
    // https://github.com/neongazer/zkp-auth-py/blob/main/zkp_auth/sigma_protocols/chaum_pedersen/prover.py#L42-L50
    let y1 = g.modpow(&signed_x, &p);
    let y2 = h.modpow(&signed_x, &p);
    log::info!("y1 = {}", y1);
    log::info!("y2 = {}", y2);

    // Send register request.
    let user = "foo".to_string();
    let resp = client
        .register(RegisterRequest {
            user: user.clone(),
            y1: y1.to_string(),
            y2: y2.to_string(),
        })
        .await?
        .into_inner();

    log::info!("{:?}", resp);

    // Login.

    // Generate random number k.
    // Should not be negative because it's used as an exponent.
    let k: BigUint = rng.sample(RandomBits::new(RANDOM_NONCE_LENGTH_BITS));
    let signed_k: BigInt = k.clone().into();

    // Compute commitment (r1, r2) for authentication challenge.
    // https://github.com/neongazer/zkp-auth-py/blob/main/zkp_auth/sigma_protocols/chaum_pedersen/prover.py#L52-L60
    let r1 = g.modpow(&signed_k, &p);
    let r2 = h.modpow(&signed_k, &p);
    log::info!("r1 = {}", r1);
    log::info!("r2 = {}", r2);

    // Send create_authentication_challenge request.
    let resp = client
        .create_authentication_challenge(AuthenticationChallengeRequest {
            user: user.clone(),
            r1: r1.to_string(),
            r2: r2.to_string(),
        })
        .await?
        .into_inner();
    let c = resp.c.parse::<BigInt>().unwrap();

    log::info!("{:?}", resp);

    // Compute challenge response s.
    // Should not be negative because it's used as an exponent.
    // https://github.com/neongazer/zkp-auth-py/blob/main/zkp_auth/sigma_protocols/chaum_pedersen/prover.py#L62-L68
    let mut s = (signed_k - c * signed_x) % &q;
    if s < Zero::zero() {
        s += q;
    }
    log::info!("s = {}", s);

    // Send verify_authentication request.
    let resp = client
        .verify_authentication(AuthenticationAnswerRequest {
            auth_id: resp.auth_id,
            s: s.to_string(),
        })
        .await?
        .into_inner();

    log::info!("{:?}", resp);

    Ok(())
}
