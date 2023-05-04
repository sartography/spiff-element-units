use serde;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

//
// read a json file at `path` and deserialize as `T`
//
pub fn read<T>(path: &PathBuf) -> Result<T, Box<dyn Error>>
where
    T: serde::de::DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let result: T = serde_json::from_reader(reader)?;

    Ok(result)
}

//
// read a json `str` and deserialize as `T`
//
pub fn read_string<'a, T>(str: &'a str) -> serde_json::Result<T>
where
    T: serde::Deserialize<'a>,
{
    serde_json::from_str::<T>(str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    use crate::specs::WorkflowSpec;

    type ReadResult<T> = Result<T, Box<dyn Error>>;

    // don't need to fully test serde just that our structs line up against the json

    #[test]
    fn test_can_read_no_tasks() -> ReadResult<()> {
        let subprocess_spec_names: [&str; 0] = [];
        let task_spec_names: [&str; 5] = [
            "Start",
            "StartEvent_1",
            "Event_0qq9il3",
            "no_tasks.EndJoin",
            "End",
        ];

        sanity_check(
            "no-tasks/no-tasks.json",
            "no_tasks",
            &subprocess_spec_names,
            &task_spec_names,
        )?;

        Ok(())
    }

    fn sanity_check(
        test_case_file: &str,
        name: &str,
        subprocess_spec_names: &[&str],
        task_spec_names: &[&str],
    ) -> ReadResult<()> {
        let path = test_case_path(test_case_file);
        let workflow_spec: WorkflowSpec = read(&path)?;

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

        assert_eq!(spec.name, name);
        assert_eq!(spec.task_specs.len(), task_spec_names.len());

        for name in task_spec_names {
            assert!(spec.task_specs.contains_key(&name.to_string()));
        }

        Ok(())
    }

    fn test_case_path(test_case: &str) -> PathBuf {
        // TODO: set an env? so this works in and out of docker
        let base_path = Path::new("/integration/tests/data/specs-json/test-cases");
        base_path.join(test_case)
    }
}
