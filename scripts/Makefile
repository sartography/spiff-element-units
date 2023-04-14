
#
# TODO: re-integrate this
#


TEST_DATA_DIR := tests/data

PROCESS_MODELS_DIR := $(TEST_DATA_DIR)/process-models
SPECS_JSON_DIR := $(TEST_DATA_DIR)/specs-json

# Used to move back and forth between the process-models clone
CDUP_TO_PROCESS_MODELS_CLONE_DIR := ../../jbirddog/process-models
CDUP_BACK_TO_THIS_CLONE_DIR := ../../sartography/spiff-element-units

#
# used to copy in/parse files from my process-models, probably will want to move these to
# their own repo at some point? thought about a submodule but I don't really love them. 
#

.PHONY: copy-process-models
copy-process-models:
	rm -rf $(PROCESS_MODELS_DIR)
	mkdir -p $(PROCESS_MODELS_DIR)
	cd $(CDUP_TO_PROCESS_MODELS_CLONE_DIR) && \
	find . -name "*.bpmn" -exec rsync -R {} $(CDUP_BACK_TO_THIS_CLONE_DIR)/$(PROCESS_MODELS_DIR) \;

.PHONY: script-specs-json
script-specs-json:
	rm -rf $(SPECS_JSON_DIR)
	$(DEV_AS_ME) python scripts/specs_json.py
