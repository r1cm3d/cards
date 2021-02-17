.PHONY: clean build test

all: test

destroy:
	@echo "\nRemoving localstack container\n"
	@@docker rm -f dynamodb 2>&1 1>/dev/null | true
	@rm -rf terraform/*tfstate* terraform/.terraform 2>&1 1>/dev/null | true

init: destroy up
	@echo "\nWaiting until localstack be ready"
	@until docker inspect --format='{{json .State.Health}}' dynamodb | grep -o healthy; do sleep 1; done
	@echo "\nCleaning AWS resources"
	-@cd scripts && bash delete-tables.sh
	@echo "\nApplying terraform scripts"
	-cd terraform/ && \
	terraform init && \
	terraform destroy  -auto-approve && \
	terraform plan && \
	terraform apply -auto-approve

lint:
	@echo "\nApplying Clippy\n"
	@cargo clippy

fmt:
	@echo "\nFormatting scripts"
	@shfmt -w scripts/*sh
	@echo "\nFormatting terraform files"
	@terraform fmt terraform/
	@echo "\nFormatting Rust files\n"
	@cargo fmt

build: fmt lint
	@echo "\nBuilding application\n"
	@cargo build

up:
	@echo "\nStarting localstack container and creating AWS local resources"
	@docker-compose up -d --force-recreate

test: fmt up
	@echo "\nRunning tests\n"
	@cargo test

run: up
	@echo "\nRunning locally"
	@cargo run

down:
	@echo "\nStopping containers"
	@docker-compose stop