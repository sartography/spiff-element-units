use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WorkflowSpec {
    serializer_version: String,
    spec: ProcessSpec,
    subprocess_specs: Vec<ProcessSpec>,
}

#[derive(Deserialize, Debug)]
pub struct ProcessSpec {
    file: String,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    #[test]
    fn test_can_deserialize_no_tasks() -> Result<(), Box<dyn Error>> {
       let path = Path::new("../../tests/data/specs-json/test-cases/no-tasks/no-tasks.json");
       let file = File::open(path)?;
       let reader = BufReader::new(file);

       let workflow_spec = serde_json::from_reader(reader)?;

       Ok(())
    }
}