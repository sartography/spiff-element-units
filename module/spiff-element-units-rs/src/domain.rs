use serde::Deserialize;
use std::collections::HashMap;

// TODO: keep filling out the fields. for those we don't care about, like position,
// use json Value? we need to be able to serialize them back out but don't need to
// inspect their values

#[derive(Deserialize, Debug)]
pub struct WorkflowSpec {
    pub serializer_version: String,
    pub spec: ProcessSpec,
    pub subprocess_specs: HashMap<String, ProcessSpec>,
}

#[derive(Deserialize, Debug)]
pub struct ProcessSpec {
    pub name: String,
    pub typename: String,
    pub file: String,
    pub task_specs: HashMap<String, TaskSpec>,
}

// TODO: enum of different task spec flavors
#[derive(Deserialize, Debug)]
pub struct TaskSpec {
    pub name: String,
    pub typename: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}
