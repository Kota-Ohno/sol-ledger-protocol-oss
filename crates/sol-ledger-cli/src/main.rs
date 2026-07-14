use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Number, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    VerifyChain {
        path: PathBuf,
        #[arg(long, value_parser = parse_sha256)]
        expected_head_sha256: String,
    },
}

fn parse_sha256(value: &str) -> std::result::Result<String, String> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        Ok(value.to_owned())
    } else {
        Err("expected 64 lowercase hexadecimal characters".into())
    }
}

fn canonical_hash(value: &Value) -> Result<String> {
    Ok(hex::encode(Sha256::digest(serde_jcs::to_vec(value)?)))
}

fn verify_chain(path: PathBuf, expected_head_sha256: &str) -> Result<()> {
    let reader =
        BufReader::new(File::open(&path).with_context(|| format!("open {}", path.display()))?);
    let (count, actual_head) = verify_reader(reader)?;
    if actual_head.as_deref() != Some(expected_head_sha256) {
        bail!(
            "trusted head mismatch: expected {expected_head_sha256}, got {}",
            actual_head.as_deref().unwrap_or("<empty>")
        );
    }
    println!("Verified {count} event(s) at trusted head {expected_head_sha256}.");
    Ok(())
}

fn verify_reader(reader: impl BufRead) -> Result<(usize, Option<String>)> {
    let mut previous: Option<String> = None;
    let mut count = 0;
    for (index, line) in reader.lines().enumerate() {
        let value = parse_json_without_duplicates(&line.context("read JSONL")?)
            .with_context(|| format!("parse line {}", index + 1))?;
        sol_ledger_schema::validate("event-envelope", &value)
            .with_context(|| format!("validate line {}", index + 1))?;
        let expected_payload_hash = value
            .pointer("/integrity/payloadSha256")
            .and_then(Value::as_str)
            .context("missing payload hash")?;
        let actual_payload_hash = canonical_hash(value.get("payload").context("missing payload")?)?;
        if expected_payload_hash != actual_payload_hash {
            bail!("payload hash mismatch at line {}", index + 1);
        }
        let linked = value
            .pointer("/integrity/previousEventSha256")
            .and_then(Value::as_str)
            .map(str::to_owned);
        if linked != previous {
            bail!("chain mismatch at line {}", index + 1);
        }
        previous = Some(canonical_hash(&value)?);
        count += 1;
    }
    Ok((count, previous))
}

fn parse_json_without_duplicates(input: &str) -> std::result::Result<Value, serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let value = NoDuplicateValue.deserialize(&mut deserializer)?;
    deserializer.end()?;
    Ok(value)
}

struct NoDuplicateValue;

impl<'de> DeserializeSeed<'de> for NoDuplicateValue {
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> std::result::Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(NoDuplicateVisitor)
    }
}

struct NoDuplicateVisitor;

impl<'de> Visitor<'de> for NoDuplicateVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a JSON value without duplicate object members")
    }

    fn visit_bool<E>(self, value: bool) -> std::result::Result<Value, E> {
        Ok(Value::Bool(value))
    }
    fn visit_i64<E>(self, value: i64) -> std::result::Result<Value, E> {
        Ok(Value::Number(value.into()))
    }
    fn visit_u64<E>(self, value: u64) -> std::result::Result<Value, E> {
        Ok(Value::Number(value.into()))
    }
    fn visit_f64<E: de::Error>(self, value: f64) -> std::result::Result<Value, E> {
        Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| E::custom("non-finite JSON number"))
    }
    fn visit_str<E: de::Error>(self, value: &str) -> std::result::Result<Value, E> {
        Ok(Value::String(value.into()))
    }
    fn visit_string<E>(self, value: String) -> std::result::Result<Value, E> {
        Ok(Value::String(value))
    }
    fn visit_none<E>(self) -> std::result::Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_unit<E>(self) -> std::result::Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_seq<A>(self, mut sequence: A) -> std::result::Result<Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(NoDuplicateValue)? {
            values.push(value);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A>(self, mut object: A) -> std::result::Result<Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = Map::new();
        while let Some(key) = object.next_key::<String>()? {
            if values.contains_key(&key) {
                return Err(de::Error::custom(format!("duplicate JSON member: {key}")));
            }
            values.insert(key, object.next_value_seed(NoDuplicateValue)?);
        }
        Ok(Value::Object(values))
    }
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::VerifyChain {
            path,
            expected_head_sha256,
        } => verify_chain(path, &expected_head_sha256),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn event(id: &str, payload: Value, previous: Option<String>) -> Value {
        let payload_hash = canonical_hash(&payload).unwrap();
        serde_json::json!({
          "schemaVersion": "0.1.0",
          "eventId": id,
          "eventType": "artifact.observed",
          "occurredAt": "2026-07-11T12:00:00Z",
          "recordedAt": "2026-07-11T12:00:01Z",
          "actor": { "kind": "agent", "id": "agent_sol" },
          "payload": payload,
          "security": { "sensitivity": "private", "contentMode": "hash_only", "retentionClass": "audit" },
          "integrity": { "payloadSha256": payload_hash, "previousEventSha256": previous }
        })
    }

    fn jsonl(values: &[Value]) -> String {
        values
            .iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn accepts_an_intact_chain() {
        let first = event("evt_01JABCDE0001", serde_json::json!({"step": 1}), None);
        let second = event(
            "evt_01JABCDE0002",
            serde_json::json!({"step": 2}),
            Some(canonical_hash(&first).unwrap()),
        );
        assert_eq!(
            verify_reader(Cursor::new(jsonl(&[first, second])))
                .unwrap()
                .0,
            2
        );
    }

    #[test]
    fn rejects_payload_tampering() {
        let mut value = event("evt_01JABCDE0001", serde_json::json!({"step": 1}), None);
        value["payload"]["step"] = serde_json::json!(999);
        assert!(verify_reader(Cursor::new(jsonl(&[value]))).is_err());
    }

    #[test]
    fn rejects_deletion_or_reordering() {
        let first = event("evt_01JABCDE0001", serde_json::json!({"step": 1}), None);
        let second = event(
            "evt_01JABCDE0002",
            serde_json::json!({"step": 2}),
            Some(canonical_hash(&first).unwrap()),
        );
        assert!(verify_reader(Cursor::new(jsonl(std::slice::from_ref(&second)))).is_err());
        assert!(verify_reader(Cursor::new(jsonl(&[second, first]))).is_err());
    }

    #[test]
    fn trusted_head_detects_tail_truncation_and_empty_replacement() {
        let first = event("evt_01JABCDE0001", serde_json::json!({"step": 1}), None);
        let second = event(
            "evt_01JABCDE0002",
            serde_json::json!({"step": 2}),
            Some(canonical_hash(&first).unwrap()),
        );
        let expected = canonical_hash(&second).unwrap();
        let (_, truncated_head) = verify_reader(Cursor::new(jsonl(&[first]))).unwrap();
        assert_ne!(truncated_head.as_deref(), Some(expected.as_str()));
        let (_, empty_head) = verify_reader(Cursor::new("")).unwrap();
        assert_ne!(empty_head.as_deref(), Some(expected.as_str()));
    }

    #[test]
    fn rejects_duplicate_json_members() {
        assert!(
            parse_json_without_duplicates(r#"{"payload":{},"payload":{"forged":true}}"#).is_err()
        );
        assert!(parse_json_without_duplicates(r#"{"payload":{"x":1,"x":2}}"#).is_err());
    }
}
