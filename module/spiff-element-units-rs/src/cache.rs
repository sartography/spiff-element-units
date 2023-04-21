use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub mod entry {
    pub enum Type {
        OriginalWorkflowSpecsJSON,
        OurWorkflowSpecsJSON,
    }

    impl Type {
        pub fn filename(&self) -> &str {
            match self {
                Type::OriginalWorkflowSpecsJSON => "workflow_specs.json",
                Type::OurWorkflowSpecsJSON => "our_workflow_specs.json",
            }
        }
    }
}

const CACHE_VERSION: &str = "v1";

pub fn write(
    cache_dir: &str,
    cache_key: &str,
    entry_type: entry::Type,
    content: &str,
) -> io::Result<()> {
    let path = created_cache_path(cache_dir, cache_key)?.join(entry_type.filename());
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn path_for_entry(cache_dir: &str, cache_key: &str, entry_type: entry::Type) -> PathBuf {
    cache_path(cache_dir, cache_key).join(entry_type.filename())
}

fn cache_path(cache_dir: &str, cache_key: &str) -> PathBuf {
    Path::new(cache_dir).join(CACHE_VERSION).join(cache_key)
}

fn created_cache_path(cache_dir: &str, cache_key: &str) -> io::Result<PathBuf> {
    let cache_path = cache_path(cache_dir, cache_key);
    fs::create_dir_all(&cache_path)?;
    Ok(cache_path)
}
