use crate::domain::{ElementIDs, ElementUnit, ElementUnits, ElementUnitsByProcessID, WorkflowSpec};
use crate::reader;
use std::error::Error;

pub fn from_json_string(
    workflow_specs_json: &str,
) -> Result<ElementUnitsByProcessID, Box<dyn Error>> {
    let mut element_units = ElementUnits {
        items: Default::default(),
        index_map: Default::default(),
    };
    let workflow_spec = reader::read_string::<WorkflowSpec>(workflow_specs_json)?;
    let process_id = workflow_spec.spec.name.to_string();
    let element_unit = ElementUnit::FullWorkflow(workflow_spec);
    let element_ids = element_unit.element_ids();

    element_units.push_for_keys(element_unit, &element_ids);

    let mut element_units_by_process_id = ElementUnitsByProcessID::new();
    element_units_by_process_id.insert(process_id, element_units);

    Ok(element_units_by_process_id)
}
