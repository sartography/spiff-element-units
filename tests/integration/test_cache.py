import json
import shutil
import spiff_element_units
import tempfile
import unittest

from .helper import read_specs_json, TEST_CACHE_DIR, TEST_CASES

class CacheTest(unittest.TestCase):

    def test_read_and_write_sample_specs_to_cache(self):
        # this test rewrites the cache dir that is committed so changes to
        # how the cache is saved and element units are formed can be viewed
        # and verified in the pr.
        #
        # bit overloaded for one test but don't want to have a race
        # condition where a read is ok then a write introduces an
        # error that break reads when the next test is run. also don't
        # want separate tests that have to be run in a certain order.
        
        shutil.rmtree(TEST_CACHE_DIR)
        for key, data in TEST_CASES.items():
            specs = read_specs_json(data.relname)
            spiff_element_units.cache_element_units_for_workflow(
                TEST_CACHE_DIR,
                key,
                specs)

        for key, data in TEST_CASES.items():
            element_unit_str = spiff_element_units.workflow_from_cached_element_unit(
                TEST_CACHE_DIR,
                key,
                data.process_id)
            
            assert isinstance(element_unit_str, str)
            element_unit_dict = json.loads(element_unit_str)
            assert isinstance(element_unit_dict, dict)
            assert data.process_id == element_unit_dict["spec"]["name"]

        # TODO: when supported, need to test passing in element ids

    def test_reading_from_empty_cache_throws_expected_exception(self):
        with tempfile.TemporaryDirectory() as empty_cache_dir:
            def read():
                spiff_element_units.workflow_from_cached_element_unit(
                    empty_cache_dir,
                    "somekey",
                    "someid")
                
            self.assertRaises(FileNotFoundError, read)
            
    def test_reading_unknown_key_throws_expected_exception(self):
        def read():
            spiff_element_units.workflow_from_cached_element_unit(
                TEST_CACHE_DIR,
                "__akeyimadeupforthistest",
                "wontmatter")
                
        self.assertRaises(FileNotFoundError, read)
            
