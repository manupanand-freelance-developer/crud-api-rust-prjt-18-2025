#DB_CONTAINER=crud_psql_container

install: 
# cargo new crud-api
# cd crud-api
	cargo add actix-web
	cargo add actix-cors
	cargo add serde_json
	cargo add serde  --features derive
	cargo add chrono --features serde 
	cargo add env_logger
	cargo add dotenv 
	cargo add uuid --features "serde v4"
	cargo add sqlx --features "runtime-async-std-native-tls postgres chrono uuid"
	cargo add jsonwebtoken 
	cargo add argon2 
	cargi add rand_core --features "std"
	# HotReload 
	#cargo install cargo-watch 
	#SQLX-CLI
	cargo install sqlx-cli
build: 
	cargo build 
run: 
	cargo run

#create_docker_container: 
#	docker run --name ${DB_CONTAINER} -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres:12-alpine

# execute database inside container
#create_postgre_db:
#	docker exec -it ${DB_CONTAINER} createdb --username=root --owner=root crudapidb