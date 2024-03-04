pub mod client;

#[doc(inline)]
pub use zkauth_protobuf::v1::auth_client::AuthClient;

#[cfg(test)]
mod test;
