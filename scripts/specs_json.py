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
    parser.add_bpmn_files(bpmn_files)
    
    specs = parser.find_all_specs()
    return {k: SPEC_CONVERTER.convert(v) for k, v in specs.items()}

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
