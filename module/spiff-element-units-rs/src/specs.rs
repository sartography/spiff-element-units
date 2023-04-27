use serde::{Deserialize, Serialize};
use serde_json;

use crate::domain::{ElementIntrospection, Map};

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
    pub serializer_version: String,
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

impl ElementIntrospection for WorkflowSpec {
    fn push_element_ids(&self, ids: &mut Vec<String>) {
        self.spec.push_element_ids(ids);
    }
}

impl ElementIntrospection for ProcessSpec {
    fn push_element_ids(&self, ids: &mut Vec<String>) {
        ids.push(self.name.to_string());

        for (_, task_spec) in &self.task_specs {
            task_spec.push_element_ids(ids);
        }
    }
}

impl ElementIntrospection for TaskSpec {
    fn push_element_ids(&self, ids: &mut Vec<String>) {
        ids.push(self.name.to_string());
    }
}
