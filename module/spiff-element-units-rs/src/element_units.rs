use std::error::Error;

use crate::domain::{ElementIDs, ElementUnit, ElementUnits, WorkflowSpec};
use crate::reader;

pub fn from_json_string(workflow_specs_json: &str) -> Result<ElementUnits, Box<dyn Error>> {
    let mut element_units = ElementUnits {
        items: Default::default(),
        index_map: Default::default(),
    };
    let workflow_spec = reader::read_string::<WorkflowSpec>(workflow_specs_json)?;
    let element_unit = ElementUnit::FullWorkflow(workflow_spec);

    // TODO: move this into the ElementIDs trait, so we can handle `with_capacity` better
    // and just say `element_unit.element_ids()`
    let mut element_ids: Vec<String> = vec![];
    element_unit.push_element_ids(&mut element_ids);
    
    element_units.push_for_keys(element_unit, &element_ids);

    Ok(element_units)
}
