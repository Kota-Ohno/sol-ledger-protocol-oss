use anyhow::{Context, Result, bail};
use jsonschema::Resource;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::LazyLock};

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventIntegrity {
    pub payload_sha256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_event_sha256: Option<String>,
}

/// Compatibility facade for the original root-level API.
///
/// New code should prefer [`generated::EventEnvelope`], whose object payload
/// and collection fields encode more of the JSON Schema contract in Rust's
/// type system. The nested root types intentionally preserve the original
/// permissive serde surface; explicit conversions bridge them to strict types.
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

impl TryFrom<EventEnvelope> for generated::EventEnvelope {
    type Error = generated::error::ConversionError;

    fn try_from(event: EventEnvelope) -> std::result::Result<Self, Self::Error> {
        let Value::Object(payload) = event.payload else {
            return Err("event payload must be a JSON object".into());
        };
        Ok(Self {
            actor: event.actor.into(),
            event_id: event.event_id,
            event_type: event.event_type,
            integrity: event.integrity.into(),
            occurred_at: event.occurred_at,
            payload,
            recorded_at: event.recorded_at,
            run_id: event.run_id,
            schema_version: Value::String(event.schema_version),
            security: event.security.into(),
            span_id: event.span_id,
            subject_refs: event.subject_refs.unwrap_or_default(),
            trace_id: event.trace_id,
        })
    }
}

impl TryFrom<generated::EventEnvelope> for EventEnvelope {
    type Error = generated::error::ConversionError;

    fn try_from(event: generated::EventEnvelope) -> std::result::Result<Self, Self::Error> {
        let Value::String(schema_version) = event.schema_version else {
            return Err("event schemaVersion must be a string".into());
        };
        Ok(Self {
            schema_version,
            event_id: event.event_id,
            event_type: event.event_type,
            occurred_at: event.occurred_at,
            recorded_at: event.recorded_at,
            run_id: event.run_id,
            trace_id: event.trace_id,
            span_id: event.span_id,
            actor: event.actor.into(),
            subject_refs: (!event.subject_refs.is_empty()).then_some(event.subject_refs),
            payload: Value::Object(event.payload),
            security: event.security.into(),
            integrity: event.integrity.into(),
        })
    }
}

macro_rules! enum_bridge {
    ($compat:ty, $generated:ty, [$($variant:ident),+ $(,)?]) => {
        impl From<$compat> for $generated {
            fn from(value: $compat) -> Self {
                match value { $(<$compat>::$variant => Self::$variant),+ }
            }
        }
        impl From<$generated> for $compat {
            fn from(value: $generated) -> Self {
                match value { $(<$generated>::$variant => Self::$variant),+ }
            }
        }
    };
}

enum_bridge!(
    ActorKind,
    generated::EventEnvelopeActorKind,
    [Human, Agent, Service, System]
);
enum_bridge!(
    Sensitivity,
    generated::SecurityPolicySensitivity,
    [Public, Internal, Private, SecretNeverExport]
);
enum_bridge!(
    ContentMode,
    generated::SecurityPolicyContentMode,
    [MetadataOnly, HashOnly, Redacted, FullOptIn]
);
enum_bridge!(
    RetentionClass,
    generated::SecurityPolicyRetentionClass,
    [Ephemeral, UserManaged, Audit, LegalHold]
);

impl From<Actor> for generated::EventEnvelopeActor {
    fn from(value: Actor) -> Self {
        Self {
            id: value.id,
            kind: value.kind.into(),
            software: value.software,
        }
    }
}

impl From<generated::EventEnvelopeActor> for Actor {
    fn from(value: generated::EventEnvelopeActor) -> Self {
        Self {
            kind: value.kind.into(),
            id: value.id,
            software: value.software,
        }
    }
}

impl From<EventIntegrity> for generated::EventEnvelopeIntegrity {
    fn from(value: EventIntegrity) -> Self {
        Self {
            payload_sha256: value.payload_sha256,
            previous_event_sha256: value.previous_event_sha256,
        }
    }
}

