SRC := spiff_element_units
TESTS := tests

DEV_SERVICE := dev

MY_USER := $(shell id -u)
MY_GROUP := $(shell id -g)
ME := $(MY_USER):$(MY_GROUP)
AS_ME := docker compose run -u $(ME) $(DEV_SERVICE)

.PHONY: all
all: dev-env

dev-env:
	docker compose build --progress=plain $(DEV_SERVICE)

.PHONY: tests
tests:
	$(AS_ME) unittest-parallel -vs $(TESTS) -p test_*.py -t .
