use serde::{Deserialize, Serialize};

use sha2::{Digest, Sha256};
use std::error::Error;

use crate::basis::{ElementIntrospection, IndexedVec, Map};
use crate::reader;
use crate::specs::{ProcessSpec, WorkflowSpec};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ElementUnit {
    FullWorkflow(WorkflowSpec),
    LazyCallActivities(ProcessSpec),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ElementUnitType {
    FullWorkflow,
    LazyCallActivities,
}

pub type ElementUnits = IndexedVec<ElementUnit>;
pub type ElementUnitsByProcessID = Map<ElementUnits>;

type ElementUnitForProcessID = (String, ElementUnit);

//
// constructs a map of element units grouped by process id that can be found in the
// given workflow specs json string.
//
pub fn from_json_string(
    workflow_specs_json: &str,
) -> Result<ElementUnitsByProcessID, Box<dyn Error>> {
    let workflow_spec = reader::read_string::<WorkflowSpec>(workflow_specs_json)?;
    let mut element_units_by_process_id = ElementUnitsByProcessID::new();

    for (process_id, element_unit) in element_units_for_workflow_spec(&workflow_spec) {
        element_units_by_process_id
            .entry(process_id)
            .and_modify(|eu| eu.push_element_unit(element_unit.clone()))
            .or_insert({
                let mut eu = ElementUnits::default();
                eu.push_element_unit(element_unit);
                eu
            });
    }

    Ok(element_units_by_process_id)
}

fn element_units_for_workflow_spec(workflow_spec: &WorkflowSpec) -> Vec<ElementUnitForProcessID> {
    let mut element_units = Vec::<ElementUnitForProcessID>::new();

    // the first element unit is always the full workflow. if nothing can be
    // decomposed we always have a fallback. this should not be permanent,
    // ideally we will always have an element unit at some point. the next step
    // away from needing this would be to only insert at the end for element ids
    // that have no element units. this is a conservative measure but is not
    // the most performant thing to do.

    let first_element_unit = ElementUnit::FullWorkflow(workflow_spec.clone());
    element_units.push((workflow_spec.spec.name.to_string(), first_element_unit));

    // next we see if we can start to isolate the process specs. this is the first
    // step to decomposing the workflow. currently under limited circumstances we can
    // remove subprocesses for call activities and turn them into their own element
    // units. instead of one large workflow this results in several smaller workflows
    // which can make our lives easier down the road.

    lazy_call_activity_element_units(&workflow_spec).map(|eu| element_units.extend(eu));

    element_units
}

fn lazy_call_activity_element_units(
    workflow_spec: &WorkflowSpec,
) -> Option<Vec<ElementUnitForProcessID>> {
    let process_spec = &workflow_spec.spec;

    let call_activity_spec_references = process_spec
        .isolable()
        .then_some(process_spec.call_activity_spec_references())
        .filter(|refs| !refs.is_empty())
        .filter(|refs| refs.len() == workflow_spec.subprocess_specs.len())?;

    let mut element_units = Vec::<ElementUnitForProcessID>::new();

    let element_unit = ElementUnit::LazyCallActivities(process_spec.clone());
    element_units.push((process_spec.name.to_string(), element_unit));

    for spec_ref in call_activity_spec_references {
        let process_spec = workflow_spec
            .subprocess_specs
            .get(&spec_ref)
            .filter(|spec| spec.isolable())?;

        let element_unit = ElementUnit::LazyCallActivities(process_spec.clone());
        element_units.push((spec_ref, element_unit));
    }

    Some(element_units)
}

impl ElementIntrospection for ElementUnit {
    fn push_element_ids(&self, ids: &mut Vec<String>) {
        use ElementUnit::*;

        match self {
            FullWorkflow(workflow_spec) => workflow_spec.push_element_ids(ids),
            LazyCallActivities(process_spec) => {
                process_spec.push_element_ids(ids);
            }
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
        use ElementUnit::*;

        match self {
            FullWorkflow(_) => ElementUnitType::FullWorkflow,
            LazyCallActivities(_) => ElementUnitType::LazyCallActivities,
        }
    }

    pub fn to_workflow_spec(&self) -> WorkflowSpec {
        use ElementUnit::*;

        match self {
            FullWorkflow(workflow_spec) => workflow_spec.clone(),
            LazyCallActivities(process_spec) => WorkflowSpec::from_process(process_spec),
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
