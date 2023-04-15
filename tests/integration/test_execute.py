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
            test_workflow_from_specs(test, specs)

    def test_can_execute_specs_from_cache_for_process(self):
        for key, test in TEST_CASES.items():
            specs_str = spiff_element_units.element_unit_for_process(
                TEST_CACHE_DIR,
                key,
                test.process_id)
            specs = json.loads(specs_str)
            test_workflow_from_specs(test, specs)

    def test_can_execute_specs_from_cache_for_element(self):
        for key, test in TEST_CASES.items():
            # TODO: this method currently does nothing with the element id
            # when it does this test will fail and need to provide valid
            # element ids via the test cases
            specs_str = spiff_element_units.element_unit_for_element(
                TEST_CACHE_DIR,
                key,
                test.process_id,
                "")
            specs = json.loads(specs_str)
            test_workflow_from_specs(test, specs)
