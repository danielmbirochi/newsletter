SHELL := /bin/bash

init-db:
	./scripts/init_db.sh

adminer-up:
	docker-compose -f ops/adminer.yaml up -d adminer  