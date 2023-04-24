use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

// TODO: look at breaking this file out into sub modules (name?) and re-exporting?
//       - actually just move things to manifest, element_units, specs, etc
// TODO: rename to basis?

//
// for domain objects we stick with this map structure to support
// stable ordering when serializing. this may not always be desired
// but is good for the integration tests which commit the cache.
//

type Map<V> = BTreeMap<String, V>;

//
// when dealing with element units we often have several keys that
// point to the same deserialized structure. instead of repeating the
// data for each key, we maintain indexes into a `vec` per key.
//

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct IndexedVec<T> {
    pub items: Vec<T>,
    pub index_map: Map<Vec<usize>>,
}

//
// element units that are supported within the lib.
//

#[derive(Debug, Deserialize, Serialize)]
pub enum ElementUnit {
    FullWorkflow(WorkflowSpec),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ElementUnitType {
    FullWorkflow,
}

pub type ElementUnits = IndexedVec<ElementUnit>;

//
// cached element unit are tracked in a manifest that contains an entry
// for each element unit associated with a given cache key.
//

#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestEntry {
    pub id: String,
    pub r#type: ElementUnitType,
    pub requirements: u64,
}

pub type Manifest = IndexedVec<ManifestEntry>;

//
// these structs define the subset of fields in each json structure
// that we need for processing. the other fields are lumped into
// the `rest` field which is then serialized back in its original
// form.
//

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkflowSpec {
    pub spec: ProcessSpec,
    pub subprocess_specs: Map<ProcessSpec>,

    #[serde(flatten)]
    rest: Map<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessSpec {
    pub name: String,
    pub typename: String,
    pub task_specs: Map<TaskSpec>,

    // for now at least we don't care about the actual value of thsese
    // fields, just that they have a value when determining if/how we
    // build element units for this process
    pub data_objects: Map<serde_json::Value>,
    pub correlation_keys: serde_json::Value,
    pub io_specification: serde_json::Value,

    #[serde(flatten)]
    rest: Map<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskSpec {
    pub name: String,
    pub typename: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,

    #[serde(flatten)]
    pub bpmn: Option<task_spec_mixin::Bpmn>,

    #[serde(flatten)]
    pub spiff: Option<task_spec_mixin::Spiff>,

    #[serde(flatten)]
    pub subprocess: Option<task_spec_mixin::Subprocess>,

    #[serde(flatten)]
    pub script: Option<task_spec_mixin::Script>,

    #[serde(flatten)]
    rest: Map<serde_json::Value>,
}

pub mod task_spec_mixin {
    use serde::{Deserialize, Serialize};
    use serde_json;

    //
    // for the breakdown of how the different specs are serialized in SpiffWorkflow:
    //
    // https://github.com/sartography/SpiffWorkflow/blob/main/SpiffWorkflow/bpmn/serializer/task_spec.py
    // https://github.com/sartography/SpiffWorkflow/blob/main/SpiffWorkflow/bpmn/serializer/helpers/spec.py
    // https://github.com/sartography/SpiffWorkflow/blob/main/SpiffWorkflow/spiff/serializer/task_spec.py
    //
    // these may not be entirely correct per the links above but are close enough for what we
    // need in this lib. any discrepencies should be thought of under that lens.
    //

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Bpmn {
        // for now at least we don't care about the actual value of thsese
        // fields, just that they have a value when determining if/how we
        // build element units for this process
        pub data_input_associations: serde_json::Value,
        pub data_output_associations: serde_json::Value,
        pub io_specification: serde_json::Value,
        pub lane: serde_json::Value,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Spiff {
        pub prescript: Option<String>,
        pub postscript: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Subprocess {
        pub spec: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Script {
        pub script: String,
    }
}

//
//
//

pub trait ElementIDs {
    fn push_element_ids(&self, ids: &mut Vec<String>);

    fn element_ids(&self) -> Vec<String> {
        // TODO: with_capacity
        let mut vec: Vec<String> = Vec::new();
        self.push_element_ids(&mut vec);
        vec
    }
}

impl ElementIDs for ElementUnit {
    fn push_element_ids(&self, ids: &mut Vec<String>) {
        match self {
            ElementUnit::FullWorkflow(w) => w.push_element_ids(ids),
        }
    }
}

impl ElementIDs for WorkflowSpec {
    fn push_element_ids(&self, ids: &mut Vec<String>) {
        self.spec.push_element_ids(ids);
    }
}

impl ElementIDs for ProcessSpec {
    fn push_element_ids(&self, ids: &mut Vec<String>) {
        ids.push(self.name.to_string());

        for (_, task_spec) in &self.task_specs {
            task_spec.push_element_ids(ids);
        }
    }
}

impl ElementIDs for TaskSpec {
    fn push_element_ids(&self, ids: &mut Vec<String>) {
        ids.push(self.name.to_string());
    }
}

//
//
//

impl ElementUnit {
    pub fn id(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}", self.r#type()));

        for element_id in self.element_ids() {
            hasher.update(element_id);
        }

        let hash = hasher.finalize();
        format!("{:x}", hash)
    }

    pub fn r#type(&self) -> ElementUnitType {
        match self {
            ElementUnit::FullWorkflow(_) => ElementUnitType::FullWorkflow,
        }
    }
}

//
//
//

impl ManifestEntry {
    pub fn from_element_unit(element_unit: &ElementUnit) -> Self {
        Self {
            id: element_unit.id(),
            r#type: element_unit.r#type(),
            requirements: 0,
        }
    }
}

//
//
//

impl WorkflowSpec {
    pub fn from_element_unit(element_unit: &ElementUnit) -> &WorkflowSpec {
        match element_unit {
            ElementUnit::FullWorkflow(ws) => ws,
        }
    }
}

//
//
//

impl<T> IndexedVec<T> {
    pub fn push_for_keys(&mut self, item: T, keys: &[String]) {
        let index = self.items.len();
        self.items.push(item);

        for key in keys {
            self.index_map
                .entry(key.to_string())
                .and_modify(|value| value.push(index))
                .or_insert(vec![index]);
        }
    }

    pub fn last_item_for_key(&self, key: String) -> Option<&T> {
        self.index_map
            .get(&key)
            .and_then(|v| v.last().copied())
            .and_then(|i| self.items.get(i))
    }
}

#[cfg(test)]
mod indexed_vec_tests {
    use super::IndexedVec;

    #[test]
    fn test_push_for_keys() {
        let mut iv: IndexedVec<&str> = Default::default();

        iv.push_for_keys("bob", &["key1".to_string(), "key2".to_string()]);
        iv.push_for_keys("joe", &["key3".to_string()]);
        iv.push_for_keys("sue", &["key1".to_string(), "key3".to_string()]);

        assert_eq!(iv.items, vec!["bob", "joe", "sue"]);
        assert_eq!(iv.index_map.len(), 3);
        assert_eq!(iv.index_map["key1"], vec![0, 2]);
        assert_eq!(iv.index_map["key2"], vec![0]);
        assert_eq!(iv.index_map["key3"], vec![1, 2]);
    }

    #[test]
    fn test_last_item_for_key() {
        let mut iv: IndexedVec<&str> = Default::default();

        iv.push_for_keys("bob", &["key1".to_string(), "key2".to_string()]);
        iv.push_for_keys("joe", &["key1".to_string(), "key3".to_string()]);
        iv.push_for_keys("sue", &["key3".to_string()]);

        assert_eq!(iv.last_item_for_key("key1".to_string()), Some(&"joe"));
        assert_eq!(iv.last_item_for_key("key2".to_string()), Some(&"bob"));
        assert_eq!(iv.last_item_for_key("key3".to_string()), Some(&"sue"));
        assert_eq!(iv.last_item_for_key("hey".to_string()), None);
    }
}
