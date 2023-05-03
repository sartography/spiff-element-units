import json
import os
from pathlib import Path

from SpiffWorkflow.bpmn.serializer.workflow import BpmnWorkflowSerializer
from SpiffWorkflow.spiff.parser.process import SpiffBpmnParser
from SpiffWorkflow.spiff.serializer.config import SPIFF_SPEC_CONFIG

SPEC_CONVERTER = BpmnWorkflowSerializer.configure_workflow_spec_converter(SPIFF_SPEC_CONFIG)
SERIALIZER_VERSION = "spiff-element-units-integration"

PROCESS_MODELS_DIR = "tests/data/process-models"

def bpmn_test_case(suffix):
    return f"{PROCESS_MODELS_DIR}/test-cases/{suffix}"

MULTIPLE_CALL_ACTIVITIES = bpmn_test_case("mutliple-call-activities/multiple_call_activities.bpmn")
NESTED_CALL_ACTIVITY = bpmn_test_case("nested-call-activities/nested_call_activity.bpmn")
SIMPLE_CALL_ACTIVITY = bpmn_test_case("simple-call-activity/simple_call_activity.bpmn")
SIMPLE_SUBPROCESS = bpmn_test_case("simple-subprocess/simple_subprocess.bpmn")
SINGLE_TASK = bpmn_test_case("single-task/single_task.bpmn")

SUPPORTING_FILES = {
    MULTIPLE_CALL_ACTIVITIES: [
        SINGLE_TASK,
        bpmn_test_case("manual-tasks/manual_tasks.bpmn"),
        bpmn_test_case("no-tasks/no-tasks.bpmn"),
    ],
    NESTED_CALL_ACTIVITY: [
        SIMPLE_CALL_ACTIVITY,
        SINGLE_TASK,
    ],
    SIMPLE_CALL_ACTIVITY: [
        SINGLE_TASK,
    ],
}

PROCESS_IDS = {
    MULTIPLE_CALL_ACTIVITIES: "Process_90mmqlw",
    NESTED_CALL_ACTIVITY: "Process_cqu23d1",
    SIMPLE_CALL_ACTIVITY: "Process_p4pfxhq",
    SIMPLE_SUBPROCESS: "Process_vv0fdgv",
}

def _required_files(bpmn_file):
    supporting_files = SUPPORTING_FILES.get(bpmn_file, [])
    return [bpmn_file] + supporting_files

def _to_dict(bpmn_file):
    parser = SpiffBpmnParser()
    bpmn_files = _required_files(bpmn_file)
    parser.add_bpmn_files(bpmn_files)
    
    specs = parser.find_all_specs()
    converted_specs = {k: SPEC_CONVERTER.convert(v) for k, v in specs.items()}

    if len(converted_specs) == 1:
        for k in converted_specs.keys():
            process_spec = converted_specs[k]
        subspecs = {}
    else:
        process_spec = converted_specs.pop(PROCESS_IDS[bpmn_file])
        subspecs = converted_specs

    return {
        "spec": process_spec,
        "subprocess_specs": subspecs,
        "serializer_version": SERIALIZER_VERSION,
    }

def _write_dict_as_json(bpmn_file, dct):
    json_filename = bpmn_file.replace("/process-models/", "/specs-json/").replace(".bpmn", ".json")
    os.makedirs(os.path.dirname(json_filename), exist_ok=True)
    with open(json_filename, "w") as f:
        f.write(json.dumps(dct, indent=4, sort_keys=True))

if __name__ == "__main__":
    bpmn_files = Path(PROCESS_MODELS_DIR).rglob("*.bpmn")
    for bpmn_file in bpmn_files:
        bpmn_filename = str(bpmn_file)
        dct = _to_dict(bpmn_filename)
        _write_dict_as_json(bpmn_filename, dct)
