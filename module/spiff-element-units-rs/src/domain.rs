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

    // for now at least we don't care about the actual value of thsese
    // fields, just that they have a value when determining if/how we
    // build element units for this process
    
    pub data_objects: Map<serde_json::Value>,
    pub correlation_keys: serde_json::Value,
    pub io_specification: serde_json::Value,
    
    #[serde(flatten)]
    rest: Map<serde_json::Value>,
}

//
// for the breakdown of how the different specs are serialized in SpiffWorkflow:
//
// https://github.com/sartography/SpiffWorkflow/blob/main/SpiffWorkflow/bpmn/serializer/helpers/spec.py
//

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseTaskSpec {
    pub name: String,
    pub typename: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BpmnTaskSpecMixin {
    // for now at least we don't care about the actual value of thsese
    // fields, just that they have a value when determining if/how we
    // build element units for this process

    pub data_input_associations: serde_json::Value,
    pub data_output_associations: serde_json::Value,
    pub io_specification: serde_json::Value,
    pub lane: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubprocessTaskSpecMixin {
    pub spec: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskSpec {
    #[serde(flatten)]
    pub base: BaseTaskSpec,

    #[serde(flatten)]
    pub bpmn: Option<BpmnTaskSpecMixin>,

    #[serde(flatten)]
    pub subprocess: Option<SubprocessTaskSpecMixin>,

    #[serde(flatten)]
    rest: Map<serde_json::Value>,
}
