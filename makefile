PROJECT_NAME ?= user-center-rs
NAMESPACE = kbcs
DOCKER_NS = kchain

BUILD_DIR ?= target
BINARY_FILE = app

SERVICE_NAME = $(DOCKER_NS)-$(PROJECT_NAME)
DOCKER_REGISTRY = harbor-k8s.kingdeeresearch.com

DOCKER_RUN_RUST_IMAGE = rust-buildenv:latest

IMAGE_NAME = $(DOCKER_NS)/kbcs-$(PROJECT_NAME)
EXTRA_VERSION ?= $(shell git rev-parse --short HEAD)
IMAGE_FULL_NAME = $(DOCKER_REGISTRY)/$(IMAGE_NAME):$(EXTRA_VERSION)

USERID = $(shell id -u)
DRUN = docker run -i --rm --user=$(USERID):$(USERID) \
	-v $(abspath .):/usr/src/myapp \
	-w /usr/src/myapp

KUBERNETES_FILE = deployment-template.yaml service-template.yaml

CONFIG_TEST="config-test.yaml"
CONFIG_PROD="config-prod.yaml"

define deploy
	for item in $(KUBERNETES_FILE); do \
		cat .ci/.kubernetes/$$item | \
		sed -e 's|__APP_LABEL__|$(SERVICE_NAME)|g' | \
		sed -e 's|__IMAGE_FULL_NAME__|$(IMAGE_FULL_NAME)|g' | \
		sed -e 's|__CONTAINER_NAME__|$(SERVICE_NAME)|g' | \
		sed -e 's|__NAMESPACE__|$(NAMESPACE)|g' | \
		sed -e 's|__DEPLOY_NAME__|$(SERVICE_NAME)|g' | \
		sed -e 's|__SERVICE_NAME__|$(SERVICE_NAME)|g' | \
		sed -e 's|__CONFIG_FILE__|$(1)|g' | \
		kubectl apply --record -f - ; \
	done
endef


help:
	@echo
	@echo "帮助文档："
	@echo "  - make help              查看可用脚本"
	@echo "  - make dep               安装依赖"
	@echo "  - make build             编译可执行文件"
	@echo "  - make docker            编译Docker镜像"
	@echo "  - make deploy-test       部署测试环境"
	@echo "  - make deploy-prod       部署正式环境"
	@echo "  - make clean             清理.build"
	@echo

clean:
	@rm -rf target

prepare:
	@export cargo vendor; cargo fmt; cargo clippy

buildenv: clean
	@docker build -t $(DOCKER_RUN_RUST_IMAGE) image/ -f image/Dockerfile.env

depmac: 
	@brew install libpq; brew reinstall sqlite; brew install mysql-client; brew install mysql-connector-c 

depapt:
	@sudo apt-get install libmysqlclient-dev libsqlite3-dev libpq-dev
	
dep:
	@cargo install diesel_cli

build: buildenv
	$(DRUN) \
	  		$(DOCKER_RUN_RUST_IMAGE) \
       		cargo build --release

docker: build
	@docker build -t $(IMAGE_FULL_NAME) . -f image/Dockerfile.in
	@docker tag $(IMAGE_FULL_NAME) $(DOCKER_REGISTRY)/$(IMAGE_NAME):latest
	@docker push $(IMAGE_FULL_NAME)

deploy-test:
	$(call deploy, $(CONFIG_TEST))

deploy-prod:
	$(call deploy, $(CONFIG_PROD))

.PHONY: dep build deploy-test deploy-prod clean
