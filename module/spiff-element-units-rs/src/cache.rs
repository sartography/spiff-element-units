use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

const WORKFLOW_SPEC_JSON_FILENAME: &str = "workflow_spec.json";

pub fn write_workflow_spec(cache_dir: &str, cache_key: &str, workflow_spec_json: &str) -> std::io::Result<()> {
let path = workflow_spec_cache_path(cache_dir, cache_key);
let mut file = File::create(path)?;
file.write_all(workflow_spec_json.as_bytes())?;

Ok(())
}

pub fn read_workflow_spec(cache_dir: &str, cache_key: &str) -> std::io::Result<String> {
std::fs::read_to_string(workflow_spec_cache_path(cache_dir, cache_key))
}

fn workflow_spec_cache_path(cache_dir: &str, cache_key: &str) -> PathBuf {
Path::new(cache_dir).join(cache_key).join(WORKFLOW_SPEC_JSON_FILENAME)
}