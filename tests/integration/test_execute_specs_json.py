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
