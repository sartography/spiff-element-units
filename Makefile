DEV_SERVICE := dev

.PHONY: all
all: dev-env

dev-env:
	docker compose build --progress=plain $(DEV_SERVICE)
