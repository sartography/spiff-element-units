use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

const CACHE_VERSION: &str = "v1";
const WORKFLOW_SPEC_JSON_FILENAME: &str = "workflow_spec.json";

pub fn write_workflow_spec(
    cache_dir: &str,
    cache_key: &str,
    workflow_spec_json: &str,
) -> io::Result<()> {
    let path = created_cache_path(cache_dir, cache_key)?.join(WORKFLOW_SPEC_JSON_FILENAME);
    let mut file = File::create(path)?;
    file.write_all(workflow_spec_json.as_bytes())?;

    Ok(())
}

pub fn read_workflow_spec(cache_dir: &str, cache_key: &str) -> io::Result<String> {
    std::fs::read_to_string(workflow_spec_cache_path(cache_dir, cache_key))
}

fn cache_path(cache_dir: &str, cache_key: &str) -> PathBuf {
    Path::new(cache_dir).join(CACHE_VERSION).join(cache_key)
}

fn created_cache_path(cache_dir: &str, cache_key: &str) -> io::Result<PathBuf> {
    let cache_path = cache_path(cache_dir, cache_key);
    fs::create_dir_all(&cache_path)?;
    Ok(cache_path)
}

fn workflow_spec_cache_path(cache_dir: &str, cache_key: &str) -> PathBuf {
    cache_path(cache_dir, cache_key).join(WORKFLOW_SPEC_JSON_FILENAME)
}
