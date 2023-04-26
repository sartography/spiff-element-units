use std::fs;
use std::io;
use std::path::{Path, PathBuf};

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

pub mod entry {

    pub enum Type {
        OriginalWorkflowSpecsJSON,
        Manifest,
        ManifestEntry(String),
    }

    impl Type {
        pub fn filename(&self) -> String {
            use Type::*;

            match self {
                OriginalWorkflowSpecsJSON => "workflow_specs.json".to_string(),
                Manifest => "manifest.json".to_string(),
                ManifestEntry(id) => format!("{}.json", id),
            }
        }
    }
}

pub mod manifest {
    use crate::domain::{ElementUnit, ElementUnitType, IndexedVec};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ManifestEntry {
        pub sha2: String,
        pub r#type: ElementUnitType,
        pub requirements: u64,
    }

    pub type Manifest = IndexedVec<ManifestEntry>;

    impl ManifestEntry {
        pub fn from_element_unit(element_unit: &ElementUnit) -> Self {
            Self {
                sha2: element_unit.sha2_str(),
                r#type: element_unit.r#type(),
                requirements: 0,
            }
        }
    }
}
