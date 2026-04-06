use clawcr_core::{SessionId, TurnId};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// Describes a client response to a pending approval request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalRespondParams {
    /// The session that owns the approval request.
    pub session_id: SessionId,
    /// The turn that owns the approval request.
    pub turn_id: TurnId,
    /// The approval request identifier being answered.
    pub approval_id: SmolStr,
    /// The decision selected by the client.
    pub decision: ApprovalDecisionValue,
    /// The scope associated with the decision.
    pub scope: ApprovalScopeValue,
}

/// Enumerates client decisions for approval requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDecisionValue {
    /// Approve the request.
    Approve,
    /// Deny the request.
    Deny,
    /// Cancel the request without granting it.
    Cancel,
}

/// Enumerates the scopes supported by approval responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalScopeValue {
    /// Apply the decision once.
    Once,
    /// Apply the decision for the remainder of the current turn.
    Turn,
    /// Apply the decision for the remainder of the session.
    Session,
    /// Apply the decision to a path-prefix resource scope.
    PathPrefix,
    /// Apply the decision to a host resource scope.
    Host,
    /// Apply the decision to a tool name scope.
    Tool,
}

/// Describes the payload for `events/subscribe`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventsSubscribeParams {
    /// The optional session filter for this subscription.
    pub session_id: Option<SessionId>,
    /// The optional exact event-type filter list.
    pub event_types: Option<Vec<String>>,
}

/// Describes the response returned by `events/subscribe`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventsSubscribeResult {
    /// The stable subscription identifier.
    pub subscription_id: SmolStr,
}
