
DEV_SERVICE := dev
MODULE := module

MY_USER := $(shell id -u)
MY_GROUP := $(shell id -g)
ME := $(MY_USER):$(MY_GROUP)

DO := docker compose run $(DEV_SERVICE)
DO_AS_ME := docker compose run -u $(ME) $(DEV_SERVICE)

.PHONY: all
all: dev-env

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

.PHONY: bindings
bindings:
	$(DO) /$(MODULE)/bin/make_bindings

#
# TODO: re-integrate, use module in tests
#

.PHONY: run-integration-tests
run-integration-tests:
	$(DO_AS_ME) /integration/tests/bin/run_tests

.PHONY: integration-tests
integration-tests: bindings run-integration-tests

#
# until i figure out if its worth adding cargo everything to run as non root
# in the dev container... then also add back `-u $(ME)` to `DO`.
#

.PHONY: take-ownership
take-ownership:
	sudo chown -R $(ME) $(MODULE)

.PHONY: check-ownership
check-ownership:
	find . ! -user $(MY_USER) ! -group $(MY_GROUP)
