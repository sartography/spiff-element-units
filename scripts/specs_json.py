import json
import os
from pathlib import Path

from SpiffWorkflow.bpmn.serializer.workflow import BpmnWorkflowSerializer
from SpiffWorkflow.spiff.parser.process import SpiffBpmnParser
from SpiffWorkflow.spiff.serializer.config import SPIFF_SPEC_CONFIG

SPEC_CONVERTER = BpmnWorkflowSerializer.configure_workflow_spec_converter(SPIFF_SPEC_CONFIG)

PROCESS_MODELS_DIR = "tests/data/process-models"

def bpmn_test_case(suffix):
    return f"{PROCESS_MODELS_DIR}/test-cases/{suffix}"

# TODO: as we add more do a convention where the folder, file and process id line up so
# this can be assumed for the majority of cases
PROCESS_IDS = {
    bpmn_test_case("countries-and-cities/country_cities.bpmn"): "Process_sypm122",
    bpmn_test_case("manual-tasks/manual_tasks.bpmn"): "Process_diu8ta2",
    bpmn_test_case("no-tasks/no-tasks.bpmn"): "no_tasks",
    bpmn_test_case("simple-call-activity/simple_call_activity.bpmn"): "Process_p4pfxhq",
    bpmn_test_case("single-task/single_task.bpmn"): "SingleTask_Process",
}

SUPPORTING_FILES = {
    bpmn_test_case("simple-call-activity/simple_call_activity.bpmn"): [
        bpmn_test_case("single-task/single_task.bpmn"),
    ],
}

def _required_files(bpmn_file):
    supporting_files = SUPPORTING_FILES.get(bpmn_file, [])
    return [bpmn_file] + supporting_files

def _to_dict(bpmn_file):
    parser = SpiffBpmnParser()
    bpmn_files = _required_files(bpmn_file)
    process_id = PROCESS_IDS[bpmn_file]
    parser.add_bpmn_files(bpmn_files)
    # TODO: backend make two find calls, one for the spec and one for the subprocesses
    specs = parser.find_all_specs()

    # spiffworkflow-backend does it like this, so
    bpmn_process_spec = parser.get_spec(process_id)
    subspecs = parser.get_subprocess_specs(process_id)

    #return {k: SPEC_CONVERTER.convert(v) for k, v in specs.items()}
    return {
        "spec": SPEC_CONVERTER.convert(bpmn_process_spec),
        "sub_processes": {k: SPEC_CONVERTER.convert(v) for k, v in subspecs.items()}
    }

def _write_dict_as_json(bpmn_file, dct):
    # correct, this is not very robust
    json_filename = bpmn_file.replace("process-models", "specs-json").replace(".bpmn", ".json")
    os.makedirs(os.path.dirname(json_filename), exist_ok=True)
    with open(json_filename, "w") as f:
        f.write(json.dumps(dct, indent=4, sort_keys=True))

if __name__ == "__main__":
    bpmn_files = Path(PROCESS_MODELS_DIR).rglob("*.bpmn")
    for bpmn_file in bpmn_files:
        bpmn_filename = str(bpmn_file)
        dct = _to_dict(bpmn_filename)
        _write_dict_as_json(bpmn_filename, dct)
