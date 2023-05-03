# spiff-element-units

`spiff-element-units` decomposes workflows into `element units` that can be executed in isolation. This allows for incrementally executing large workflows.

`spiff-element-units` is available on PyPi. Requirs Python >= 3.9.

The library requires callers to provide:

1. a serialized workflow spec in the form of `{"spec": {..}, "subprocess_specs": {..}}`  dumped to a json string
1. a cache key that is used when refering to the above serialized workflow spec
1. a directory used to store the element unit cache
1. process and element ids

The library currently assumes that:

1. A full workflow is loaded into `SpiffWorkflow` once to find all the specs. There have been recent changes to `SpiffWorkflow` that support loading single files. Once tested this assumption will be removed.
1. Callers can support lazy loading of subprocess specs. Please see `helpers.py` under `tests/integration` for an example. Once this stabilizes more documentation will be provided.

## The Python API:

Currently extremely simple to get started. Expect the public api to change as it matures.

`import spiff_element_units`

### Caching element units

```
def cache_element_units_for_workflow(
    cache_dir: str,
    cache_key: str,
    workflow_spec_json: str,
) -> None:
    
```

Forms element units for the workflow specs provided in json format and associates them with `cache_key`.

Raises `ValueError` if `workflow_spec_json` is invalid or `cache_dir` and child directories cannot be created.

### Getting cached element units

```
def workflow_from_cached_element_unit(
    cache_dir: str,
    cache_key: str,
    process_id: str,
    element_id: str,
) -> str:
```

Returns the json representation of a workflow capable of executing the first element unit available in `process_id` for the `element_id` associated with `cache_key`. This can be used to start or resume a process from previously cached element units.

Raises `ValueError` if `cache_dir` does not exist or a workflow cannot be formed with the given inputs.

## Development

`make dev-env` to set up the development environment.

`make compile` compiles the code.

`make tests` tests the code.

`make fmt` formats the code.

`make bindings` creates the shared library that can be loaded as a Python module.

`make run-integration-tests` runs the integration tests with the latest result of `make bindings`.

`make integration-tests` does the two steps above.

`make wheel` makes a wheel for local testing in external applications.