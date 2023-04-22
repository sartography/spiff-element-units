1. run integration tests as part of CI
1. run cargo tests as part of CI
1. maybe not run full build matrix on pr?
1. bump SpiffWorkflow, move task.complete() to task.run()
1. cleanup the auto generated descriptions
1. separate the cache from the creation of element units
   1. at some point spiff-element-units-cache is its own lib in the workspace
1. make sure type hints work for the host application
1. write first element unit which is our workflow specs json
   1. add `manifest.json` to cache which maps element ids -> json file(s)
   1. use elements_units+manifest to write individual element units
1. the element unit id used to make the manifest entry identifier is not good enough
   1. switch to sha2? - https://docs.rs/sha2/latest/sha2/