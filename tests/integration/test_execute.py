import json
import spiff_element_units
import unittest

from .helper import (
    load_specs_json,
    test_workflow_from_specs,
    TEST_CACHE_DIR,
    TEST_CASES,
)

# TODO: test cases that need service tasks are not executed right now
# TODO: add delegate to mock out responses to fix ^
class ExecuteTest(unittest.TestCase):

    def test_can_execute_specs_json_test_cases(self):
        for k in TEST_CASES.keys():
            test = TEST_CASES[k]
            specs = load_specs_json(test.relname)
            test_workflow_from_specs(test, specs, None)

    def test_can_execute_specs_from_cache_for_element(self):
        for key, test in TEST_CASES.items():
            def specs_loader(process_id, element_id):
                specs_str = spiff_element_units.workflow_from_cached_element_unit(
                    TEST_CACHE_DIR,
                    key,
                    process_id,
                    element_id)
                specs = json.loads(specs_str)
                return specs
            
            # TODO: need to test invalid process ids and other elements ids
            specs = specs_loader(test.process_id, test.process_id)
            test_workflow_from_specs(test, specs, specs_loader)
