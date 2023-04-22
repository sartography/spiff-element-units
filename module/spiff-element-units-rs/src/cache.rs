use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub mod entry {
    pub enum Type {
        ElementUnitMap,
        OriginalWorkflowSpecsJSON,
        OurWorkflowSpecsJSON,
    }

    impl Type {
        pub fn filename(&self) -> &str {
            match self {
                Type::ElementUnitMap => "element_unit_map.json",
                Type::OriginalWorkflowSpecsJSON => "workflow_specs.json",
                Type::OurWorkflowSpecsJSON => "our_workflow_specs.json",
            }
        }
    }
}

const CACHE_VERSION: &str = "v1";

pub fn created_path_for_entry(
    cache_dir: &str,
    cache_key: &str,
    entry_type: entry::Type,
) -> io::Result<PathBuf> {
    let cache_path = cache_path(cache_dir, cache_key);
    fs::create_dir_all(&cache_path)?;

    Ok(cache_path.join(entry_type.filename()))
}

pub fn path_for_entry(cache_dir: &str, cache_key: &str, entry_type: entry::Type) -> PathBuf {
    cache_path(cache_dir, cache_key).join(entry_type.filename())
}

fn cache_path(cache_dir: &str, cache_key: &str) -> PathBuf {
    Path::new(cache_dir).join(CACHE_VERSION).join(cache_key)
}
