SHELL := /bin/bash

.PHONY: setup local docker update clean \
\
_createCluster _deleteCluster \
_createSecrets \
_buildDockerImage _rolloutUpdate \
_applyManifestsLocal _applyManifestsDocker _applyManifestsProduction \
_deleteManifestsLocal _deleteManifestsDocker _deleteManifestsProduction \
_waitForDatabase \
_runLocal


setup: _createCluster _createSecrets

local: _applyManifestsLocal _waitForDatabase _runLocal

docker: _buildDockerImage _applyManifestsDocker

production: _applyManifestsProduction

update: _buildDockerImage _rolloutUpdate

clean: _deleteManifestsLocal _deleteManifestsDocker _deleteManifestsProduction _deleteCluster


################################################################################
# Environment variables                                                        #
################################################################################


POSTGRES_USER ?= flowcar_user
POSTGRES_PASSWORD ?= flowcar_password
POSTGRES_SERVICE ?= postgres-service
POSTGRES_PORT ?= 5432
POSTGRES_DB ?= flowcar_db


################################################################################
# Auxiliary targets                                                            #
################################################################################


_createCluster:
	-k3d registry create flowcar-registry.localhost --port 5000
	-k3d cluster create flowcar --registry-use k3d-flowcar-registry.localhost:5000


_deleteCluster:
	-k3d registry delete k3d-flowcar-registry.localhost
	-k3d cluster delete flowcar


_createSecrets:
	-kubectl create secret generic secrets \
	--from-literal=POSTGRES_USER="$(POSTGRES_USER)" \
	--from-literal=POSTGRES_PASSWORD="$(POSTGRES_PASSWORD)" \
	--from-literal=POSTGRES_SERVICE="$(POSTGRES_SERVICE)" \
	--from-literal=POSTGRES_PORT="$(POSTGRES_PORT)" \
	--from-literal=POSTGRES_DB="$(POSTGRES_DB)"


_buildDockerImage:
	docker build -t k3d-flowcar-registry.localhost:5000/fraguinha/flowcar-webapp:latest webapp/
	docker push k3d-flowcar-registry.localhost:5000/fraguinha/flowcar-webapp:latest


_rolloutUpdate:
	kubectl rollout restart deployment flowcar-deployment


_applyManifestsLocal:
	kubectl apply -k k8s/overlays/local


_applyManifestsDocker:
	kubectl apply -k k8s/overlays/docker


_applyManifestsProduction:
	kubectl apply -k k8s/overlays/production


_waitForDatabase:
	until kubectl wait --for=condition=ready --timeout=600s --selector=app=postgres pod 2>/dev/null; do sleep 1; done
	until kubectl wait --for=condition=complete --timeout=600s job/postgres-job 2>/dev/null; do sleep 1; done


_deleteManifestsLocal:
	-kubectl delete -k k8s/overlays/local


_deleteManifestsDocker:
	-kubectl delete -k k8s/overlays/docker


_deleteManifestsProduction:
	-kubectl delete -k k8s/overlays/production


_runLocal:
	POSTGRES_USER=$(POSTGRES_USER) \
	POSTGRES_PASSWORD=$(POSTGRES_PASSWORD) \
	POSTGRES_SERVICE=$(shell kubectl get services postgres-service -o jsonpath='{.status.loadBalancer.ingress[0].ip}') \
	POSTGRES_PORT=$(shell kubectl get services postgres-service -o jsonpath='{.spec.ports[0].port}') \
	POSTGRES_DB=$(POSTGRES_DB) \
	cargo leptos --manifest-path webapp/Cargo.toml watch
