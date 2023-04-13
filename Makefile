SRC := spiff_element_units
TESTS := tests

TEST_DATA_DIR := $(TESTS)/data

PROCESS_MODELS_DIR := $(TEST_DATA_DIR)/process-models
SPECS_JSON_DIR := $(TEST_DATA_DIR)/specs-json

# Used to move back and forth between the process-models clone
CDUP_TO_PROCESS_MODELS_CLONE_DIR := ../../jbirddog/process-models
CDUP_BACK_TO_THIS_CLONE_DIR := ../../sartography/spiff-element-units

DEV_SERVICE := dev

MY_USER := $(shell id -u)
MY_GROUP := $(shell id -g)
ME := $(MY_USER):$(MY_GROUP)
AS_ME := docker compose run -u $(ME)
DEV_AS_ME := $(AS_ME) $(DEV_SERVICE)

.PHONY: all
all: dev-env

dev-env:
	docker compose build --progress=plain $(DEV_SERVICE)

.PHONY: tests
tests:
	$(DEV_AS_ME) unittest-parallel -vs $(TESTS) -p test_\*.py -t .

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

#
# one off tasks
#

.PHONY: owner-check
owner-check:
	find . ! -user $(MY_USER) ! -group $(MY_GROUP)
