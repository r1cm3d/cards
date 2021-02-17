.PHONY: clean build test

all: test

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

clean:
	@echo "\nRemoving localstack container\n"
	@@docker rm -f dynamodb 2>&1 1>/dev/null | true
	@rm -rf terraform/*tfstate* terraform/.terraform 2>&1 1>/dev/null | true

build: clean fmt lint
	@echo "\nBuilding application\n"
	@cargo build

up: clean
	@echo "\nStarting localstack container and creating AWS local resources"
	@docker-compose up -d --force-recreate
	@echo "\nWaiting until localstack be ready"
	@until docker inspect --format='{{json .State.Health}}' aws | grep -o healthy; do sleep 1; done
	@echo "\nApplying terraform scripts"
	-cd terraform/ && \
	terraform init && \
	terraform destroy  -auto-approve && \
	terraform plan && \
	terraform apply -auto-approve

test: fmt up
	@echo "\nRunning tests\n"
	@cargo test

codecov: up
	@echo "\nRunning Codecov\n"

run: up
	@echo "\nRunning locally"
	@cargo run

stop:
	@echo "\nStopping containers"
	@docker-compose stop