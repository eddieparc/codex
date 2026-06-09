use super::*;

use pretty_assertions::assert_eq;

#[test]
fn client_metadata_contains_canonical_fields_and_turn_metadata() {
    let metadata = CodexRequestMetadata::new(
        "install-123".to_string(),
        "session-123".to_string(),
        "thread-123".to_string(),
        Some("turn-123".to_string()),
        "thread-123:4".to_string(),
    );

    assert_eq!(
        metadata.client_metadata(Some(r#"{"turn_id":"turn-123"}"#)),
        HashMap::from([
            (
                X_CODEX_INSTALLATION_ID_HEADER.to_string(),
                "install-123".to_string(),
            ),
            (SESSION_ID_KEY.to_string(), "session-123".to_string()),
            (THREAD_ID_KEY.to_string(), "thread-123".to_string()),
            (TURN_ID_KEY.to_string(), "turn-123".to_string()),
            (
                X_CODEX_WINDOW_ID_HEADER.to_string(),
                "thread-123:4".to_string(),
            ),
            (
                X_CODEX_TURN_METADATA_HEADER.to_string(),
                r#"{"turn_id":"turn-123"}"#.to_string(),
            ),
        ])
    );
}

#[test]
fn client_metadata_omits_optional_turn_fields() {
    let metadata = CodexRequestMetadata::new(
        "install-123".to_string(),
        "session-123".to_string(),
        "thread-123".to_string(),
        None,
        "thread-123:4".to_string(),
    );

    assert_eq!(
        metadata.client_metadata(/*turn_metadata_header*/ None),
        HashMap::from([
            (
                X_CODEX_INSTALLATION_ID_HEADER.to_string(),
                "install-123".to_string(),
            ),
            (SESSION_ID_KEY.to_string(), "session-123".to_string()),
            (THREAD_ID_KEY.to_string(), "thread-123".to_string()),
            (
                X_CODEX_WINDOW_ID_HEADER.to_string(),
                "thread-123:4".to_string(),
            ),
        ])
    );
}

#[test]
fn compatibility_headers_preserve_direct_window_and_turn_metadata_headers() {
    let metadata = CodexRequestMetadata::new(
        "install-123".to_string(),
        "session-123".to_string(),
        "thread-123".to_string(),
        Some("turn-123".to_string()),
        "thread-123:4".to_string(),
    );
    let turn_metadata_header = HeaderValue::from_static(r#"{"turn_id":"turn-123"}"#);

    let headers = metadata.compatibility_headers(Some(&turn_metadata_header));

    assert_eq!(
        headers
            .get(X_CODEX_WINDOW_ID_HEADER)
            .and_then(|value| value.to_str().ok()),
        Some("thread-123:4")
    );
    assert_eq!(
        headers
            .get(X_CODEX_TURN_METADATA_HEADER)
            .and_then(|value| value.to_str().ok()),
        Some(r#"{"turn_id":"turn-123"}"#)
    );
}

#[test]
fn insert_turn_metadata_fields_overwrites_identity_values() {
    let metadata = CodexRequestMetadata::new(
        "install-123".to_string(),
        "session-123".to_string(),
        "thread-123".to_string(),
        Some("turn-123".to_string()),
        "thread-123:4".to_string(),
    );
    let mut turn_metadata = Map::from_iter([
        (
            SESSION_ID_KEY.to_string(),
            Value::String("client-session".to_string()),
        ),
        (
            THREAD_ID_KEY.to_string(),
            Value::String("client-thread".to_string()),
        ),
        (
            TURN_ID_KEY.to_string(),
            Value::String("client-turn".to_string()),
        ),
        (
            WINDOW_ID_KEY.to_string(),
            Value::String("client-window".to_string()),
        ),
    ]);

    metadata.insert_turn_metadata_fields(&mut turn_metadata);

    assert_eq!(
        turn_metadata,
        Map::from_iter([
            (
                SESSION_ID_KEY.to_string(),
                Value::String("session-123".to_string()),
            ),
            (
                THREAD_ID_KEY.to_string(),
                Value::String("thread-123".to_string()),
            ),
            (
                TURN_ID_KEY.to_string(),
                Value::String("turn-123".to_string()),
            ),
            (
                WINDOW_ID_KEY.to_string(),
                Value::String("thread-123:4".to_string()),
            ),
        ])
    );
}
