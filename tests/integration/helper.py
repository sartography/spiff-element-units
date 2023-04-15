import json
from dataclasses import dataclass

from SpiffWorkflow.bpmn.serializer.workflow import BpmnWorkflowSerializer
from SpiffWorkflow.bpmn.workflow import BpmnWorkflow
from SpiffWorkflow.spiff.parser.process import SpiffBpmnParser
from SpiffWorkflow.spiff.serializer.config import SPIFF_SPEC_CONFIG

SPEC_CONVERTER = BpmnWorkflowSerializer.configure_workflow_spec_converter(SPIFF_SPEC_CONFIG)

TEST_CACHE = "tests/cache"

def _do_engine_steps(workflow):
    workflow.do_engine_steps()

@dataclass
class TestCaseData:
    relname: str
    process_id: str
    expected_result: dict
    cache_key: str
    executor: any

TEST_CASES = {
    "no-tasks": TestCaseData("no-tasks/no-tasks.json", "no_tasks", {}, "no-tasks", _do_engine_steps),
}

def read_specs_json(relname):
    with open(f"tests/data/specs-json/test-cases/{relname}") as f:
        return f.read()

def load_specs_json(relname):
    with open(f"tests/data/specs-json/test-cases/{relname}") as f:
        return json.load(f)

def converted_specs(specs, process_id):
    converted_specs = {k: SPEC_CONVERTER.restore(v) for k, v in specs.items()}
    top_level = converted_specs.pop(process_id)
    subprocesses = converted_specs
    return (top_level, subprocesses)
    
def workflow_from_specs_json(relname, process_id):
    specs = load_specs_json(relname)
    top_level, subprocesses = converted_specs(specs, process_id)
    return BpmnWorkflow(top_level, subprocesses)

def test_from_specs_json(key):
    test = TEST_CASES[key]
    workflow = workflow_from_specs_json(test.relname, test.process_id)
    test.executor(workflow)

    assert workflow.is_completed()
    assert workflow.data == test.expected_result
