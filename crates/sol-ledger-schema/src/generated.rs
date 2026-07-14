#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "`ArtifactRef`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://sol-ledger.dev/schema/artifact-ref/0.1.0\","]
#[doc = "  \"title\": \"Sol Ledger artifact reference\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"artifactId\","]
#[doc = "    \"byteLength\","]
#[doc = "    \"mediaType\","]
#[doc = "    \"redaction\","]
#[doc = "    \"storage\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"artifactId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"byteLength\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"locator\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"mediaType\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"originalName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"redaction\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"none\","]
#[doc = "        \"partial\","]
#[doc = "        \"full\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"storage\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"none\","]
#[doc = "        \"local_blob\","]
#[doc = "        \"external\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ArtifactRef {
    #[serde(rename = "artifactId")]
    pub artifact_id: ::std::string::String,
    #[serde(rename = "byteLength")]
    pub byte_length: i64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub locator: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mediaType")]
    pub media_type: ::std::string::String,
    #[serde(
        rename = "originalName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub original_name: ::std::option::Option<::std::string::String>,
    pub redaction: ArtifactRefRedaction,
    pub storage: ArtifactRefStorage,
}
#[doc = "`ArtifactRefRedaction`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"none\","]
#[doc = "    \"partial\","]
#[doc = "    \"full\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ArtifactRefRedaction {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "full")]
    Full,
}
impl ::std::fmt::Display for ArtifactRefRedaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Partial => f.write_str("partial"),
            Self::Full => f.write_str("full"),
        }
    }
}
impl ::std::str::FromStr for ArtifactRefRedaction {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "partial" => Ok(Self::Partial),
            "full" => Ok(Self::Full),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ArtifactRefRedaction {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ArtifactRefRedaction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ArtifactRefRedaction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ArtifactRefStorage`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"none\","]
#[doc = "    \"local_blob\","]
#[doc = "    \"external\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ArtifactRefStorage {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "local_blob")]
    LocalBlob,
    #[serde(rename = "external")]
    External,
}
impl ::std::fmt::Display for ArtifactRefStorage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::LocalBlob => f.write_str("local_blob"),
            Self::External => f.write_str("external"),
        }
    }
}
impl ::std::str::FromStr for ArtifactRefStorage {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "local_blob" => Ok(Self::LocalBlob),
            "external" => Ok(Self::External),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ArtifactRefStorage {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ArtifactRefStorage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ArtifactRefStorage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`EventEnvelope`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://sol-ledger.dev/schema/event-envelope/0.1.0\","]
#[doc = "  \"title\": \"Sol Ledger event envelope\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"actor\","]
#[doc = "    \"eventId\","]
#[doc = "    \"eventType\","]
#[doc = "    \"integrity\","]
#[doc = "    \"occurredAt\","]
#[doc = "    \"payload\","]
#[doc = "    \"recordedAt\","]
#[doc = "    \"schemaVersion\","]
#[doc = "    \"security\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"actor\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"kind\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"kind\": {"]
#[doc = "          \"enum\": ["]
#[doc = "            \"human\","]
#[doc = "            \"agent\","]
#[doc = "            \"service\","]
#[doc = "            \"system\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"software\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"eventId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"eventType\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"integrity\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"payloadSha256\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"payloadSha256\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"previousEventSha256\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"occurredAt\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"payload\": {"]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    \"recordedAt\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"runId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"schemaVersion\": {"]
#[doc = "      \"const\": \"0.1.0\""]
#[doc = "    },"]
#[doc = "    \"security\": {"]
#[doc = "      \"$id\": \"https://sol-ledger.dev/schema/security-policy/0.1.0\","]
#[doc = "      \"title\": \"SecurityPolicy\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"contentMode\","]
#[doc = "        \"retentionClass\","]
#[doc = "        \"sensitivity\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"contentMode\": {"]
#[doc = "          \"enum\": ["]
#[doc = "            \"metadata_only\","]
#[doc = "            \"hash_only\","]
#[doc = "            \"redacted\","]
#[doc = "            \"full_opt_in\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"expiresAt\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"redactionProfile\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"retentionClass\": {"]
#[doc = "          \"enum\": ["]
#[doc = "            \"ephemeral\","]
#[doc = "            \"user_managed\","]
#[doc = "            \"audit\","]
#[doc = "            \"legal_hold\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"sensitivity\": {"]
#[doc = "          \"enum\": ["]
#[doc = "            \"public\","]
#[doc = "            \"internal\","]
#[doc = "            \"private\","]
#[doc = "            \"secret_never_export\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false,"]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "    },"]
#[doc = "    \"spanId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"subjectRefs\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"traceId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EventEnvelope {
    pub actor: EventEnvelopeActor,
    #[serde(rename = "eventId")]
    pub event_id: ::std::string::String,
    #[serde(rename = "eventType")]
    pub event_type: ::std::string::String,
    pub integrity: EventEnvelopeIntegrity,
    #[serde(rename = "occurredAt")]
    pub occurred_at: ::std::string::String,
    pub payload: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "recordedAt")]
    pub recorded_at: ::std::string::String,
    #[serde(
        rename = "runId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub run_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "schemaVersion")]
    pub schema_version: ::serde_json::Value,
    pub security: SecurityPolicy,
    #[serde(
        rename = "spanId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub span_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "subjectRefs",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub subject_refs: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "traceId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trace_id: ::std::option::Option<::std::string::String>,
}
#[doc = "`EventEnvelopeActor`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"kind\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"human\","]
#[doc = "        \"agent\","]
#[doc = "        \"service\","]
#[doc = "        \"system\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"software\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EventEnvelopeActor {
    pub id: ::std::string::String,
    pub kind: EventEnvelopeActorKind,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub software: ::std::option::Option<::std::string::String>,
}
#[doc = "`EventEnvelopeActorKind`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"human\","]
#[doc = "    \"agent\","]
#[doc = "    \"service\","]
#[doc = "    \"system\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EventEnvelopeActorKind {
    #[serde(rename = "human")]
    Human,
    #[serde(rename = "agent")]
    Agent,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "system")]
    System,
}
impl ::std::fmt::Display for EventEnvelopeActorKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Human => f.write_str("human"),
            Self::Agent => f.write_str("agent"),
            Self::Service => f.write_str("service"),
            Self::System => f.write_str("system"),
        }
    }
}
impl ::std::str::FromStr for EventEnvelopeActorKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "human" => Ok(Self::Human),
            "agent" => Ok(Self::Agent),
            "service" => Ok(Self::Service),
            "system" => Ok(Self::System),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventEnvelopeActorKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventEnvelopeActorKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventEnvelopeActorKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`EventEnvelopeIntegrity`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"payloadSha256\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"payloadSha256\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"previousEventSha256\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EventEnvelopeIntegrity {
    #[serde(rename = "payloadSha256")]
    pub payload_sha256: ::std::string::String,
    #[serde(
        rename = "previousEventSha256",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub previous_event_sha256: ::std::option::Option<::std::string::String>,
}
#[doc = "`ProvenanceEdge`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://sol-ledger.dev/schema/provenance-edge/0.1.0\","]
#[doc = "  \"title\": \"Sol Ledger provenance edge\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"edgeId\","]
#[doc = "    \"fromRef\","]
#[doc = "    \"recordedAt\","]
#[doc = "    \"relationship\","]
#[doc = "    \"toRef\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"attributes\": {"]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    \"edgeId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"fromRef\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"recordedAt\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"relationship\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"generated_by\","]
#[doc = "        \"used\","]
#[doc = "        \"derived_from\","]
#[doc = "        \"attributed_to\","]
#[doc = "        \"acted_on_behalf_of\","]
#[doc = "        \"invalidated_by\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"toRef\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceEdge {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub attributes: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "edgeId")]
    pub edge_id: ::std::string::String,
    #[serde(rename = "fromRef")]
    pub from_ref: ::std::string::String,
    #[serde(rename = "recordedAt")]
    pub recorded_at: ::std::string::String,
    pub relationship: ProvenanceEdgeRelationship,
    #[serde(rename = "toRef")]
    pub to_ref: ::std::string::String,
}
#[doc = "`ProvenanceEdgeRelationship`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"generated_by\","]
#[doc = "    \"used\","]
#[doc = "    \"derived_from\","]
#[doc = "    \"attributed_to\","]
#[doc = "    \"acted_on_behalf_of\","]
#[doc = "    \"invalidated_by\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProvenanceEdgeRelationship {
    #[serde(rename = "generated_by")]
    GeneratedBy,
    #[serde(rename = "used")]
    Used,
    #[serde(rename = "derived_from")]
    DerivedFrom,
    #[serde(rename = "attributed_to")]
    AttributedTo,
    #[serde(rename = "acted_on_behalf_of")]
    ActedOnBehalfOf,
    #[serde(rename = "invalidated_by")]
    InvalidatedBy,
}
impl ::std::fmt::Display for ProvenanceEdgeRelationship {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::GeneratedBy => f.write_str("generated_by"),
            Self::Used => f.write_str("used"),
            Self::DerivedFrom => f.write_str("derived_from"),
            Self::AttributedTo => f.write_str("attributed_to"),
            Self::ActedOnBehalfOf => f.write_str("acted_on_behalf_of"),
            Self::InvalidatedBy => f.write_str("invalidated_by"),
        }
    }
}
impl ::std::str::FromStr for ProvenanceEdgeRelationship {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "generated_by" => Ok(Self::GeneratedBy),
            "used" => Ok(Self::Used),
            "derived_from" => Ok(Self::DerivedFrom),
            "attributed_to" => Ok(Self::AttributedTo),
            "acted_on_behalf_of" => Ok(Self::ActedOnBehalfOf),
            "invalidated_by" => Ok(Self::InvalidatedBy),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProvenanceEdgeRelationship {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ProvenanceEdgeRelationship {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProvenanceEdgeRelationship {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SecurityPolicy`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://sol-ledger.dev/schema/security-policy/0.1.0\","]
#[doc = "  \"title\": \"SecurityPolicy\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"contentMode\","]
#[doc = "    \"retentionClass\","]
#[doc = "    \"sensitivity\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"contentMode\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"metadata_only\","]
#[doc = "        \"hash_only\","]
#[doc = "        \"redacted\","]
#[doc = "        \"full_opt_in\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"expiresAt\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"redactionProfile\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"retentionClass\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"ephemeral\","]
#[doc = "        \"user_managed\","]
#[doc = "        \"audit\","]
#[doc = "        \"legal_hold\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sensitivity\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"public\","]
#[doc = "        \"internal\","]
#[doc = "        \"private\","]
#[doc = "        \"secret_never_export\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SecurityPolicy {
    #[serde(rename = "contentMode")]
    pub content_mode: SecurityPolicyContentMode,
    #[serde(
        rename = "expiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expires_at: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "redactionProfile",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub redaction_profile: ::std::option::Option<::std::string::String>,
    #[serde(rename = "retentionClass")]
    pub retention_class: SecurityPolicyRetentionClass,
    pub sensitivity: SecurityPolicySensitivity,
}
#[doc = "`SecurityPolicyContentMode`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"metadata_only\","]
#[doc = "    \"hash_only\","]
#[doc = "    \"redacted\","]
#[doc = "    \"full_opt_in\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SecurityPolicyContentMode {
    #[serde(rename = "metadata_only")]
    MetadataOnly,
    #[serde(rename = "hash_only")]
    HashOnly,
    #[serde(rename = "redacted")]
    Redacted,
    #[serde(rename = "full_opt_in")]
    FullOptIn,
}
impl ::std::fmt::Display for SecurityPolicyContentMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MetadataOnly => f.write_str("metadata_only"),
            Self::HashOnly => f.write_str("hash_only"),
            Self::Redacted => f.write_str("redacted"),
            Self::FullOptIn => f.write_str("full_opt_in"),
        }
    }
}
impl ::std::str::FromStr for SecurityPolicyContentMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "metadata_only" => Ok(Self::MetadataOnly),
            "hash_only" => Ok(Self::HashOnly),
            "redacted" => Ok(Self::Redacted),
            "full_opt_in" => Ok(Self::FullOptIn),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SecurityPolicyContentMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SecurityPolicyContentMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SecurityPolicyContentMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SecurityPolicyRetentionClass`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"ephemeral\","]
#[doc = "    \"user_managed\","]
#[doc = "    \"audit\","]
#[doc = "    \"legal_hold\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SecurityPolicyRetentionClass {
    #[serde(rename = "ephemeral")]
    Ephemeral,
    #[serde(rename = "user_managed")]
    UserManaged,
    #[serde(rename = "audit")]
    Audit,
    #[serde(rename = "legal_hold")]
    LegalHold,
}
impl ::std::fmt::Display for SecurityPolicyRetentionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ephemeral => f.write_str("ephemeral"),
            Self::UserManaged => f.write_str("user_managed"),
            Self::Audit => f.write_str("audit"),
            Self::LegalHold => f.write_str("legal_hold"),
        }
    }
}
impl ::std::str::FromStr for SecurityPolicyRetentionClass {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ephemeral" => Ok(Self::Ephemeral),
            "user_managed" => Ok(Self::UserManaged),
            "audit" => Ok(Self::Audit),
            "legal_hold" => Ok(Self::LegalHold),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SecurityPolicyRetentionClass {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SecurityPolicyRetentionClass {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SecurityPolicyRetentionClass {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SecurityPolicySensitivity`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"public\","]
#[doc = "    \"internal\","]
#[doc = "    \"private\","]
#[doc = "    \"secret_never_export\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SecurityPolicySensitivity {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "secret_never_export")]
    SecretNeverExport,
}
impl ::std::fmt::Display for SecurityPolicySensitivity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Public => f.write_str("public"),
            Self::Internal => f.write_str("internal"),
            Self::Private => f.write_str("private"),
            Self::SecretNeverExport => f.write_str("secret_never_export"),
        }
    }
}
impl ::std::str::FromStr for SecurityPolicySensitivity {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "public" => Ok(Self::Public),
            "internal" => Ok(Self::Internal),
            "private" => Ok(Self::Private),
            "secret_never_export" => Ok(Self::SecretNeverExport),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SecurityPolicySensitivity {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SecurityPolicySensitivity {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SecurityPolicySensitivity {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
