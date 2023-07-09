SHELL := /bin/bash

init-db:
	./scripts/init_db.sh

adminer-up:
	docker-compose -f ops/adminer.yaml up -d adminer  

test:
	cargo test

test-pretty:
	TEST_LOG=true cargo test | bunyan