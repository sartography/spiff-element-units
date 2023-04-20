use serde::{Deserialize, Serialize};
use serde_json;

type Map<V> = std::collections::BTreeMap<String, V>;

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
    pub data_objects: Map<serde_json::Value>,
    /*
    "correlation_keys": {},
    "io_specification": null,
    */
    #[serde(flatten)]
    rest: Map<serde_json::Value>,
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
    rest: Map<serde_json::Value>,
}
