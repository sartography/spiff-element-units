use serde::{Deserialize, Serialize};
use serde_json;

type Map<V> = std::collections::BTreeMap<String, V>;

// TODO: keep filling out the fields. for those we don't care about, like position,
// use json Value? we need to be able to serialize them back out but don't need to
// inspect their values

// TODO: more consideration to disallowing unknown fields, probably the safest bet?
// perhaps collect unknown fields and check/write them to disk instead?

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkflowSpec {
    pub serializer_version: String,
    pub spec: ProcessSpec,
    pub subprocess_specs: Map<ProcessSpec>,

    #[serde(flatten)]
    unrecognized_fields: Map<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessSpec {
    pub name: String,
    pub description: String,
    pub typename: String,
    pub file: String,
    pub task_specs: Map<TaskSpec>,
    pub data_objects: Map<serde_json::Value>,
    /*
    "correlation_keys": {},
    "io_specification": null,
    */
    #[serde(flatten)]
    unrecognized_fields: Map<serde_json::Value>,
}

// TODO: enum of different task spec flavors
// TODO: can serde flatten be used along with struct composition?
// TODO: or a macro to include the common fields?
#[derive(Debug, Deserialize, Serialize)]
pub struct TaskSpec {
    pub name: String,
    pub typename: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,

    #[serde(flatten)]
    unrecognized_fields: Map<serde_json::Value>,
}
