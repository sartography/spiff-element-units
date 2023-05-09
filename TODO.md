# TODOs

## v0.3.1

1. for each call activity, element unit of lazy at the top level + its spec as subprocess
   1. push for call activity id and each of its recognized task id (no top level/process ids)
   1. ResumableCallActivity
1. in the integration tests, do some save/restore (maybe on human task?)
1. Rename the *CallActivity* element units to Subprocess?

## mostly unordered queue

1. new api to get element units without using the cache
1. new api to write existing element units to the cache
1. split cache into spiff-element-units-disk-cache
1. split spiffworkflow-specs to own lib
1. add support for the `all_specs` structure
1. run integration tests as part of CI
1. run cargo tests as part of CI
1. maybe not run full build matrix on pr?
   1. nothing real arch/os specific happening so just linux/x86_64 and sdist?
1. cleanup the auto generated descriptions (pypi page is blank)
1. are there too many to_strings?
   1. yes tied into domain objects vs serde objects
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
1. write log under each cache key
1. "mutliple-call-activities/multiple_call_activities.json" naming has tripped me up 2x now
1. look at more (self) -> X to transition objects instead of clone