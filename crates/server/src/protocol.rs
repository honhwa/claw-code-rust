use serde::{Deserialize, Serialize};

/// Carries one client-to-server request envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientRequest<T> {
    /// The stable request identifier.
    pub id: serde_json::Value,
    /// The exact request method.
    pub method: String,
    /// The typed request payload.
    pub params: T,
}

/// Carries one client-to-server notification envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientNotification<T> {
    /// The exact notification method.
    pub method: String,
    /// The typed notification payload.
    pub params: T,
}

/// Carries one successful response envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    /// The request identifier being answered.
    pub id: serde_json::Value,
    /// The typed result payload.
    pub result: T,
}

/// Carries one error response envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// The request identifier being answered.
    pub id: serde_json::Value,
    /// The protocol error payload.
    pub error: ProtocolError,
}

/// Carries one outbound notification envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationEnvelope<T> {
    /// The notification method name.
    pub method: String,
    /// The typed notification payload.
    pub params: T,
}

/// Carries one server-initiated request envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerRequestEnvelope<T> {
    /// The stable server-generated request identifier.
    pub id: serde_json::Value,
    /// The exact server-initiated request method.
    pub method: String,
    /// The typed request payload.
    pub params: T,
}

/// Enumerates the protocol errors exposed by the runtime server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
pub enum ProtocolErrorCode {
    /// The client issued a request before initialization completed.
    #[error("NotInitialized")]
    NotInitialized,
    /// The request payload was malformed or incomplete.
    #[error("InvalidParams")]
    InvalidParams,
    /// The requested session does not exist.
    #[error("SessionNotFound")]
    SessionNotFound,
    /// The requested turn does not exist.
    #[error("TurnNotFound")]
    TurnNotFound,
    /// Another turn is already active for the session.
    #[error("TurnAlreadyRunning")]
    TurnAlreadyRunning,
    /// The referenced approval request does not exist.
    #[error("ApprovalNotFound")]
    ApprovalNotFound,
    /// The action was denied by policy.
    #[error("PolicyDenied")]
    PolicyDenied,
    /// The request exceeded the active context budget.
    #[error("ContextLimitExceeded")]
    ContextLimitExceeded,
    /// No active turn exists for the request.
    #[error("NoActiveTurn")]
    NoActiveTurn,
    /// The expected active turn did not match the current turn.
    #[error("ExpectedTurnMismatch")]
    ExpectedTurnMismatch,
    /// The active turn kind does not support same-turn steering.
    #[error("ActiveTurnNotSteerable")]
    ActiveTurnNotSteerable,
    /// The request required non-empty input but none was provided.
    #[error("EmptyInput")]
    EmptyInput,
    /// An internal invariant or transport failure occurred.
    #[error("InternalError")]
    InternalError,
}

/// Carries a typed protocol error response payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolError {
    /// The stable protocol error code.
    pub code: ProtocolErrorCode,
    /// The human-readable error message.
    pub message: String,
    /// Optional structured error details.
    pub data: serde_json::Value,
}
