import spiff_element_units

from unittest import TestCase

from .helper import read_specs_json, TEST_CACHE

class CacheTest(TestCase):

    # these tests write to the cache dir that is committed so changes to
    # how the cache is saved and element units are formed can be viewed
    # and verified in the pr.

    def test_can_write_no_tasks_to_cache(self):
        specs = read_specs_json("no-tasks/no-tasks.json")
        spiff_element_units.create_element_units(
            TEST_CACHE,
            "1",
            specs)
        
