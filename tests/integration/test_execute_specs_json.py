from .helper import test_from_specs_json, TEST_CASES
from unittest import TestCase

# TODO: test cases that need service tasks are not executed right now
# TODO: add delegate to mock out responses to fix ^
class ExecuteSpecJsonFilesTest(TestCase):

    def test_can_execute_sample_specs_json(self):
        for k in TEST_CASES.keys():
            test_from_specs_json(k)
