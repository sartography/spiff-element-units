
DEV_SERVICE := dev
MODULE := module

MY_USER := $(shell id -u)
MY_GROUP := $(shell id -g)
ME := $(MY_USER):$(MY_GROUP)

DO := docker compose run $(DEV_SERVICE)

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
	$(DO) cargo build --color=never

.PHONY: tests
tests:
	$(DO) cargo test --color=never

.PHONY: fmt
fmt:
	$(DO) cargo fmt

.PHONY: bindings
bindings:
	$(DO) /$(MODULE)/bin/make_bindings

#
# TODO: re-integrate, use module in tests
#

.PHONY: integration-tests
integration-tests:
	$(DEV_AS_ME) unittest-parallel -vs tests -p test_\*.py -t .

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
