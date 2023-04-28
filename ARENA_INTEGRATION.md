# Spiff Arena Integration

## Current State

`spiff-element-units` v0.1.0 is added to the backend. Currently feature flagged off by default.
When on it saves each process instance workflow specs to the cache. When an instance is resumed
it can use the cached workflow to run the remaining portions of the process.

## Open Questions

1. seeding the cache with all workflow specs
1. updating the cache on save/upload/commit
1. is the current cache key optimal?
1. the full workflow spec is going to need to be loaded once per atomic unit
   1. if any of the workflow files change, the full workflow spec needs to be cached
   1. once cached, ideally we never need to do it again (until change) to start an instance
1. everything is namespaced by process id, confirm for subprocesses
   1. recent pr made names non unique between subprocesses?
1. for an atomic unit, what would happen if many instances cloned the same task tree?
1. what happens if a subset of a workflow is used at some stage?
1. what happens if "phantom" process/element ids are in this workflow subset
1. what happens if real process/element ids are in this workflow subset
1. what happens if call activity subspecs are not returned
   1. spiff will throw a key error
   1. can we requery and get the element unit for the call activity spec?
   1. ^ recursively for nested call activities
   1. can this be the same as if the called process was run directly?
1. what happens if anything that applies to call activities is done to subprocesses?
1. do lanes affect the ability to decompose anything?