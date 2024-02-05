# variáveis
DD = cargo add

run:
	@cargo watch -q -c -x run

up:
	@docker-compose up -d
	
down:
	@docker-compose down

install:
	@$(DD) actix-web
	@$(DD) actix-cors
	@$(DD) serde_json
	@$(DD) serde --features derive
	@$(DD) chrono --features serde
	@$(DD) env_logger
	@$(DD) dotenv
	@$(DD) uuid --features "serde v4"
	@$(DD) sqlx --features "runtime-async-std-native-tls postgres chrono uuid"
	# HotReload
	@cargo install cargo-watch
	# SQLX-CLI
	@cargo install sqlx-cli