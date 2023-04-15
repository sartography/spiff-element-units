import json
import shutil
import spiff_element_units
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
            spiff_element_units.create_element_units(
                TEST_CACHE_DIR,
                key,
                specs)

        for key, data in TEST_CASES.items():
            element_unit_str = spiff_element_units.element_unit_for_process(
                TEST_CACHE_DIR,
                key,
                data.process_id)
            
            assert isinstance(element_unit_str, str)
            element_unit_dict = json.loads(element_unit_str)
            assert isinstance(element_unit_dict, dict)
            assert data.process_id in element_unit_dict

        # TODO: this method currently returns regardless of element id
        # when that changes this test will fail and needs to be checked
        # for each element id in the full workflow. right now we are just
        # checking that this call doesn't bomb/return unusable results
        for key, data in TEST_CASES.items():
            element_unit_str = spiff_element_units.element_unit_for_element(
                TEST_CACHE_DIR,
                key,
                data.process_id,
                "")
            
            assert isinstance(element_unit_str, str)
            element_unit_dict = json.loads(element_unit_str)
            assert isinstance(element_unit_dict, dict)
            assert data.process_id in element_unit_dict
            # TODO: check the element id is there also when they are needed