impl From<generated::EventEnvelopeIntegrity> for EventIntegrity {
    fn from(value: generated::EventEnvelopeIntegrity) -> Self {
        Self {
            payload_sha256: value.payload_sha256,
            previous_event_sha256: value.previous_event_sha256,
        }
    }
}

impl From<SecurityPolicy> for generated::SecurityPolicy {
    fn from(value: SecurityPolicy) -> Self {
        Self {
            content_mode: value.content_mode.into(),
            expires_at: value.expires_at,
            redaction_profile: value.redaction_profile,
            retention_class: value.retention_class.into(),
            sensitivity: value.sensitivity.into(),
        }
    }
}

impl From<generated::SecurityPolicy> for SecurityPolicy {
    fn from(value: generated::SecurityPolicy) -> Self {
        Self {
            sensitivity: value.sensitivity.into(),
            content_mode: value.content_mode.into(),
            retention_class: value.retention_class.into(),
            expires_at: value.expires_at,
            redaction_profile: value.redaction_profile,
        }
    }
}
static VALIDATORS: LazyLock<HashMap<&'static str, jsonschema::Validator>> = LazyLock::new(|| {
    [
        (
            "event-envelope",
            include_str!("../schemas/event-envelope.schema.json"),
        ),
        (
            "security-policy",
            include_str!("../schemas/security-policy.schema.json"),
        ),
        (
            "artifact-ref",
            include_str!("../schemas/artifact-ref.schema.json"),
        ),
        (
            "provenance-edge",
            include_str!("../schemas/provenance-edge.schema.json"),
        ),
    ]
    .into_iter()
    .map(|(name, source)| {
        let schema: Value =
            serde_json::from_str(source).expect("embedded schema must be valid JSON");
        let security_schema: Value =
            serde_json::from_str(include_str!("../schemas/security-policy.schema.json"))
                .expect("embedded security schema must be valid JSON");
        let validator = jsonschema::options()
            .with_draft(jsonschema::Draft::Draft202012)
            .should_validate_formats(true)
            .with_resource(
                "https://sol-ledger.dev/schema/security-policy/0.1.0",
                Resource::from_contents(security_schema)
                    .expect("embedded security schema must register"),
            )
            .build(&schema)
            .expect("embedded schema must compile");
        (name, validator)
    })
    .collect()
});

pub fn validate(schema_name: &str, instance: &Value) -> Result<()> {
    let validator = VALIDATORS
        .get(schema_name)
        .with_context(|| format!("unknown schema: {schema_name}"))?;
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
        let generated = generated::EventEnvelope::try_from(event.clone()).unwrap();
        assert_eq!(EventEnvelope::try_from(generated).unwrap(), event);
        validate("event-envelope", &serde_json::to_value(event).unwrap()).unwrap();
    }

    #[test]
    fn compatibility_event_rejects_non_object_payload_when_converted() {
        let value: Value =
            serde_json::from_str(include_str!("../../../fixtures/valid/event-envelope.json"))
                .unwrap();
        let mut event: EventEnvelope = serde_json::from_value(value).unwrap();
        event.payload = Value::String("not an object".into());

        assert!(generated::EventEnvelope::try_from(event).is_err());
    }

    #[test]
    fn root_facade_preserves_permissive_nested_deserialization() {
        let mut value: Value =
            serde_json::from_str(include_str!("../../../fixtures/valid/event-envelope.json"))
                .unwrap();
        value["actor"]["futureField"] = Value::Bool(true);

        assert!(serde_json::from_value::<EventEnvelope>(value.clone()).is_ok());
        assert!(serde_json::from_value::<generated::EventEnvelope>(value).is_err());
    }

    #[test]
    fn generated_bridge_normalizes_empty_subject_refs_to_omission() {
        let value: Value =
            serde_json::from_str(include_str!("../../../fixtures/valid/event-envelope.json"))
                .unwrap();
        let mut event: EventEnvelope = serde_json::from_value(value).unwrap();
        event.subject_refs = Some(Vec::new());

        let generated = generated::EventEnvelope::try_from(event).unwrap();
        assert!(generated.subject_refs.is_empty());
        assert_eq!(
            EventEnvelope::try_from(generated).unwrap().subject_refs,
            None
        );
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
