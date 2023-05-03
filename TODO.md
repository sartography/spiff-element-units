# TODOs

## v0.3.0

1. test more in spiff-arena
   1. any process with manual/user tasks is not isolable, will revist after 0.3.0

1. new api to get element units without using the cache
1. new api to write existing element units to the cache
1. split cache into spiff-element-units-disk-cache
1. split spiffworkflow-specs to own lib
1. add support for the `all_specs` structure

## mostly unordered queue

1. run integration tests as part of CI
1. run cargo tests as part of CI
1. maybe not run full build matrix on pr?
   1. nothing real arch/os specific happening so just linux/x86_64 and sdist?
1. cleanup the auto generated descriptions (pypi page is blank)
1. make sure type hints work for the host applidcation
1. are there too many to_strings?
1. there is some issue in the tests when after an rmtree the cache dir can't be created again
   1. create a test for it?
   1. not sure if it affects callers that own the cache dir
1. can dmn `decision_table`s be lazy loaded for `BusinessRuleTasks`?
   1. for "free" if we make a subprocess out of them and swap the node for a `CallActivity`?
1. way to type the main spec and subprocess_specs differently
1. move things to Iterator (element_ids, processes, etc)
1. using the serde objects as domain objects is probably too expensive/restrictive
   1. have intermediate objects that are ref/slice based?
   1. most likely just do for element units before returning them out
   1. could help with spec mixin interactions?
   1. specific task types?
1. need to have test versions of element units
   1. build.rs? read from disk? just hardcode?
   1. how much do we leverage the integration tests vs rust tests?
   1. when we don't use the serde objects, unit tests become easier?
1, need to add the full workflow element unit for each subprocess spec to the manifest
   1. if subprocess add elements also
1. write log under each cache key
1. check for known task types?