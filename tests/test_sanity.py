from unittest import TestCase

from spiff_element_units import this_returns_true

class SanityTest(TestCase):
    def test_it_returns_true(self):
        assert this_returns_true()
