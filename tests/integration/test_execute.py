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
