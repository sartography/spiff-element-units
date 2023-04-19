use serde_json;
use std::error::Error;

use crate::domain::WorkflowSpec;

// TODO: generic parse result types

pub fn parse_str(str: &str) -> Result<WorkflowSpec, Box<dyn Error>> {
    let workflow_spec: WorkflowSpec = serde_json::from_str(str)?;

    Ok(workflow_spec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io;
    use std::path::{Path, PathBuf};

    fn test_case_path(test_case: &str) -> PathBuf {
        // TODO: set an env? so this works in and out of docker
        let base_path = Path::new("/integration/tests/data/specs-json/test-cases");
        base_path.join(test_case)
    }

    fn test_case_contents(test_case: &str) -> io::Result<String> {
        let path = test_case_path(test_case);
        fs::read_to_string(path)
    }

    #[test]
    fn test_can_deserialize_no_tasks() -> Result<(), Box<dyn Error>> {
        // TODO: make test cases for sanity checks, don't need to fully test serde just
        // that our structs line up against the json
        let contents = test_case_contents("no-tasks/no-tasks.json")?;
        let workflow_spec = parse_str(contents.as_str())?;

        assert_eq!(
            workflow_spec.serializer_version,
            "spiff-element-units-integration"
        );
        assert_eq!(workflow_spec.subprocess_specs.len(), 0);

        let spec = workflow_spec.spec;

        assert_eq!(spec.name, "no_tasks");
        assert_eq!(spec.typename, "BpmnProcessSpec");
        assert_eq!(
            spec.file,
            "tests/data/process-models/test-cases/no-tasks/no-tasks.bpmn"
        );
        assert_eq!(spec.task_specs.len(), 5);
        assert!(spec.task_specs.contains_key("Start"));
        assert!(spec.task_specs.contains_key("StartEvent_1"));
        assert!(spec.task_specs.contains_key("Event_0qq9il3"));
        assert!(spec.task_specs.contains_key("no_tasks.EndJoin"));
        assert!(spec.task_specs.contains_key("End"));

        Ok(())
    }
}
