from shutil import rmtree
from spiff_element_units import create_element_units
from unittest import TestCase

from .helper import read_specs_json, TEST_CACHE_DIR, TEST_CASES

class CacheTest(TestCase):

    # these tests write to the cache dir that is committed so changes to
    # how the cache is saved and element units are formed can be viewed
    # and verified in the pr.

    def test_can_write_sample_specs_to_cache(self):
        rmtree(TEST_CACHE_DIR)
        for key, data in TEST_CASES.items():
            specs = read_specs_json(data.relname)
            create_element_units(
                TEST_CACHE_DIR,
                key,
                specs)
