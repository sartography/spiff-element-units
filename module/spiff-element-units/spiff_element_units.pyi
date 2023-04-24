def cache_element_units_for_workflow(
        cache_dir: str,
        cache_key: str,
        workflow_specs_json: str) -> None:
    """
    Creates and caches elements units for the given workflow described by
    workflow_specs_json and associates them with cache_key for later 
    retrieval.
    """

def workflow_from_cached_element_unit(
        cache_dir: str,
        cache_key: str,
        element_id: str) -> str:
    """
    Returns a workflow described in specs json format that is capable of 
    executing an element unit that contains element_id and is associated 
    with cache_key.
    """
