use std::fmt;

use tonic::{Code, Status};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImRpcError {
    kind: ImRpcErrorKind,
    message: String,
}

impl ImRpcError {
    pub fn invalid_argument(message: impl Into<String>) -> Self {
        Self::new(ImRpcErrorKind::InvalidArgument, message)
    }

    pub fn unauthenticated(message: impl Into<String>) -> Self {
        Self::new(ImRpcErrorKind::Unauthenticated, message)
    }

    pub fn permission_denied(message: impl Into<String>) -> Self {
        Self::new(ImRpcErrorKind::PermissionDenied, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ImRpcErrorKind::NotFound, message)
    }

    pub fn already_exists(message: impl Into<String>) -> Self {
        Self::new(ImRpcErrorKind::AlreadyExists, message)
    }

    pub fn failed_precondition(message: impl Into<String>) -> Self {
        Self::new(ImRpcErrorKind::FailedPrecondition, message)
    }

    pub fn resource_exhausted(message: impl Into<String>) -> Self {
        Self::new(ImRpcErrorKind::ResourceExhausted, message)
    }

    pub fn unavailable(message: impl Into<String>) -> Self {
        Self::new(ImRpcErrorKind::Unavailable, message)
    }

    pub fn unimplemented(message: impl Into<String>) -> Self {
        Self::new(ImRpcErrorKind::Unimplemented, message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ImRpcErrorKind::Internal, message)
    }

    pub fn decode(message: impl Into<String>) -> Self {
        Self::new(ImRpcErrorKind::InvalidArgument, message)
    }

    pub fn code(&self) -> Code {
        match self.kind {
            ImRpcErrorKind::InvalidArgument => Code::InvalidArgument,
            ImRpcErrorKind::Unauthenticated => Code::Unauthenticated,
            ImRpcErrorKind::PermissionDenied => Code::PermissionDenied,
            ImRpcErrorKind::NotFound => Code::NotFound,
            ImRpcErrorKind::AlreadyExists => Code::AlreadyExists,
            ImRpcErrorKind::FailedPrecondition => Code::FailedPrecondition,
            ImRpcErrorKind::ResourceExhausted => Code::ResourceExhausted,
            ImRpcErrorKind::Unavailable => Code::Unavailable,
            ImRpcErrorKind::Unimplemented => Code::Unimplemented,
            ImRpcErrorKind::Internal => Code::Internal,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    fn new(kind: ImRpcErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for ImRpcError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ImRpcError {}

impl From<prost::EncodeError> for ImRpcError {
    fn from(error: prost::EncodeError) -> Self {
        Self::internal(format!("failed to encode RPC response: {error}"))
    }
}

impl From<prost::DecodeError> for ImRpcError {
    fn from(error: prost::DecodeError) -> Self {
        Self::decode(format!("failed to decode RPC payload: {error}"))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ImRpcErrorKind {
    InvalidArgument,
    Unauthenticated,
    PermissionDenied,
    NotFound,
    AlreadyExists,
    FailedPrecondition,
    ResourceExhausted,
    Unavailable,
    Unimplemented,
    Internal,
}

pub fn map_rpc_error_to_status(error: ImRpcError) -> Status {
    Status::new(error.code(), error.message)
}
