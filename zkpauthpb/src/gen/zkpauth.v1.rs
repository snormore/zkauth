// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterRequest {
    #[prost(string, tag="1")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub y1: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub y2: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationChallengeRequest {
    #[prost(string, tag="1")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub r1: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub r2: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationChallengeResponse {
    #[prost(string, tag="1")]
    pub auth_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub c: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationAnswerRequest {
    #[prost(string, tag="1")]
    pub auth_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub s: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationAnswerResponse {
    #[prost(string, tag="1")]
    pub session_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPublicParametersRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPublicParametersResponse {
    #[prost(string, tag="1")]
    pub p: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub q: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub g: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub h: ::prost::alloc::string::String,
}
/// Encoded file descriptor set for the `zkpauth.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xa6, 0x12, 0x0a, 0x08, 0x76, 0x31, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x7a,
    0x6b, 0x70, 0x61, 0x75, 0x74, 0x68, 0x2e, 0x76, 0x31, 0x22, 0x45, 0x0a, 0x0f, 0x52, 0x65, 0x67,
    0x69, 0x73, 0x74, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04,
    0x75, 0x73, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x75, 0x73, 0x65, 0x72,
    0x12, 0x0e, 0x0a, 0x02, 0x79, 0x31, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x79, 0x31,
    0x12, 0x0e, 0x0a, 0x02, 0x79, 0x32, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x79, 0x32,
    0x22, 0x12, 0x0a, 0x10, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x22, 0x54, 0x0a, 0x1e, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69,
    0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x75, 0x73, 0x65, 0x72, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x75, 0x73, 0x65, 0x72, 0x12, 0x0e, 0x0a, 0x02, 0x72, 0x31,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x72, 0x31, 0x12, 0x0e, 0x0a, 0x02, 0x72, 0x32,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x72, 0x32, 0x22, 0x48, 0x0a, 0x1f, 0x41, 0x75,
    0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x68, 0x61, 0x6c,
    0x6c, 0x65, 0x6e, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x17, 0x0a,
    0x07, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06,
    0x61, 0x75, 0x74, 0x68, 0x49, 0x64, 0x12, 0x0c, 0x0a, 0x01, 0x63, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x01, 0x63, 0x22, 0x44, 0x0a, 0x1b, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69,
    0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x41, 0x6e, 0x73, 0x77, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x17, 0x0a, 0x07, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61, 0x75, 0x74, 0x68, 0x49, 0x64, 0x12, 0x0c, 0x0a, 0x01,
    0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x01, 0x73, 0x22, 0x3d, 0x0a, 0x1c, 0x41, 0x75,
    0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x41, 0x6e, 0x73, 0x77,
    0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65,
    0x73, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09,
    0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x22, 0x1c, 0x0a, 0x1a, 0x47, 0x65, 0x74,
    0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x55, 0x0a, 0x1b, 0x47, 0x65, 0x74, 0x50, 0x75,
    0x62, 0x6c, 0x69, 0x63, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x0c, 0x0a, 0x01, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x01, 0x70, 0x12, 0x0c, 0x0a, 0x01, 0x71, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x01, 0x71, 0x12, 0x0c, 0x0a, 0x01, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x01, 0x67,
    0x12, 0x0c, 0x0a, 0x01, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x01, 0x68, 0x32, 0xa2,
    0x03, 0x0a, 0x04, 0x41, 0x75, 0x74, 0x68, 0x12, 0x68, 0x0a, 0x13, 0x47, 0x65, 0x74, 0x50, 0x75,
    0x62, 0x6c, 0x69, 0x63, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x12, 0x26,
    0x2e, 0x7a, 0x6b, 0x70, 0x61, 0x75, 0x74, 0x68, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x50,
    0x75, 0x62, 0x6c, 0x69, 0x63, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x27, 0x2e, 0x7a, 0x6b, 0x70, 0x61, 0x75, 0x74, 0x68,
    0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x50, 0x61, 0x72,
    0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22,
    0x00, 0x12, 0x47, 0x0a, 0x08, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x12, 0x1b, 0x2e,
    0x7a, 0x6b, 0x70, 0x61, 0x75, 0x74, 0x68, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x73,
    0x74, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x1c, 0x2e, 0x7a, 0x6b, 0x70,
    0x61, 0x75, 0x74, 0x68, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x12, 0x7a, 0x0a, 0x1d, 0x43, 0x72,
    0x65, 0x61, 0x74, 0x65, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x12, 0x2a, 0x2e, 0x7a, 0x6b,
    0x70, 0x61, 0x75, 0x74, 0x68, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74,
    0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x2b, 0x2e, 0x7a, 0x6b, 0x70, 0x61, 0x75, 0x74,
    0x68, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x12, 0x6b, 0x0a, 0x14, 0x56, 0x65, 0x72, 0x69, 0x66, 0x79,
    0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x27,
    0x2e, 0x7a, 0x6b, 0x70, 0x61, 0x75, 0x74, 0x68, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x75, 0x74, 0x68,
    0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x41, 0x6e, 0x73, 0x77, 0x65, 0x72,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x28, 0x2e, 0x7a, 0x6b, 0x70, 0x61, 0x75, 0x74,
    0x68, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x41, 0x6e, 0x73, 0x77, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x22, 0x00, 0x42, 0x62, 0x0a, 0x0e, 0x63, 0x6f, 0x6d, 0x2e, 0x7a, 0x6b, 0x70, 0x61, 0x75,
    0x74, 0x68, 0x2e, 0x76, 0x31, 0x42, 0x07, 0x56, 0x31, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01,
    0xa2, 0x02, 0x03, 0x5a, 0x58, 0x58, 0xaa, 0x02, 0x0a, 0x5a, 0x6b, 0x70, 0x61, 0x75, 0x74, 0x68,
    0x2e, 0x56, 0x31, 0xca, 0x02, 0x0a, 0x5a, 0x6b, 0x70, 0x61, 0x75, 0x74, 0x68, 0x5c, 0x56, 0x31,
    0xe2, 0x02, 0x16, 0x5a, 0x6b, 0x70, 0x61, 0x75, 0x74, 0x68, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50,
    0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x0b, 0x5a, 0x6b, 0x70, 0x61,
    0x75, 0x74, 0x68, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0x87, 0x0a, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x2d, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x01, 0x00, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x03, 0x00,
    0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x17, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x04, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x04, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x04, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x05,
    0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x05, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x05, 0x09, 0x0b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x05, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x06, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x06, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x06, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x06, 0x0e, 0x0f, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x03, 0x09, 0x00, 0x1b, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x09, 0x08, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02,
    0x12, 0x04, 0x0b, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0b,
    0x08, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0c, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x0d, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x0d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d,
    0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x0e, 0x0f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0e, 0x02, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x0e, 0x0e, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x11,
    0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x11, 0x08, 0x27, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x12, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x12, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x12, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03,
    0x13, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x13, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x13, 0x09, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x13, 0x0d, 0x0e, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x16, 0x00, 0x19, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01,
    0x12, 0x03, 0x16, 0x08, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x17,
    0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x17, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17, 0x09, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x17, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x18, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x18, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x18, 0x09, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x18, 0x0d, 0x0e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x1b, 0x00, 0x1d, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x1c, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x1c, 0x09, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1c,
    0x16, 0x17, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x03, 0x1f, 0x00, 0x25, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12,
    0x04, 0x21, 0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x21, 0x08,
    0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x22, 0x02, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x22, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x09, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x22, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01,
    0x12, 0x03, 0x23, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x23, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x23, 0x09,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x23, 0x0d, 0x0e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x24, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x02, 0x05, 0x12, 0x03, 0x24, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x24, 0x09, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x24, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x03, 0x12, 0x03,
    0x25, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x05, 0x12, 0x03, 0x25, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03, 0x25, 0x09, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x03, 0x25, 0x0d, 0x0e, 0x0a, 0x0a, 0x0a,
    0x02, 0x06, 0x00, 0x12, 0x04, 0x28, 0x00, 0x2d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01,
    0x12, 0x03, 0x28, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x29,
    0x02, 0x5e, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x06, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x29, 0x1a, 0x34, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x3f, 0x5a, 0x0a, 0x0b, 0x0a, 0x04,
    0x06, 0x00, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x02, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x2a, 0x06, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02,
    0x12, 0x03, 0x2a, 0x0f, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x2a, 0x29, 0x39, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x02, 0x70,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x06, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x24, 0x42, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2b, 0x4d, 0x6c, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x2c, 0x02, 0x61, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x2c, 0x06, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03,
    0x2c, 0x1b, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2c, 0x41,
    0x5d, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("zkpauth.v1.serde.rs");
include!("zkpauth.v1.tonic.rs");
// @@protoc_insertion_point(module)