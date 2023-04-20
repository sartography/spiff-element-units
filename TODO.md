1. look into stable sort for serde_json to_string to reduce commit noise
1. move away from parser to reader/writer
   1. cache.rs just provides entries which include path to write to
1. run integration tests as part of CI
1. run cargo tests as part of CI
1. maybe not run full build matrix on pr?
1. bump SpiffWorkflow, move task.complete() to task.run()
1. cleanup the auto generated descriptions
1. separate the cache from the creation of element units
   1. at some point spiff-element-units-cache is its own lib in the workspace
1. make sure type hints work for the host application
