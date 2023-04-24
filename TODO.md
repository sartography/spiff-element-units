# TODOs

## v0.2.0

1. the element unit id used to make the manifest entry identifier is not good enough
   1. switch to sha2? - https://docs.rs/sha2/latest/sha2/

## v0.3.0

1. break up domain.rs
   1. move manifest to cache::manifest
   1. move *Spec to spiffworkflow_specs
   1. move ElementUnit* to element_units
1. form element unit that is full workflow with call activity subprocesses removed
   1. this element unit will have the first requirement flag (LAZY_CALL_ACTIVITIES)
1. don't always take the last element unit, let callers provide capabilities mask

## v0.4.0

1. new api to get element units without using the cache
1. new api to write existing element units to the cache
1. split cache into spiff-element-units-disk-cache
1. split spiffworkflow-specs to own lib

## rest

1. run integration tests as part of CI
1. run cargo tests as part of CI
1. maybe not run full build matrix on pr?
1. cleanup the auto generated descriptions (pypi page is blank)
1. make sure type hints work for the host applidcation