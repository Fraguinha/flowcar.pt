SHELL := /bin/bash


.PHONY: all
all: setup local webapp

.PHONY: setup
setup: _setupTools _createCluster

.PHONY: local
local: _applyManifestsLocal _waitForDatabase

.PHONY: docker
docker: _applyManifestsDocker _waitForDatabase

.PHONY: webapp
webapp: _runWebapp

.PHONY: update
update: _buildDockerImage _rolloutUpdate

.PHONY: clean
clean: _deleteCluster


################################################################################
# Environment variables                                                        #
################################################################################

NAMESPACES = database webapp

POSTGRES_USER ?= flowcar_user
POSTGRES_PASSWORD ?= flowcar_password
POSTGRES_SERVICE ?= postgres-service.database.svc.cluster.local
POSTGRES_PORT ?= 5432
POSTGRES_DB ?= flowcar_db


################################################################################
# Auxiliary targets                                                            #
################################################################################


.PHONY: _setupTools
_setupTools:
	rustup target add wasm32-unknown-unknown
	rustup component add rust-analyzer
	cargo install --locked cargo-leptos


.PHONY: _createCluster
_createCluster:
	minikube start -n 2 --addons 'registry'


.PHONY: _deleteCluster
_deleteCluster:
	minikube delete


.PHONY: _applyManifestsLocal
_applyManifestsLocal:
	kubectl apply -k k8s/overlays/local


.PHONY: _buildDockerImage
_buildDockerImage:
	docker build -t fraguinha/flowcar:latest webapp/
	docker push fraguinha/flowcar:latest


.PHONY: _applyManifestsDocker
_applyManifestsDocker: _buildDockerImage
	kubectl apply -k k8s/overlays/docker


.PHONY: _createSecrets
_createSecrets:
	for namespace in ${NAMESPACES}; do \
		kubectl create secret generic secrets \
		--namespace="$${namespace}" \
		--from-literal=POSTGRES_USER="$(POSTGRES_USER)" \
		--from-literal=POSTGRES_PASSWORD="$(POSTGRES_PASSWORD)" \
		--from-literal=POSTGRES_SERVICE="$(POSTGRES_SERVICE)" \
		--from-literal=POSTGRES_PORT="$(POSTGRES_PORT)" \
		--from-literal=POSTGRES_DB="$(POSTGRES_DB)"; \
	done


.PHONY: _waitForDatabase
_waitForDatabase: _createSecrets
	until kubectl wait --for=condition=ready --timeout=600s --selector=app=postgres --namespace=database pod 2>/dev/null; do sleep 1; done
	until kubectl wait --for=condition=complete --timeout=600s --namespace=database job/postgres-job 2>/dev/null; do sleep 1; done


.PHONY: _runWebapp
_runWebapp:
	POSTGRES_USER=$(POSTGRES_USER) \
	POSTGRES_PASSWORD=$(POSTGRES_PASSWORD) \
	POSTGRES_SERVICE="$(shell minikube ip)" \
	POSTGRES_PORT=$(POSTGRES_PORT) \
	POSTGRES_DB=$(POSTGRES_DB) \
	cargo leptos --manifest-path webapp/Cargo.toml watch


.PHONY: _rolloutUpdate
_rolloutUpdate:
	kubectl rollout restart deployment flowcarployment