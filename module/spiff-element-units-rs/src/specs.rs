use serde::{Deserialize, Serialize};
use serde_json;

use crate::basis::{ElementIntrospection, Map};

//
// these structs define the subset of fields in each json structure
// that we need for processing. the other fields are lumped into
// the `rest` field which is then serialized back in its original
// form.
//

pub type RestMap = Map<serde_json::Value>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkflowSpec {
    pub spec: ProcessSpec,
    pub subprocess_specs: Map<ProcessSpec>,

    #[serde(flatten)]
    rest: RestMap,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    rest: RestMap,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    rest: RestMap,
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

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Bpmn {
        // for now at least we don't care about the actual value of thsese
        // fields, just that they have a value when determining if/how we
        // build element units for this process
        pub data_input_associations: serde_json::Value,
        pub data_output_associations: serde_json::Value,
        pub io_specification: serde_json::Value,
        pub lane: serde_json::Value,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Spiff {
        pub prescript: Option<String>,
        pub postscript: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Subprocess {
        pub spec: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Script {
        pub script: String,
    }
}

//
//
//

pub struct SpecReference {
    pub spec_name: String,
    pub task_typename: String,
}

//
//
//

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

impl WorkflowSpec {
    pub fn from_process(process_spec: &ProcessSpec) -> Self {
        Self {
            spec: process_spec.clone(),
            subprocess_specs: Map::<ProcessSpec>::new(),
            rest: RestMap::default(),
        }
    }
}

impl ProcessSpec {
    pub fn isolable(&self) -> bool {
        self.data_objects.len() == 0
            && is_empty(&self.io_specification)
            && is_empty(&self.correlation_keys)
    }

    pub fn spec_references(&self) -> Vec<SpecReference> {
        self.task_specs
            .values()
            .map(|ts| ts.spec_reference())
            .into_iter()
            .flatten()
            .collect()
    }
}

impl TaskSpec {
    pub fn spec_reference(&self) -> Option<SpecReference> {
        let spec_name = self.subprocess.as_ref()?.spec.to_string();
        let task_typename = self.typename.to_string();

        Some(SpecReference {
            spec_name,
            task_typename,
        })
    }
}

fn is_empty(val: &serde_json::Value) -> bool {
    use serde_json::Value::*;

    match val {
        Null => true,
        Object(o) => o.len() == 0,
        _ => false,
    }
}

#[cfg(test)]
mod spec_tests {
    use super::*;
    use std::error::Error;
    use std::path::{Path, PathBuf};

    use crate::reader::read;

    type ReadResult<T> = Result<T, Box<dyn Error>>;

    #[test]
    fn test_no_tasks() -> ReadResult<()> {
        let path = test_case_path("no-tasks/no-tasks.json");
        let workflow_spec: WorkflowSpec = read(&path)?;

	assert_eq!(workflow_spec.spec.isolable(), true);
	assert_eq!(workflow_spec.spec.spec_references().len(), 0);

        Ok(())
    }

    #[test]
    fn test_simple_call_activity() -> ReadResult<()> {
        let path = test_case_path("simple-call-activity/simple_call_activity.json");
        let workflow_spec: WorkflowSpec = read(&path)?;

	assert_eq!(workflow_spec.spec.isolable(), true);
	assert_eq!(workflow_spec.spec.spec_references().len(), 1);

        Ok(())
    }

    #[test]
    fn test_simple_subprocess() -> ReadResult<()> {
        let path = test_case_path("simple-subprocess/simple_subprocess.json");
        let workflow_spec: WorkflowSpec = read(&path)?;

	assert_eq!(workflow_spec.spec.isolable(), true);
	assert_eq!(workflow_spec.spec.spec_references().len(), 1);

        Ok(())
    }

    fn test_case_path(test_case: &str) -> PathBuf {
        // TODO: set an env? so this works in and out of docker
	// TODO: test helper module? broader test items in TODOs
        let base_path = Path::new("/integration/tests/data/specs-json/test-cases");
        base_path.join(test_case)
    }
}
