use serde::Deserialize;
use serde_json;
use std::collections::HashMap;

// TODO: keep filling out the fields. for those we don't care about, like position,
// use json Value? we need to be able to serialize them back out but don't need to
// inspect their values

// TODO: more consideration to disallowing unknown fields, probably the safest bet?
// perhaps collect unknown fields and check/write them to disk instead?

#[derive(Deserialize, Debug)]
pub struct WorkflowSpec {
    pub serializer_version: String,
    pub spec: ProcessSpec,
    pub subprocess_specs: HashMap<String, ProcessSpec>,

    #[serde(flatten)]
    pub unrecognized_fields: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct ProcessSpec {
    pub name: String,
    pub description: String,
    pub typename: String,
    pub file: String,
    pub task_specs: HashMap<String, TaskSpec>,
    pub data_objects: HashMap<String, serde_json::Value>,
    /*
    "correlation_keys": {},
    "io_specification": null,
    */

    #[serde(flatten)]
    pub unrecognized_fields: HashMap<String, serde_json::Value>,
}

// TODO: enum of different task spec flavors
// TODO: can serde flatten be used along with struct composition?
// TODO: or a macro to include the common fields?
#[derive(Deserialize, Debug)]
pub struct TaskSpec {
    pub name: String,
    pub typename: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,

    #[serde(flatten)]
    pub unrecognized_fields: HashMap<String, serde_json::Value>,
}
