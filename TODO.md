1. run integration tests as part of CI
1. run cargo tests as part of CI
1. maybe not run full build matrix on pr?
1. bump SpiffWorkflow, move task.complete() to task.run()
1. cleanup the auto generated descriptions
1. separate the cache from the creation of element units
   1. at some point spiff-element-units-cache is its own lib
1. make sure type hints work for the host application
1. start to parse the specs json files
   1. read specs-json files from rust unit tests
   1. no-tasks that executes after serialization
1. add integration tests to compare our specs vs original specs
1. add integration tests to execute our specs