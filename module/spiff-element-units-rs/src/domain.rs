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

// TODO: move to parser
#[cfg(test)]
mod tests {
    use serde_json;
    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;
    use super::*;

    #[test]
    fn test_can_deserialize_no_tasks() -> Result<(), Box<dyn Error>> {
       // TODO: set an env? so this works in and out of docker
       let path = Path::new("/integration/tests/data/specs-json/test-cases/no-tasks/no-tasks.json");
       let file = File::open(path)?;
       let reader = BufReader::new(file);

       let workflow_spec: WorkflowSpec = serde_json::from_reader(reader)?;

       // TODO: make expected objects and assert_eq! instead of checking each field
       assert_eq!(workflow_spec.serializer_version, "spiff-element-units-integration");
       assert_eq!(workflow_spec.subprocess_specs.len(), 0);

       let spec = workflow_spec.spec;
       
       assert_eq!(spec.name, "no_tasks");
       assert_eq!(spec.typename, "BpmnProcessSpec");
       assert_eq!(spec.file, "tests/data/process-models/test-cases/no-tasks/no-tasks.bpmn");
       assert_eq!(spec.task_specs.len(), 5);
       assert!(spec.task_specs.contains_key("Start"));
       assert!(spec.task_specs.contains_key("StartEvent_1"));
       assert!(spec.task_specs.contains_key("Event_0qq9il3"));
       assert!(spec.task_specs.contains_key("no_tasks.EndJoin"));
       assert!(spec.task_specs.contains_key("End"));

       Ok(())
    }
}