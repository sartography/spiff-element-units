
TEST_DATA_DIR := tests/data

PROCESS_MODELS_DIR := $(TEST_DATA_DIR)/process-models
SPECS_JSON_DIR := $(TEST_DATA_DIR)/specs-json

# Used to move back and forth between the process-models clone
CDUP_TO_PROCESS_MODELS_CLONE_DIR := ../../jbirddog/process-models
CDUP_BACK_TO_THIS_CLONE_DIR := ../../sartography/spiff-element-units

DEV_SERVICE := dev
MODULE := module

MY_USER := $(shell id -u)
MY_GROUP := $(shell id -g)
ME := $(MY_USER):$(MY_GROUP)

DO := docker compose run $(DEV_SERVICE)
DO_AS_ME := docker compose run -u $(ME) $(DEV_SERVICE)

.PHONY: all
all: dev-env

#
# local development
#

.PHONY: dev-env
dev-env:
	docker compose build --progress=plain $(DEV_SERVICE)

.PHONY: shell
shell:
	$(DO) /bin/bash

.PHONY: compile
compile:
	$(DO) cargo build --color=never # --offline

.PHONY: tests
tests:
	$(DO) cargo test --color=never # --offline

.PHONY: fmt
fmt:
	$(DO) cargo fmt

#
# integration between spiff-element-units and the outside SpiffWorkflow world.
#

.PHONY: copy-process-models
copy-process-models:
	rm -rf $(PROCESS_MODELS_DIR)
	mkdir -p $(PROCESS_MODELS_DIR)
	cd $(CDUP_TO_PROCESS_MODELS_CLONE_DIR) && \
	find . -name "*.bpmn" -exec rsync -R {} $(CDUP_BACK_TO_THIS_CLONE_DIR)/$(PROCESS_MODELS_DIR) \;

,PHONY: script-specs-json
script-specs-json:
	rm -rf $(SPECS_JSON_DIR)
	$(DO_AS_ME) /integration/bin/script_specs_json

.PHONY: bindings
bindings:
	$(DO) /$(MODULE)/bin/make_bindings

.PHONY: run-integration-tests
run-integration-tests:
	$(DO_AS_ME) /integration/bin/run_tests

.PHONY: integration-tests
integration-tests: bindings run-integration-tests

.PHONY: wheel
wheel:
	$(DO) /$(MODULE)/bin/make_wheel

#
# until i figure out if its worth adding cargo everything to run as non root
# in the dev container... then also add back `-u $(ME)` to `DO`.
#

.PHONY: take-ownership
take-ownership:
	sudo chown -R $(ME) .

.PHONY: check-ownership
check-ownership:
	find . ! -user $(MY_USER) ! -group $(MY_GROUP)
