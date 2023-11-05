all: help

REPOSITORY=matthausen
APP_NAME=rust_actix_starter_v2
PORT=8080
ENVIRONMENT=${ENV}

# https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help: ## This help
	@awk 'BEGIN {FS = ":.*?## "} /^[a-z0-9A-Z_-]+:.*?## / {printf "\033[36m%-45s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install:
	cargo install --path .

build:
	docker build -t ${REPOSITORY}/${APP_NAME} .

run:
	cargo run

watch:
	cargo watch -x run

test:
	cargo test

docker-run:
	docker run -p ${PORT}:${PORT} ${REPOSITORY}/${APP_NAME}

compose-services:
	DOCKER_BUILDKIT=1 docker-compose -f ./docker-compose.yml up

request:
	curl -X POST -H "Content-Type: application/json" \
    -d '{"name": "linuxize", "email": "linuxize@example.com"}' http://0.0.0.0:${PORT}/api/v1/calculate

docker-push:
	docker push ${REPOSITORY}/${APP_NAME}

kill-all:
	docker kill $(docker container ls -q)

.PHONY: help install build watch run docker-push kill-all