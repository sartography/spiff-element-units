use std::error::Error;

use crate::domain::{ElementUnit, ElementUnits, WorkflowSpec};
use crate::reader;

pub fn from_json_string(workflow_specs_json: &str) -> Result<ElementUnits, Box<dyn Error>> {
    let mut element_units = ElementUnits {
        items: Default::default(),
        index_map: Default::default(),
    };
    let workflow_spec = reader::read_string::<WorkflowSpec>(workflow_specs_json)?;
    let element_unit = ElementUnit::FullWorkflow(workflow_spec);

    // TODO: add in workflow_spec for each element

    Ok(element_units)
}
