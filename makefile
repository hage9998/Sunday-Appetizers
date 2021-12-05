# ENV ?= dev
# Include configurable flags.
# include envs/$(ENV).env
-include envs/local.env

# Database host.
PGHOST ?= $(firstword $(DB_HOST) localhost)
# Database name.
PGDATABASE ?= $(firstword $(DB_NAME) postgres)
# Database password.
PGPASSWORD ?= $(firstword $(DB_PASS) postgres)
# Database pool size
PGPOOLSIZE ?= $(firstword $(DB_POOL_SIZE) $(shell nproc))
# Database user
PGUSER ?= $(firstword $(DB_USER) postgres)

DATABASE_URL ?= postgres://$(PGUSER):$(PGPASSWORD)@/$(PGDATABASE)?host=$(PGHOST)

test:
	@cargo test

init:
	@$(if $(shell which diesel),,cargo install diesel_cli --no-default-features --features postgres 2> /dev/null)
	@sudo service postgresql restart
	@cd migrations > /dev/null; diesel database reset; cd ../..

# Run all the necessary database migrations.
migrate: 
	@cd migrations > /dev/null; diesel migration run; cd ../..

# Revert the last migration.
revert:
	@cd migrations > /dev/null; diesel migration revert; cd ../..

# Create a new migration based on the `NAME` variable.
migration:
	@cd migrations; diesel migration generate $(NAME); cd ../..
