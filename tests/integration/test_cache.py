import json
import os
import shutil
import spiff_element_units
import tempfile
import unittest

from .helper import (
    read_specs_json,
    test_workflow_from_specs,
    TEST_CACHE_DIR,
    TEST_CASES,
)

class CacheTest(unittest.TestCase):
    
    @classmethod
    def setUpClass(cls):
        # this rewrites the cache dir that is committed so changes to
        # how the cache is saved and element units are formed can be viewed
        # and verified in the pr.
        #
        # bit much going on but we do all this here to prevent any race conditions
        # between tests (reading from an old cache before the new write from another
        # test happens...), or having to ensure tests run in a certain order, etc.
        #
        # please do not write to the cache outside of this method.
        
        shutil.rmtree(TEST_CACHE_DIR)

        # we pretty print for sanity in the tests
        os.environ["SPIFF_ELEMENT_UNITS_PRETTY_JSON"] = "true"

        # prime the cache with all of our sample test cases
        for key, data in TEST_CASES.items():
            specs = read_specs_json(data.relname)
            spiff_element_units.cache_element_units_for_workflow(
                TEST_CACHE_DIR,
                key,
                specs)
        
        # sideloaded cases are those that don't originate from bpmn diagrams in the
        # test-cases group of the process models. These typically are processes that
        # are too complex to be considered a single test case but still something we
        # want to work with.
        
        for sideloaded in ["pp1"]:
            with open(f"tests/data/sideloaded/{sideloaded}.json") as f:
                specs = f.read()
        
                spiff_element_units.cache_element_units_for_workflow(
                    TEST_CACHE_DIR,
                    sideloaded,
                    specs)

    def test_read_sample_specs_from_cache(self):
        for key, data in TEST_CASES.items():
            element_unit_str = spiff_element_units.workflow_from_cached_element_unit(
                TEST_CACHE_DIR,
                key,
                data.process_id,
                data.process_id)
            
            assert isinstance(element_unit_str, str)
            element_unit_dict = json.loads(element_unit_str)
            assert isinstance(element_unit_dict, dict)
            assert data.process_id == element_unit_dict["spec"]["name"]
            assert element_unit_dict["serializer_version"].startswith("spiff-element-units-")

        # TODO: need to test passing in element ids

    def test_reading_from_empty_cache_throws_expected_exception(self):
        with tempfile.TemporaryDirectory() as empty_cache_dir:
            def read():
                spiff_element_units.workflow_from_cached_element_unit(
                    empty_cache_dir,
                    "somekey",
                    "someid",
                    "anotherid")
                
            self.assertRaises(ValueError, read)
            
    def test_reading_unknown_key_throws_expected_exception(self):
        def read():
            spiff_element_units.workflow_from_cached_element_unit(
                TEST_CACHE_DIR,
                "__akeyimadeupforthistest",
                "wontmatter",
                "whocares")
                
        self.assertRaises(ValueError, read)
    
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
