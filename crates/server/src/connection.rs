use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Describes the transport kind used by one connected client.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientTransportKind {
    /// The client is connected over stdio.
    Stdio,
    /// The client is connected over a WebSocket transport.
    WebSocket,
}

/// Stores the lifecycle state of one transport connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionState {
    /// The transport is connected but the handshake has not started yet.
    Connected,
    /// The transport is processing `initialize` but is not yet ready.
    Initializing,
    /// The transport completed `initialize` and `initialized`.
    Ready,
    /// The transport has closed.
    Closed,
}

/// Carries the data required by the initial `initialize` request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitializeParams {
    /// The human-readable client name.
    pub client_name: String,
    /// The client version string.
    pub client_version: String,
    /// The transport used by the client.
    pub transport: ClientTransportKind,
    /// Whether the client can consume streamed deltas.
    pub supports_streaming: bool,
    /// Whether the client supports binary image payloads directly.
    pub supports_binary_images: bool,
    /// Exact notification method names the client wants suppressed.
    pub opt_out_notification_methods: Vec<String>,
}

/// Carries the result returned by a successful `initialize` request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitializeResult {
    /// The server product name.
    pub server_name: String,
    /// The server version string.
    pub server_version: String,
    /// The operating-system family of the running server.
    pub platform_family: String,
    /// The operating-system identifier of the running server.
    pub platform_os: String,
    /// The server home directory.
    pub server_home: PathBuf,
    /// The capability flags supported by this server instance.
    pub capabilities: ServerCapabilities,
}

/// Advertises which runtime capabilities this server instance supports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerCapabilities {
    /// Whether `session/resume` is supported.
    pub session_resume: bool,
    /// Whether `session/fork` is supported.
    pub session_fork: bool,
    /// Whether `turn/interrupt` is supported.
    pub turn_interrupt: bool,
    /// Whether approval requests may be routed to clients.
    pub approval_requests: bool,
    /// Whether streaming events are emitted.
    pub event_streaming: bool,
}
