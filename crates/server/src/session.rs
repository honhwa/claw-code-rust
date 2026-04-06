use std::path::PathBuf;

use chrono::{DateTime, Utc};
use clawcr_core::{SessionId, SessionTitleState};
use serde::{Deserialize, Serialize};

use crate::turn::TurnSummary;

/// Stores the runtime-level status of one session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionRuntimeStatus {
    /// The session is loaded with no active turn.
    Idle,
    /// The session currently owns one active turn.
    ActiveTurn,
    /// The session is waiting on client interaction such as approval.
    WaitingClient,
    /// The session is archived.
    Archived,
    /// The session is not loaded in memory.
    Unloaded,
}

/// Stores one session summary projected onto the server API surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionSummary {
    /// The stable session identifier.
    pub session_id: SessionId,
    /// The working directory associated with the session.
    pub cwd: PathBuf,
    /// The timestamp when the session was created.
    pub created_at: DateTime<Utc>,
    /// The timestamp of the last known session update.
    pub updated_at: DateTime<Utc>,
    /// The current best-known session title.
    pub title: Option<String>,
    /// The lifecycle state for the current title.
    pub title_state: SessionTitleState,
    /// Whether the session is ephemeral.
    pub ephemeral: bool,
    /// The latest resolved model slug for the session.
    pub resolved_model: Option<String>,
    /// The current runtime status visible to API clients.
    pub status: SessionRuntimeStatus,
}

/// Describes the payload for `session/start`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionStartParams {
    /// The working directory for the new session.
    pub cwd: PathBuf,
    /// Whether the session should be treated as ephemeral.
    pub ephemeral: bool,
    /// The explicit title to assign at creation time, if any.
    pub title: Option<String>,
    /// An optional requested model slug.
    pub model: Option<String>,
}

/// Describes the response returned by `session/start`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionStartResult {
    /// The created session identifier.
    pub session_id: SessionId,
    /// The session creation timestamp.
    pub created_at: DateTime<Utc>,
    /// The working directory assigned to the session.
    pub cwd: PathBuf,
    /// Whether the session is ephemeral.
    pub ephemeral: bool,
    /// The model resolved for the initial session state.
    pub resolved_model: Option<String>,
}

/// Describes the payload for `session/resume`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionResumeParams {
    /// The session identifier that should be resumed.
    pub session_id: SessionId,
}

/// Describes the response returned by `session/resume`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionResumeResult {
    /// The resumed session summary visible to the client.
    pub session: SessionSummary,
    /// The latest turn for the session, when one exists.
    pub latest_turn: Option<TurnSummary>,
    /// The number of items loaded while resuming the session.
    pub loaded_item_count: u64,
}

/// Describes the payload for `session/fork`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionForkParams {
    /// The source session that should be forked.
    pub session_id: SessionId,
    /// The explicit title for the forked session, when provided.
    pub title: Option<String>,
    /// The optional working-directory override for the fork.
    pub cwd: Option<PathBuf>,
}

/// Describes the response returned by `session/fork`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionForkResult {
    /// The forked session summary visible to the client.
    pub session: SessionSummary,
    /// The source session identifier.
    pub forked_from_session_id: SessionId,
}
