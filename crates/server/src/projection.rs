use clawcr_core::{SessionRecord, TurnRecord};

use crate::session::{SessionRuntimeStatus, SessionSummary};
use crate::turn::TurnSummary;

/// Projects a canonical core session record into the API-visible session summary.
pub trait SessionProjector {
    /// Converts one core session record into a transport-facing session summary.
    fn project_session(
        &self,
        session: &SessionRecord,
        ephemeral: bool,
        status: SessionRuntimeStatus,
    ) -> SessionSummary;
}

/// Projects a canonical core turn record into the API-visible turn summary.
pub trait TurnProjector {
    /// Converts one core turn record into a transport-facing turn summary.
    fn project_turn(&self, turn: &TurnRecord) -> TurnSummary;
}

/// Default projector that performs field-by-field protocol projection.
#[derive(Debug, Clone, Default)]
pub struct DefaultProjection;

impl SessionProjector for DefaultProjection {
    fn project_session(
        &self,
        session: &SessionRecord,
        ephemeral: bool,
        status: SessionRuntimeStatus,
    ) -> SessionSummary {
        SessionSummary {
            session_id: session.id,
            cwd: session.cwd.clone(),
            created_at: session.created_at,
            updated_at: session.updated_at,
            title: session.title.clone(),
            title_state: session.title_state.clone(),
            ephemeral,
            resolved_model: session.model.clone(),
            status,
        }
    }
}

impl TurnProjector for DefaultProjection {
    fn project_turn(&self, turn: &TurnRecord) -> TurnSummary {
        TurnSummary {
            turn_id: turn.id,
            session_id: turn.session_id,
            sequence: turn.sequence,
            status: turn.status.clone(),
            model_slug: turn.model_slug.clone(),
            started_at: turn.started_at,
            completed_at: turn.completed_at,
        }
    }
}
