import json

from SpiffWorkflow.bpmn.serializer.workflow import BpmnWorkflowSerializer
from SpiffWorkflow.bpmn.workflow import BpmnWorkflow
from SpiffWorkflow.spiff.parser.process import SpiffBpmnParser
from SpiffWorkflow.spiff.serializer.config import SPIFF_SPEC_CONFIG

from unittest import TestCase

# TODO: most likely will want these moves to a test helper file
SPEC_CONVERTER = BpmnWorkflowSerializer.configure_workflow_spec_converter(SPIFF_SPEC_CONFIG)

def _load_specs_json(relname):
    with open(f"tests/data/specs-json/test-cases/{relname}") as f:
        return json.load(f)

def _converted_specs(specs, process_id):
    converted_specs = {k: SPEC_CONVERTER.restore(v) for k, v in specs.items()}
    top_level = converted_specs.pop(process_id)
    subprocesses = converted_specs
    return (top_level, subprocesses)
    
def _workflow_from_specs_json(relname, process_id):
    specs = _load_specs_json(relname)
    top_level, subprocesses = _converted_specs(specs, process_id)
    return BpmnWorkflow(top_level, subprocesses)

def _do_engine_steps(workflow):
    workflow.do_engine_steps()

def _test(relname, process_id, executor, result):
    workflow = _workflow_from_specs_json(relname, process_id)
    executor(workflow)
        
    assert workflow.is_completed()
    assert workflow.data == result

# TODO: test cases that need service tasks are not executed right now
# TODO: add delegate to mock out responses to fix ^
class ExecuteSpecJsonFilesTest(TestCase):
    
    def test_no_tasks_executes(self):
        _test("no-tasks/no-tasks.json", "no_tasks", _do_engine_steps, {})

    def test_single_task_executes(self):
        _test("single-task/single_task.json", "SingleTask_Process", _do_engine_steps, {"x": 1})

    def test_simple_call_activity_executes(self):
        _test("simple-call-activity/simple_call_activity.json", "Process_p4pfxhq", _do_engine_steps, {"x": 1})

    def test_manual_tasks_executes(self):
        def executor(workflow):
            workflow.do_engine_steps()
            workflow.get_ready_user_tasks()[0].complete()
            workflow.do_engine_steps()
            workflow.get_ready_user_tasks()[0].complete()
            workflow.do_engine_steps()

        _test("manual-tasks/manual_tasks.json", "Process_diu8ta2", executor, {})
