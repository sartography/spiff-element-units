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

# TODO: to leverage the unittest-parallel, is it better to have many small test suites?
# does the number of files matter or just the TestCase classes?
class ExecuteSpecJsonFilesTest(TestCase):
    def test_no_tasks_executes(self):
        workflow = _workflow_from_specs_json("no-tasks/no-tasks.json", "no_tasks")
        workflow.do_engine_steps()
        
        assert workflow.is_completed()
        assert workflow.data == {}
