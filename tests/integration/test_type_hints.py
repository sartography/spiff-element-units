import spiff_element_units
import typing
import unittest

class TypeHintTest(unittest.TestCase):

    @unittest.skip
    def test_has_type_hints(self):
        hints = typing.get_type_hints(spiff_element_units.cache_element_units_for_workflow)
        assert hints != {}
        
        hints = typing.get_type_hints(spiff_element_units.workflow_from_cached_element_unit)
        assert hints != {}
