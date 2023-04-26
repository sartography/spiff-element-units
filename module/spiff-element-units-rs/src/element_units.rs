use serde::{Deserialize, Serialize};

use sha2::{Digest, Sha256};
use std::error::Error;

use crate::domain::{ElementIDs, IndexedVec, Map, WorkflowSpec};
use crate::reader;

#[derive(Debug, Deserialize, Serialize)]
pub enum ElementUnit {
    FullWorkflow(WorkflowSpec),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ElementUnitType {
    FullWorkflow,
}

pub type ElementUnits = IndexedVec<ElementUnit>;
pub type ElementUnitsByProcessID = Map<ElementUnits>;

//
// constructs a map of element units grouped by process id that can be found in the
// given workflow specs json string.
//
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

impl ElementIDs for ElementUnit {
    fn push_element_ids(&self, ids: &mut Vec<String>) {
        match self {
            ElementUnit::FullWorkflow(w) => w.push_element_ids(ids),
        }
    }
}

impl ElementUnit {
    pub fn sha2_str(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}", self.r#type()));

        let mut element_ids = self.element_ids();
        element_ids.sort();

        for element_id in element_ids {
            hasher.update(element_id);
        }

        let hash = hasher.finalize();
        format!("{:x}", hash)
    }

    pub fn r#type(&self) -> ElementUnitType {
        match self {
            ElementUnit::FullWorkflow(_) => ElementUnitType::FullWorkflow,
        }
    }

    pub fn to_workflow_spec(&self) -> &WorkflowSpec {
        match self {
            ElementUnit::FullWorkflow(ws) => ws,
        }
    }
}
