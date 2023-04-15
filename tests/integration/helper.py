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

def _two_manual_tasks(workflow):
    workflow.do_engine_steps()
    workflow.get_ready_user_tasks()[0].complete()
    workflow.do_engine_steps()
    workflow.get_ready_user_tasks()[0].complete()
    workflow.do_engine_steps()

@dataclass
class TestCaseData:
    relname: str
    process_id: str
    executor: any
    expected_result: dict

TEST_CASES = {
    "no-tasks": TestCaseData("no-tasks/no-tasks.json", "no_tasks", _do_engine_steps, {}),
    "single-task": TestCaseData("single-task/single_task.json", "SingleTask_Process", _do_engine_steps, {"x": 1}),
    "simple-call-activity": TestCaseData("simple-call-activity/simple_call_activity.json", "Process_p4pfxhq", _do_engine_steps, {"x": 1}),
    "manual-tasks": TestCaseData("manual-tasks/manual_tasks.json", "Process_diu8ta2", _two_manual_tasks, {}),
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
