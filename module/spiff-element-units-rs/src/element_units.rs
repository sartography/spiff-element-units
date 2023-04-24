use crate::domain::{ElementIDs, ElementUnit, ElementUnits, WorkflowSpec};
use crate::reader;
use std::error::Error;

pub fn from_json_string(workflow_specs_json: &str) -> Result<ElementUnits, Box<dyn Error>> {
    let mut element_units = ElementUnits {
        items: Default::default(),
        index_map: Default::default(),
    };
    let workflow_spec = reader::read_string::<WorkflowSpec>(workflow_specs_json)?;
    let element_unit = ElementUnit::FullWorkflow(workflow_spec);
    let element_ids = element_unit.element_ids();

    element_units.push_for_keys(element_unit, &element_ids);

    Ok(element_units)
}
