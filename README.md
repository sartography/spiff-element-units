# spiff-element-units

`spiff-element-units` decomposes workflows into `element units` that can be executed in isolation. This allows for incrementally executing large workflows.

The library requires callers to provide:

1. a serialized workflow spec from `find_all_specs` dumped to a json string
1. a cache key that is used when refering to the above serialized workflow spec
1. a directory used to store the element unit cache
1. process and element ids

The library currently assumes that:

1. A full workflow is loaded into `SpiffWorkflow` once to `find_all_specs`. This requirement may change in the future.

## The api:

Currently extremely simple to get started. Expect the public api to change as it matures.

`import spiff_element_units`

### Forming element units

```
def create_element_units(
    cache_dir: str,
    cache_key: str,
    workflow_spec_json: str,
) -> None:
    
```

Forms element units for the workflow spec provided in json format and associates them with `cache_key`.

TODO: exceptions raised.

### Getting element units

```
def element_unit_for_process(
    cache_dir: str,
    cache_key: str,
    process_id: str,
) -> str:
```

Returns the json representation of the first element unit available for the process with `process_id` in the workflow spec associated with `cache_key`. This can be used to start a process from previously cached element units without knowing the first element id.

TODO: exceptions raised.

```
def element_unit_for_element(
    cache_dir: str,
    cache_key: str,
    process_id: str,
    element_id: str,
) -> str:
```

Returns the json representation of the element unit for the element with `element_id` in process `process_id` in the workflow spec associated with `cache_key`. This can be used to resume a process from previously cached element units using a known element id.

TODO: exceptions raised.

## Development

`make dev-env` to set up the development environment.

`make compile` compiles the code.

`make tests` tests the code.

`make fmt` formats the code.

`make bindings` creates the shared library that can be loaded as a Python module.

`make run-integration-tests` runs the integration tests with the latest result of `make bindings`.

`make integration-tests` does the two steps above.

`make wheel` makes a wheel for local testing in external applications.