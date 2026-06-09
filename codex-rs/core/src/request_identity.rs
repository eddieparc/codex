use std::collections::HashMap;

use serde_json::Map;
use serde_json::Value;

use crate::client::X_CODEX_INSTALLATION_ID_HEADER;
use crate::client::X_CODEX_TURN_METADATA_HEADER;
use crate::client::X_CODEX_WINDOW_ID_HEADER;

const SESSION_ID_KEY: &str = "session_id";
const THREAD_ID_KEY: &str = "thread_id";
const TURN_ID_KEY: &str = "turn_id";
const WINDOW_ID_KEY: &str = "window_id";

/// Canonical Codex request identity shared by Responses HTTP and websocket transports.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodexRequestIdentity {
    pub(crate) installation_id: String,
    pub(crate) session_id: String,
    pub(crate) thread_id: String,
    pub(crate) turn_id: Option<String>,
    pub(crate) window_id: String,
}

impl CodexRequestIdentity {
    pub(crate) fn new(
        installation_id: String,
        session_id: String,
        thread_id: String,
        turn_id: Option<String>,
        window_id: String,
    ) -> Self {
        Self {
            installation_id,
            session_id,
            thread_id,
            turn_id,
            window_id,
        }
    }

    pub(crate) fn client_metadata(
        &self,
        turn_metadata_header: Option<&str>,
    ) -> HashMap<String, String> {
        let mut client_metadata = HashMap::from([
            (
                X_CODEX_INSTALLATION_ID_HEADER.to_string(),
                self.installation_id.clone(),
            ),
            (SESSION_ID_KEY.to_string(), self.session_id.clone()),
            (THREAD_ID_KEY.to_string(), self.thread_id.clone()),
            (X_CODEX_WINDOW_ID_HEADER.to_string(), self.window_id.clone()),
        ]);
        if let Some(turn_id) = &self.turn_id {
            client_metadata.insert(TURN_ID_KEY.to_string(), turn_id.clone());
        }
        if let Some(turn_metadata_header) = turn_metadata_header
            && !turn_metadata_header.is_empty()
        {
            client_metadata.insert(
                X_CODEX_TURN_METADATA_HEADER.to_string(),
                turn_metadata_header.to_string(),
            );
        }
        client_metadata
    }

    pub(crate) fn insert_turn_metadata_fields(&self, metadata: &mut Map<String, Value>) {
        metadata.extend([
            (
                SESSION_ID_KEY.to_string(),
                Value::String(self.session_id.clone()),
            ),
            (
                THREAD_ID_KEY.to_string(),
                Value::String(self.thread_id.clone()),
            ),
            (
                WINDOW_ID_KEY.to_string(),
                Value::String(self.window_id.clone()),
            ),
        ]);
        if let Some(turn_id) = &self.turn_id {
            metadata.insert(TURN_ID_KEY.to_string(), Value::String(turn_id.clone()));
        } else {
            metadata.remove(TURN_ID_KEY);
        }
    }
}
