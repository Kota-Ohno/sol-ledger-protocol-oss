use anyhow::{Context, Result};
use quote::quote;
use schemars::schema::Schema;
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::PathBuf, process::Command};
use typify::{TypeSpace, TypeSpaceSettings};

fn main() -> Result<()> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let schema_dir = root.join("schemas");
    let definitions = [
        ("ArtifactRef", "artifact-ref.schema.json"),
        ("EventEnvelope", "event-envelope.schema.json"),
        ("ProvenanceEdge", "provenance-edge.schema.json"),
    ];
    let expected_schema_files = [
        "artifact-ref.schema.json",
        "event-envelope.schema.json",
        "provenance-edge.schema.json",
        "security-policy.schema.json",
    ];
    let mut actual_schema_files = fs::read_dir(&schema_dir)?
        .filter_map(|entry| {
            let name = entry.ok()?.file_name().into_string().ok()?;
            name.ends_with(".schema.json").then_some(name)
        })
        .collect::<Vec<_>>();
    actual_schema_files.sort();
    anyhow::ensure!(
        actual_schema_files == expected_schema_files,
        "schema generator coverage mismatch: {actual_schema_files:?}"
    );

    let mut schemas = BTreeMap::new();
    for (name, file) in definitions {
        let source = fs::read_to_string(schema_dir.join(file))
            .with_context(|| format!("read schema {file}"))?;
        let mut value: Value =
            serde_json::from_str(&source).with_context(|| format!("parse schema {file}"))?;
        if file == "event-envelope.schema.json" {
            let security_source =
                fs::read_to_string(schema_dir.join("security-policy.schema.json"))
                    .context("read security-policy schema for local reference")?;
            let mut security: Value = serde_json::from_str(&security_source)
                .context("parse security-policy schema for local reference")?;
            security["title"] = Value::String("SecurityPolicy".to_owned());
            value["properties"]["security"] = security;
        }
        project_for_types(&mut value);
        let schema: Schema = serde_json::from_value(value)
            .with_context(|| format!("project schema {file} for Rust types"))?;
        schemas.insert(name.to_owned(), schema);
    }

    let mut type_space = TypeSpace::new(&TypeSpaceSettings::default());
    type_space.add_ref_types(schemas)?;
    let generated = type_space.to_stream();
    let output = quote! {
        // Generated from schemas/*.schema.json by pnpm generate. Do not edit.
        #generated
    }
    .to_string();
    let output_path = root.join("crates/sol-ledger-schema/src/generated.rs");
    fs::write(&output_path, output).context("write generated Rust types")?;
    let status = Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .arg(&output_path)
        .status()
        .context("run rustfmt on generated Rust types")?;
    anyhow::ensure!(status.success(), "rustfmt failed for generated Rust types");
    Ok(())
}

/// Typify 0.7 does not support conditional schemas and intentionally cannot
/// encode runtime-only constraints in Rust's type system. Remove only those
/// validation keywords from the in-memory projection; checked-in schemas stay
/// untouched and remain the runtime-validation source of truth.
fn project_for_types(value: &mut Value) {
    match value {
        Value::Object(object) => {
            if let Some(Value::Array(branches)) = object.get_mut("allOf") {
                branches.retain(|branch| branch.get("if").is_none());
                if branches.is_empty() {
                    object.remove("allOf");
                }
            }
            for keyword in [
                "if",
                "then",
                "else",
                "format",
                "pattern",
                "minimum",
                "maximum",
                "minLength",
                "maxLength",
                "uniqueItems",
            ] {
                object.remove(keyword);
            }
            for child in object.values_mut() {
                project_for_types(child);
            }
        }
        Value::Array(items) => items.iter_mut().for_each(project_for_types),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_preserves_non_conditional_all_of_composition() {
        let mut schema = serde_json::json!({
            "allOf": [
                {"type": "object", "properties": {"kept": {"type": "string"}}},
                {"if": {"properties": {"mode": {"const": "strict"}}}, "then": {"required": ["kept"]}}
            ]
        });

        project_for_types(&mut schema);

        assert_eq!(schema["allOf"].as_array().unwrap().len(), 1);
        assert_eq!(schema["allOf"][0]["properties"]["kept"]["type"], "string");
    }
}
