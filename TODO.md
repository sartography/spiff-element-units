# TODOs

## v0.3.0

1. break up domain.rs
   1. move manifest to cache::manifest
   1. move *Spec to spiffworkflow_specs
   1. move ElementUnit* to element_units
1. form element unit that is full workflow with call activity subprocesses removed
   1. this element unit will have the first requirement flag (LAZY_CALL_ACTIVITIES)
1. don't always take the last element unit, let callers provide capabilities mask
1. env var to pretty print, off by default, set in tests for committed cache
1. don't parrot `serializer_version`, recognize it and return our own
   1. embed version: https://stackoverflow.com/questions/27840394/how-can-a-rust-program-access-metadata-from-its-cargo-package
1. manifest needs to be by process id
1. build element units for subprocesses that map to a call activity
   1. same as if that process was passed in with the same cache key

## v0.4.0

1. new api to get element units without using the cache
1. new api to write existing element units to the cache
1. split cache into spiff-element-units-disk-cache
1. split spiffworkflow-specs to own lib

## rest

1. run integration tests as part of CI
1. run cargo tests as part of CI
1. maybe not run full build matrix on pr?
   1. nothing real arch/os specific happening so just linux/x86_64 and sdist?
1. cleanup the auto generated descriptions (pypi page is blank)
1. make sure type hints work for the host applidcation
1. are there too many to_strings?