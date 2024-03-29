import json
from dataclasses import dataclass

from SpiffWorkflow.bpmn.serializer.workflow import BpmnWorkflowSerializer
from SpiffWorkflow.bpmn.workflow import BpmnWorkflow
from SpiffWorkflow.spiff.parser.process import SpiffBpmnParser
from SpiffWorkflow.spiff.serializer.config import SPIFF_SPEC_CONFIG
from SpiffWorkflow.task import TaskState

SPEC_CONVERTER = BpmnWorkflowSerializer.configure_workflow_spec_converter(SPIFF_SPEC_CONFIG)
SERIALIZER = BpmnWorkflowSerializer(SPEC_CONVERTER)

TEST_CACHE_DIR = "tests/cache"

def _lazy_load_specs(workflow, specs_loader):
    tasks = workflow.get_tasks(TaskState.DEFINITE_MASK)
    loaded_specs = set(workflow.subprocess_specs.keys())
    for task in tasks:
        if task.task_spec.description != "Call Activity":
            continue
        spec_to_check = task.task_spec.spec
        
        if spec_to_check not in loaded_specs:
            lazy_spec, lazy_subprocess_specs = converted_specs(specs_loader(spec_to_check, spec_to_check))
            lazy_subprocess_specs[spec_to_check] = lazy_spec

            for name, spec in lazy_subprocess_specs.items():
                if name not in loaded_specs:
                    workflow.subprocess_specs[name] = spec
                    loaded_specs.add(name)

def _run_tasks(workflow, specs_loader):
    def next_task(wf):
        tasks = workflow.get_tasks(TaskState.READY)
        return tasks[0] if len(tasks) > 0 else None
        
    while not workflow.is_completed():
        task = next_task(workflow)
        if task is None:
            break
        _lazy_load_specs(workflow, specs_loader)
        task.run()
        workflow.refresh_waiting_tasks()

        wf_dict = SERIALIZER.workflow_to_dict(workflow)
        workflow = SERIALIZER.workflow_from_dict(wf_dict)
        
    return workflow

@dataclass
class TestCaseData:
    relname: str
    process_id: str
    executor: any
    expected_result: dict

TEST_CASES = {
    "manual-tasks": TestCaseData("manual-tasks/manual_tasks.json", "Process_diu8ta2", _run_tasks, {}),
    "multiple-call-activities": TestCaseData("mutliple-call-activities/multiple_call_activities.json", "Process_90mmqlw", _run_tasks, {"x": 1}),
    "nested-call-activities": TestCaseData("nested-call-activities/nested_call_activity.json", "Process_cqu23d1", _run_tasks, {"x": 1}),
    "no-tasks": TestCaseData("no-tasks/no-tasks.json", "no_tasks", _run_tasks, {}),
    "single-task": TestCaseData("single-task/single_task.json", "SingleTask_Process", _run_tasks, {"x": 1}),
    "simple-call-activity": TestCaseData("simple-call-activity/simple_call_activity.json", "Process_p4pfxhq", _run_tasks, {"x": 1}),
    "simple-subprocess": TestCaseData("simple-subprocess/simple_subprocess.json", "Process_vv0fdgv", _run_tasks, {"x": 1}),
}

def read_specs_json(relname):
    with open(f"tests/data/specs-json/test-cases/{relname}") as f:
        return f.read()

def load_specs_json(relname):
    with open(f"tests/data/specs-json/test-cases/{relname}") as f:
        return json.load(f)

def converted_specs(specs):
    converted_specs = {
        "spec": SPEC_CONVERTER.restore(specs["spec"]),
        "subprocess_specs": {
            k: SPEC_CONVERTER.restore(v) for k, v in specs["subprocess_specs"].items()
        },
    }
    top_level = converted_specs["spec"]
    subprocesses = converted_specs["subprocess_specs"]
    return (top_level, subprocesses)

def workflow_from_specs(specs):
    top_level, subprocesses = converted_specs(specs)
    return BpmnWorkflow(top_level, subprocesses)

def test_workflow_from_specs(test, specs, specs_loader):
    workflow = workflow_from_specs(specs)
    workflow = test.executor(workflow, specs_loader)
    assert workflow.is_completed()
    assert workflow.data == test.expected_result

