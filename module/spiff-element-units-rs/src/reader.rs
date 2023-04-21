use serde;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn read_file<T>(path: &PathBuf) -> Result<T, Box<dyn Error>>
where
    T: serde::de::DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let result: T = serde_json::from_reader(reader)?;

    Ok(result)
}

pub fn read_str<'a, T>(str: &'a str) -> Result<T, Box<dyn Error>>
where
    T: serde::Deserialize<'a>,
{
    Ok(serde_json::from_str::<T>(str)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    use crate::domain::WorkflowSpec;

    // don't need to fully test serde just that our structs line up against the json

    #[test]
    fn test_can_read_no_tasks() -> Result<(), Box<dyn Error>> {
        // don't need to fully test serde just that our structs line up against the json
        let path = test_case_path("no-tasks/no-tasks.json");
        let workflow_spec: WorkflowSpec = read_file(&path)?;

        let subprocess_spec_names = vec![];
        let task_spec_names = vec![
            "Start",
            "StartEvent_1",
            "Event_0qq9il3",
            "no_tasks.EndJoin",
            "End",
        ];

        sanity_check(
            &workflow_spec,
            "no_tasks",
            &subprocess_spec_names,
            &task_spec_names,
        );

        Ok(())
    }

    fn sanity_check(
        workflow_spec: &WorkflowSpec,
        name: &str,
        subprocess_spec_names: &[&str],
        task_spec_names: &[&str],
    ) {
        assert_eq!(
            workflow_spec.subprocess_specs.len(),
            subprocess_spec_names.len()
        );

        for name in subprocess_spec_names {
            assert!(workflow_spec
                .subprocess_specs
                .contains_key(&name.to_string()));
        }

        let spec = &workflow_spec.spec;

        assert_eq!(spec.name, "no_tasks");
        assert_eq!(spec.task_specs.len(), task_spec_names.len());

        for name in task_spec_names {
            assert!(spec.task_specs.contains_key(&name.to_string()));
        }
    }

    fn test_case_path(test_case: &str) -> PathBuf {
        // TODO: set an env? so this works in and out of docker
        let base_path = Path::new("/integration/tests/data/specs-json/test-cases");
        base_path.join(test_case)
    }
}
