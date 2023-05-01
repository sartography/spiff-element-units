use serde::{Deserialize, Serialize};

use sha2::{Digest, Sha256};
use std::error::Error;

use crate::basis::{ElementIntrospection, IndexedVec, Map};
use crate::reader;
use crate::specs::{ProcessSpec, WorkflowSpec};

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    let workflow_spec = reader::read_string::<WorkflowSpec>(workflow_specs_json)?;
    let mut element_units_by_process_id = ElementUnitsByProcessID::new();

    {
        let mut element_units = ElementUnits::default();
        let process_id = workflow_spec.spec.name.to_string();

        push_element_units_for_workflow_spec(&workflow_spec, &mut element_units);
        push_element_units_for_process_spec(&workflow_spec.spec, &mut element_units);

        element_units_by_process_id.insert(process_id, element_units);
    }

    for (process_id, process_spec) in workflow_spec.subprocess_specs {
        let mut element_units = ElementUnits::default();

        push_element_units_for_process_spec(&process_spec, &mut element_units);

	if element_units.items.len() > 0 {
        element_units_by_process_id.insert(process_id, element_units);
	}
    }

    Ok(element_units_by_process_id)
}

fn push_element_units_for_workflow_spec(
    workflow_spec: &WorkflowSpec,
    element_units: &mut ElementUnits,
) {
    // the first element unit is always the full workflow. if nothing can be
    // decomposed we always have a fallback. this should not be permanent,
    // ideally we will always have an element unit at some point. the next step
    // away from needing this would be to only insert at the end for element ids
    // that have no element units. this is a conservative measure but is not
    // the most performant thing to do.

    let first_element_unit = ElementUnit::FullWorkflow(workflow_spec.clone());
    element_units.push_element_unit(first_element_unit);
}

fn push_element_units_for_process_spec(
    _process_spec: &ProcessSpec,
    _element_units: &mut ElementUnits,
) {
}

impl ElementIntrospection for ElementUnit {
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

impl ElementUnits {
    pub fn push_element_unit(&mut self, element_unit: ElementUnit) {
        let element_ids = element_unit.element_ids();

        self.push_for_keys(element_unit, &element_ids);
    }
}

impl Default for ElementUnits {
    fn default() -> Self {
        ElementUnits {
            items: Default::default(),
            index_map: Default::default(),
        }
    }
}
