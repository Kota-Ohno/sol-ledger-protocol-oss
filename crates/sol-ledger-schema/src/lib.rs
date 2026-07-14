use anyhow::{Context, Result, bail};
use jsonschema::Resource;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Schema-derived types. The existing root-level API remains available for
/// compatibility while consumers migrate to these generated definitions.
pub mod generated;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorKind {
    Human,
    Agent,
    Service,
    System,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sensitivity {
    Public,
    Internal,
    Private,
    SecretNeverExport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentMode {
    MetadataOnly,
    HashOnly,
    Redacted,
    FullOptIn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    Ephemeral,
    UserManaged,
    Audit,
    LegalHold,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityPolicy {
    pub sensitivity: Sensitivity,
    pub content_mode: ContentMode,
    pub retention_class: RetentionClass,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redaction_profile: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    pub kind: ActorKind,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventIntegrity {
    pub payload_sha256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_event_sha256: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventEnvelope {
    pub schema_version: String,
    pub event_id: String,
    pub event_type: String,
    pub occurred_at: String,
    pub recorded_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
    pub actor: Actor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_refs: Option<Vec<String>>,
    pub payload: Value,
    pub security: SecurityPolicy,
    pub integrity: EventIntegrity,
}

pub fn validate(schema_name: &str, instance: &Value) -> Result<()> {
    let source = match schema_name {
        "event-envelope" => include_str!("../../../schemas/event-envelope.schema.json"),
        "security-policy" => include_str!("../../../schemas/security-policy.schema.json"),
        "artifact-ref" => include_str!("../../../schemas/artifact-ref.schema.json"),
        "provenance-edge" => include_str!("../../../schemas/provenance-edge.schema.json"),
        _ => bail!("unknown schema: {schema_name}"),
    };
    let schema: Value = serde_json::from_str(source).context("parse embedded schema")?;
    let security_schema: Value =
        serde_json::from_str(include_str!("../../../schemas/security-policy.schema.json"))
            .context("parse security schema")?;
    let validator = jsonschema::options()
        .with_draft(jsonschema::Draft::Draft202012)
        .should_validate_formats(true)
        .with_resource(
            "https://sol-ledger.dev/schema/security-policy/0.1.0",
            Resource::from_contents(security_schema).context("register security schema")?,
        )
        .build(&schema)
        .context("compile schema")?;
    let errors: Vec<String> = validator
        .iter_errors(instance)
        .map(|error| error.to_string())
        .collect();
    if errors.is_empty() {
        Ok(())
    } else {
        bail!(errors.join("; "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    #[test]
    fn fixture_matrix_matches_expectations() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        for expected in ["valid", "invalid"] {
            for entry in fs::read_dir(root.join("fixtures").join(expected)).unwrap() {
                let path = entry.unwrap().path();
                let name = path.file_stem().unwrap().to_str().unwrap();
                let value: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
                assert_eq!(
                    validate(name, &value).is_ok(),
                    expected == "valid",
                    "{}",
                    path.display()
                );
            }
        }
    }

    #[test]
    fn generated_types_round_trip_valid_fixtures() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        for schema_name in [
            "artifact-ref",
            "event-envelope",
            "provenance-edge",
            "security-policy",
        ] {
            let path = root
                .join("fixtures/valid")
                .join(format!("{schema_name}.json"));
            let value: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            let round_tripped = match schema_name {
                "artifact-ref" => serde_json::to_value(
                    serde_json::from_value::<generated::ArtifactRef>(value).unwrap(),
                ),
                "event-envelope" => serde_json::to_value(
                    serde_json::from_value::<generated::EventEnvelope>(value).unwrap(),
                ),
                "provenance-edge" => serde_json::to_value(
                    serde_json::from_value::<generated::ProvenanceEdge>(value).unwrap(),
                ),
                "security-policy" => serde_json::to_value(
                    serde_json::from_value::<generated::SecurityPolicy>(value).unwrap(),
                ),
                _ => unreachable!(),
            }
            .unwrap();
            validate(schema_name, &round_tripped).unwrap();
        }
    }

    #[test]
    fn typed_event_serializes_to_the_schema_contract() {
        let event = EventEnvelope {
            schema_version: "0.1.0".into(),
            event_id: "evt_01JABCDE0001".into(),
            event_type: "artifact.observed".into(),
            occurred_at: "2026-07-11T12:00:00Z".into(),
            recorded_at: "2026-07-11T12:00:01Z".into(),
            run_id: None,
            trace_id: None,
            span_id: None,
            actor: Actor {
                kind: ActorKind::Agent,
                id: "agent_sol".into(),
                software: None,
            },
            subject_refs: None,
            payload: serde_json::json!({"status": "observed"}),
            security: SecurityPolicy {
                sensitivity: Sensitivity::Private,
                content_mode: ContentMode::HashOnly,
                retention_class: RetentionClass::Audit,
                expires_at: None,
                redaction_profile: None,
            },
            integrity: EventIntegrity {
                payload_sha256: "a".repeat(64),
                previous_event_sha256: None,
            },
        };
        validate("event-envelope", &serde_json::to_value(event).unwrap()).unwrap();
    }

    #[test]
    fn rejects_invalid_date_time_formats() {
        let mut event: Value =
            serde_json::from_str(include_str!("../../../fixtures/valid/event-envelope.json"))
                .unwrap();
        event["occurredAt"] = Value::String("not-a-date".into());
        assert!(validate("event-envelope", &event).is_err());
    }

    #[test]
    fn stored_artifacts_require_locators() {
        let artifact = serde_json::json!({
            "artifactId": format!("artifact:sha256:{}", "a".repeat(64)),
            "mediaType": "text/plain",
            "byteLength": 1,
            "storage": "local_blob",
            "redaction": "none"
        });
        assert!(validate("artifact-ref", &artifact).is_err());
    }
}
