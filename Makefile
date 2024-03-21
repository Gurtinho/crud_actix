# variáveis
DOC = docker compose
URL = postgres://postgres:crud_actix@database:5432/crud_actix?schema=public

run:
	@cargo watch -q -c -x run

app:
	@$(DOC) exec app bash

up:
	@$(DOC) up -d
	
down:
	@$(DOC) down

start:
	@$(DOC) start

stop:
	@$(DOC) stop

reload:
	@docker compose up -d --force-recreate

create:
	# use make create NAME=users
	@sqlx migrate add -r --source src/http/databases/migrations $(NAME)

migrate:
	@sqlx migrate run --source src/http/databases/migrations

revert:
	@sqlx migrate revert --source src/http/databases/migrations

migrate-docker:
	@sqlx migrate run --database-url="$(URL)" --source src/http/databases/migrations

revert-docker:
	@sqlx migrate revert --database-url="$(URL)" --source src/http/databases/migrations

build:
	@docker compose up -d
	@docker build -t crud_actix:latest -f Dockerfile .
	@docker run --rm -p 8081:8081 crud_actix:latest

add:
	@cargo add actix-web
	@cargo add actix-cors
	@cargo add serde_json
	@cargo add serde --features derive
	@cargo add chrono --features serde
	@cargo add env_logger
	@cargo add dotenv
	@cargo add uuid --features "serde v4"
	@cargo add sqlx --features "runtime-async-std-native-tls postgres chrono uuid"
	@cargo add bcrypt
	@cargo add swagger
	@cargo add validator
	# HotReload
	@cargo install cargo-watch
	# SQLX-CLI
	@cargo install sqlx-cli