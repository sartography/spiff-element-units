use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeMap;

// TODO: look at breaking this file out into sub modules (name?) and re-exporting?
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
pub enum ElementUnitType {
    FullWorkflow,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ElementUnit {
    FullWorkflow(WorkflowSpec),
}

pub type ElementUnits = IndexedVec<ElementUnit>;

//
// element units are tracked in a manifest which maps an element id to
// a vector of element unit types and cache path locations.
//

#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestEntry {
    pub element_unit_type: ElementUnitType,
    pub path: String,
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
}

impl ElementIDs for ElementUnit {
fn push_element_ids(&self, ids: &mut Vec<String>)
    {
        match self {
            ElementUnit::FullWorkflow(w) => w.push_element_ids(ids),
        }
    }
}

impl ElementIDs for WorkflowSpec {
fn push_element_ids(&self, ids: &mut Vec<String>)
    {
        self.spec.push_element_ids(ids);
    }
}

impl ElementIDs for ProcessSpec {
fn push_element_ids(&self, ids: &mut Vec<String>)
    {
        ids.push(self.name.to_string());

        for (_, task_spec) in &self.task_specs {
            task_spec.push_element_ids(ids);
        }
    }
}

impl ElementIDs for TaskSpec {
fn push_element_ids(&self, ids: &mut Vec<String>)
    {
        ids.push(self.name.to_string());
    }
}

//
//
//

impl ElementUnit {
    pub fn element_ids(&self) -> Vec<String> {
        // TODO: with_capacity
        let mut vec: Vec<String> = Vec::new();
        self.push_element_ids(&mut vec);
        vec
    }
}

//
//
//

use std::hash::{Hash, Hasher};

impl Hash for ElementUnit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // TODO: add some other things in here
        for element_id in self.element_ids() {
            element_id.hash(state);
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
}

#[cfg(test)]
mod indexed_vec_tests {
    use super::IndexedVec;

    #[test]
    fn test_can_push_for_keys() {
        let mut iv: IndexedVec<String> = Default::default();

        iv.push_for_keys("bob".to_string(), &["key1".to_string(), "key2".to_string()]);
        iv.push_for_keys("joe".to_string(), &["key3".to_string()]);
        iv.push_for_keys("sue".to_string(), &["key1".to_string(), "key3".to_string()]);

        assert_eq!(iv.items, vec!["bob", "joe", "sue"]);
        assert_eq!(iv.index_map.len(), 3);
        assert_eq!(iv.index_map["key1"], vec![0, 2]);
        assert_eq!(iv.index_map["key2"], vec![0]);
        assert_eq!(iv.index_map["key3"], vec![1, 2]);
    }
}
