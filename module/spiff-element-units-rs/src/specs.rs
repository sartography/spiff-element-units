use std::collections::HashSet;

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
pub type SubprocessSpecs = Map<ProcessSpec>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkflowSpec {
    pub spec: ProcessSpec,
    pub subprocess_specs: SubprocessSpecs,

    #[serde(flatten)]
    pub rest: RestMap,
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

impl ElementIntrospection for WorkflowSpec {
    fn push_element_ids(&self, ids: &mut Vec<String>) {
        self.spec.push_element_ids(ids);

        for (_, subprocess_spec) in &self.subprocess_specs {
            subprocess_spec.push_element_ids(ids);
        }
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
        if self.isolable() {
            ids.push(self.name.to_string());
        }
    }
}

impl WorkflowSpec {
    pub fn has_unique_element_ids(&self) -> bool {
        let element_ids = self.element_ids();
        let unique_element_ids: Vec<_> = element_ids
            .iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        !element_ids.is_empty() && element_ids.len() == unique_element_ids.len()
    }

    pub fn set_serializer_version(&mut self, version: &str) {
        let key = "serializer_version".to_string();
        let value = serde_json::Value::String(format!("spiff-element-units-{}", version));

        self.rest.insert(key, value);
    }
}

impl ProcessSpec {
    pub fn isolable(&self) -> bool {
        self.data_objects.len() == 0
            && is_empty(&self.io_specification)
            && is_empty(&self.correlation_keys)
    }

    pub fn call_activity_spec_references(&self) -> Vec<String> {
        self.task_specs
            .values()
            .map(|ts| ts.call_activity_spec_reference())
            .into_iter()
            .flatten()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }
}

impl TaskSpec {
    pub fn call_activity_spec_reference(&self) -> Option<String> {
        (self.typename == "CallActivity").then_some(self.subprocess.as_ref()?.spec.to_string())
    }

    pub fn isolable(&self) -> bool {
        self.typename != "Simple" && self.is_rendered() && !self.is_event()
    }

    fn is_event(&self) -> bool {
        !is_empty_or_missing("event_definition", &self.rest)
    }

    fn is_rendered(&self) -> bool {
        !is_empty_or_missing("position", &self.rest)
    }
}

fn is_empty(val: &serde_json::Value) -> bool {
    use serde_json::Value::*;

    // TODO: fill this out as needed, eventually get rid of _ =>
    match val {
        Null => true,
        Object(o) => o.len() == 0,
        _ => false,
    }
}

fn is_empty_or_missing(key: &str, map: &RestMap) -> bool {
    map.get(key).filter(|val| !is_empty(val)).is_none()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::path::{Path, PathBuf};

    use crate::reader::read;

    type ReadResult<T> = Result<T, Box<dyn Error>>;

    #[test]
    fn test_manual_tasks() -> ReadResult<()> {
        let path = test_case_path("manual-tasks/manual_tasks.json");
        let workflow_spec: WorkflowSpec = read(&path)?;

        assert_eq!(workflow_spec.has_unique_element_ids(), true);
        assert_eq!(workflow_spec.spec.isolable(), true);
        assert_eq!(workflow_spec.spec.call_activity_spec_references().len(), 0);

	let manual_task = workflow_spec.spec.task_specs.get("Activity_1n7p3m4").unwrap();
	assert_eq!(manual_task.is_rendered(), true);
	assert_eq!(manual_task.is_event(), false);
	assert_eq!(manual_task.isolable(), true);

        Ok(())
    }

    #[test]
    fn test_no_tasks() -> ReadResult<()> {
        let path = test_case_path("no-tasks/no-tasks.json");
        let workflow_spec: WorkflowSpec = read(&path)?;

        assert_eq!(workflow_spec.has_unique_element_ids(), true);
        assert_eq!(workflow_spec.spec.isolable(), true);
        assert_eq!(workflow_spec.spec.call_activity_spec_references().len(), 0);

        Ok(())
    }

    #[test]
    fn test_simple_call_activity() -> ReadResult<()> {
        let path = test_case_path("simple-call-activity/simple_call_activity.json");
        let workflow_spec: WorkflowSpec = read(&path)?;

        assert_eq!(workflow_spec.has_unique_element_ids(), true);
        assert_eq!(workflow_spec.spec.isolable(), true);
        assert_eq!(workflow_spec.spec.call_activity_spec_references().len(), 1);

        Ok(())
    }

    #[test]
    fn test_simple_subprocess() -> ReadResult<()> {
        let path = test_case_path("simple-subprocess/simple_subprocess.json");
        let workflow_spec: WorkflowSpec = read(&path)?;

        assert_eq!(workflow_spec.has_unique_element_ids(), false);
        assert_eq!(workflow_spec.spec.isolable(), true);
        assert_eq!(workflow_spec.spec.call_activity_spec_references().len(), 0);

        Ok(())
    }

    fn test_case_path(test_case: &str) -> PathBuf {
        // TODO: set an env? so this works in and out of docker
        // TODO: test helper module? broader test items in TODOs
        let base_path = Path::new("/integration/tests/data/specs-json/test-cases");
        base_path.join(test_case)
    }
}
