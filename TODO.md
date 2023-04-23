# TODOs

# v0.2.0

1. the element unit id used to make the manifest entry identifier is not good enough
   1. switch to sha2? - https://docs.rs/sha2/latest/sha2/

# v0.3.0

1. break up domain.rs
1. form element unit that is full workflow with call activity subprocesses removed
1. don't always take the last element unit, let callers provide capabilities mask

# rest

1. run integration tests as part of CI
1. run cargo tests as part of CI
1. maybe not run full build matrix on pr?
1. cleanup the auto generated descriptions (pypi page is blank)
1. make sure type hints work for the host applidcation