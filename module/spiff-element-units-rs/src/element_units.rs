use std::error::Error;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::basis::{ElementIntrospection, IndexedVec, Map};
use crate::reader;
use crate::specs::{ProcessSpec, RestMap, SubprocessSpecs, WorkflowSpec};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ElementUnit {
    FullWorkflow(WorkflowSpec),
    LazyCallActivities(ProcessSpec, SubprocessSpecs),
    PromotedCallActivity(ProcessSpec, SubprocessSpecs),
    ResumableCallActivity(String, ProcessSpec, SubprocessSpecs),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ElementUnitType {
    FullWorkflow,
    LazyCallActivities,
    PromotedCallActivity,
    ResumableCallActivity,
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

    // we have a restriction in place that all element ids must be unique across
    // all process specs. this is to prevent an ambiguos situation where a element
    // id could point to multiple element units. like all of our restrictions ideally
    // this is lifted in the future but better to run the whole workflow than the
    // wrong portion. this does mean any workflow with collapsed/expanded subprocesses
    // cannot be decomposed.

    if !workflow_spec.has_unique_element_ids() {
        return element_units;
    }

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

    // this wave of element units consists of three layers that work together
    //
    // 1 - the full workflow with all call activity subprocess specs removed
    //     the caller is assumed to be able to lazy load the specs when needed

    let call_activity_spec_references = process_spec
        .isolable()
        .then_some(process_spec.call_activity_spec_references())
        .filter(|refs| !refs.is_empty())
        .filter(|refs| refs.len() == workflow_spec.subprocess_specs.len())?;

    let mut element_units = Vec::<ElementUnitForProcessID>::new();
    let subprocess_specs = Map::<ProcessSpec>::new();

    let element_unit = ElementUnit::LazyCallActivities(process_spec.clone(), subprocess_specs);
    element_units.push((process_spec.name.to_string(), element_unit));

    // 2 - each call activity subprocess elevated to a top level process to facilitate
    //     lazy loading

    for spec_ref in call_activity_spec_references {
        let subprocess_spec = workflow_spec
            .subprocess_specs
            .get(&spec_ref)
            .filter(|spec| spec.isolable())
            .filter(|spec| spec.call_activity_spec_references().len() == 0)?;

        let mut subprocess_specs = Map::<ProcessSpec>::new();
        let element_unit =
            ElementUnit::PromotedCallActivity(subprocess_spec.clone(), subprocess_specs.clone());
        element_units.push((spec_ref.clone(), element_unit));

        // 3 - a workflow per call activity with its subprocess specs loaded to facilitate
        //     resuming a workflow from an element within a call activity

        subprocess_specs.insert(subprocess_spec.name.to_string(), subprocess_spec.clone());

        let element_unit =
            ElementUnit::ResumableCallActivity(spec_ref, process_spec.clone(), subprocess_specs);
        element_units.push((process_spec.name.to_string(), element_unit));
    }

    Some(element_units)
}

impl ElementIntrospection for ElementUnit {
    fn push_element_ids(&self, ids: &mut Vec<String>) {
        use ElementUnit::*;

        match self {
            FullWorkflow(workflow_spec) => workflow_spec.push_element_ids(ids),
            LazyCallActivities(process_spec, subprocess_specs)
            | PromotedCallActivity(process_spec, subprocess_specs) => {
                self.push_all_process_element_ids(ids, &process_spec, &subprocess_specs)
            }
            ResumableCallActivity(spec_ref, process_spec, subprocess_specs) => self
                .push_resumable_process_element_ids(
                    ids,
                    &spec_ref,
                    &process_spec,
                    &subprocess_specs,
                ),
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
            LazyCallActivities(_, _) => ElementUnitType::LazyCallActivities,
            PromotedCallActivity(_, _) => ElementUnitType::PromotedCallActivity,
            ResumableCallActivity(_, _, _) => ElementUnitType::ResumableCallActivity,
        }
    }

    pub fn to_workflow_spec(self) -> WorkflowSpec {
        use ElementUnit::*;

        match self {
            FullWorkflow(workflow_spec) => workflow_spec,
            LazyCallActivities(spec, subprocess_specs)
            | PromotedCallActivity(spec, subprocess_specs)
            | ResumableCallActivity(_, spec, subprocess_specs) => WorkflowSpec {
                spec,
                subprocess_specs,
                rest: RestMap::default(),
            },
        }
    }

    fn push_all_process_element_ids(
        &self,
        ids: &mut Vec<String>,
        process_spec: &ProcessSpec,
        subprocess_specs: &SubprocessSpecs,
    ) {
        process_spec.push_element_ids(ids);

        for (_, subprocess_spec) in subprocess_specs {
            subprocess_spec.push_element_ids(ids);
        }
    }

    fn push_resumable_process_element_ids(
        &self,
        ids: &mut Vec<String>,
        spec_ref: &String,
        process_spec: &ProcessSpec,
        subprocess_specs: &SubprocessSpecs,
    ) {
        // a resumable call activity element unit is designed to be called using either
        // the process referenced by the call activity
        ids.push(spec_ref.to_string());

        // the name of the call activities that reference the spec
	for call_activity in process_spec.call_activities_referencing_spec(spec_ref) {
	    call_activity.push_element_ids(ids);
	}

        // any element within the call activity
        subprocess_specs
            .get(spec_ref)
            .map(|subprocess| subprocess.push_element_ids(ids));
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
