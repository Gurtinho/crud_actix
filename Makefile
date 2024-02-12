# variáveis
DOC = docker compose

run:
	@cargo watch -q -c -x run

up:
	@$(DOC) up -d
	
down:
	@$(DOC) down

start:
	@$(DOC) start

create:
	@sqlx migrate add -r --source src/http/databases/migrations $(NAME)

migrate:
	@sqlx migrate run --source src/http/databases/migrations

revert:
	@sqlx migrate revert --source src/http/databases/migrations

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
	# HotReload
	@cargo install cargo-watch
	# SQLX-CLI
	@cargo install sqlx-cli