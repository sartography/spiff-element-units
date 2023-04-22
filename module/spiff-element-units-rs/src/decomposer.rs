use std::collections::BTreeMap;
use std::error::Error;

use crate::domain::{ElementUnit, ElementUnitsByID, WorkflowSpec};
use crate::reader;

pub fn decompose_json_string(
    workflow_specs_json: &str,
) -> Result<ElementUnitsByID, Box<dyn Error>> {
    let mut map: ElementUnitsByID = BTreeMap::new();
    let workflow_spec = reader::read_string::<WorkflowSpec>(workflow_specs_json)?;

    // TODO: add in workflow_spec for each element

    Ok(map)
}
