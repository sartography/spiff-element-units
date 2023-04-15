from .helper import test_from_specs_json, workflow_from_specs_json
from unittest import TestCase

def _do_engine_steps(workflow):
    workflow.do_engine_steps()

def _test(relname, process_id, executor, result):
    workflow = workflow_from_specs_json(relname, process_id)
    executor(workflow)
        
    assert workflow.is_completed()
    assert workflow.data == result

# TODO: test cases that need service tasks are not executed right now
# TODO: add delegate to mock out responses to fix ^
class ExecuteSpecJsonFilesTest(TestCase):
    
    def test_no_tasks_executes(self):
        test_from_specs_json("no-tasks")

    def test_single_task_executes(self):
        test_from_specs_json("single-task")

    def test_simple_call_activity_executes(self):
        test_from_specs_json("simple-call-activity")

    def test_manual_tasks_executes(self):
        test_from_specs_json("manual-tasks")
