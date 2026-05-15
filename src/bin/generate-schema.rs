use std::path::PathBuf;

use job_posting_schema::JobPosting;
use schemars::schema_for;

fn main() {
    let schema = schema_for!(JobPosting);
    let json = serde_json::to_string_pretty(&schema)
        .expect("failed to serialize JSON schema");

    let output_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "schema.json"].iter().collect();
    std::fs::write(&output_path, format!("{json}\n"))
        .expect("failed to write schema.json");

    println!("wrote {}", output_path.display());
}
