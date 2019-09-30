.PHONY: build ci-code-format ci-code-coverage ci-lint ci-tests docker-build docker-clean docker-push-registry run version

ENVFILE?=.env
ifeq ($(shell test -e $(ENVFILE) && echo -n yes),yes)
	include ${ENVFILE}
	export
endif

DOCKER_REGISTRY?=
DOCKER_IMG_NAME?=mediacloudai/manifest_worker
ifneq ($(DOCKER_REGISTRY), ) 
	DOCKER_IMG_NAME := /${DOCKER_IMG_NAME}
endif
VERSION=$(shell cargo metadata --no-deps --format-version 1 | jq '.packages[0].version' )

build:
	@cargo build

ci-code-format:
	@cargo fmt --all -- --check

ci-code-coverage:
	@cargo tarpaulin

ci-lint:
	@cargo clippy

ci-tests:
	@cargo test

docker-build:
	@docker build -t ${DOCKER_REGISTRY}${DOCKER_IMG_NAME}:${VERSION} .
	@docker tag ${DOCKER_REGISTRY}${DOCKER_IMG_NAME}:${VERSION} ${DOCKER_REGISTRY}${DOCKER_IMG_NAME}:${CI_COMMIT_SHORT_SHA}

docker-clean:
	@docker rmi ${DOCKER_REGISTRY}${DOCKER_IMG_NAME}:${VERSION}
	@docker rmi ${DOCKER_REGISTRY}${DOCKER_IMG_NAME}:${CI_COMMIT_SHORT_SHA}

docker-registry-login:
	@docker login --username "${DOCKER_REGISTRY_LOGIN}" -p"${DOCKER_REGISTRY_PWD}" ${DOCKER_REGISTRY}
	
docker-push-registry:
	@docker push ${DOCKER_REGISTRY}${DOCKER_IMG_NAME}:${VERSION}
	@docker push ${DOCKER_REGISTRY}${DOCKER_IMG_NAME}:${CI_COMMIT_SHORT_SHA}

run:
	@cargo run rs_manifest_worker

version:
	@echo ${VERSION}
