
install:
	npm i

gen:
	make install
	rm -rf ./login
	mkdir ./login
	rm -rf ./rendered
	pulumi login file://login 
	export PULUMI_CONFIG_PASSPHRASE="" && pulumi stack init --stack dev
	# pulumi destroy --stack dev  --yes --skip-preview
	export ENVIRONMENT="development" && export PULUMI_CONFIG_PASSPHRASE="" && pulumi update --yes --skip-preview --stack dev

generate_local:
	make install
	rm -rf ./login
	mkdir ./login
	# rm -rf ./manifests/local
	pulumi login file://login 
	export PULUMI_CONFIG_PASSPHRASE="" && pulumi stack init --stack dev
	# pulumi destroy --stack dev  --yes --skip-preview
	export IMAGE_TAG_REACT_WEB="local" && \
	export IMAGE_TAG_GRAPHQL_MONGO="local" && \
	export IMAGE_TAG_GRPC_MONGO="local" && \
	export IMAGE_TAG_GRAPHQL_POSTGRES="local" && \
	export ENVIRONMENT="local" && \
	export PULUMI_CONFIG_PASSPHRASE="" && \
	pulumi update --yes --skip-preview --stack dev

generate_production:
	make install
	rm -rf ./login
	mkdir ./login
	pulumi login file://login 
	export PULUMI_CONFIG_PASSPHRASE="" && pulumi stack init --stack dev
	export IMAGE_TAG_REACT_WEB="local" && \
	export IMAGE_TAG_GRAPHQL_MONGO="local" && \
	export IMAGE_TAG_GRPC_MONGO="local" && \
	export IMAGE_TAG_GRAPHQL_POSTGRES="local" && \
	export ENVIRONMENT="production" && \
	export PULUMI_CONFIG_PASSPHRASE="" && \
	pulumi update --yes --skip-preview --stack dev

generate:
	make install
	rm -rf ./login
	mkdir ./login
	rm -rf ./rendered
	pulumi login file://login 
	export PULUMI_CONFIG_PASSPHRASE="" && pulumi stack init --stack dev
	# pulumi destroy --stack dev  --yes --skip-preview
	export ENVIRONMENT="development" && export PULUMI_CONFIG_PASSPHRASE="" && pulumi update --yes --skip-preview --stack dev

start-web:
	# (cd ../typescript && make dev)

start-k8s-apps:
	make gen
	k3d cluster delete local
	k3d cluster create local --port 8080:80@loadbalancer --port 8443:443@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"
	skaffold dev --trigger="manual" --no-prune=true --no-prune-children=true
	# skaffold dev --trigger="manual"
	# skaffold dev
	
# start-app-build:
dev:
	sh -c 'make start-web & make start-k8s-apps & wait'


# BASEDIR = ./rendered/applications/1-manifest
# MYDIR = ${BASEDIR}
# all:
#     @for f in $(shell ls ${MYDIR}); do echo $${f}; done

# NOTE
####  ##*/ gets the file name out of the path;

generate_temporary:
	make install
	rm -rf ./login
	mkdir ./login
	pulumi login file://login 
	export PULUMI_CONFIG_PASSPHRASE="" && pulumi stack init --stack dev
	export IMAGE_TAG_REACT_WEB="local" && \
	export IMAGE_TAG_GRAPHQL_MONGO="local" && \
	export IMAGE_TAG_GRPC_MONGO="local" && \
	export IMAGE_TAG_GRAPHQL_POSTGRES="local" && \
	export ENVIRONMENT="production" && \
	export TEMPORARY_DIR="generated-temporary" && \
	export PULUMI_CONFIG_PASSPHRASE="" && \
	pulumi update --yes --skip-preview --stack dev

BASEDIR=./manifests/${ENVIRONMENT}
GENERATED_DIR=${BASEDIR}/generated-temporary
SECRETS_ENCRYPTED_DIR=${BASEDIR}/sealed-secrets
seal:
	# Example usage: ENVIRONMENT="production" make seal
	
	# Apply namespace first
	kubectl apply -R -f ${GENERATED_DIR}/namespaces

	# Apply setups with sealed secret controller
	kubectl apply -R -f  ${GENERATED_DIR}/cluster-setup  

	# Wait for bitnami sealed secrets controller to be in running phase so that we can use it to encrypt secrets
	kubectl rollout status deployment/sealed-secrets-controller -n=kube-system

	mkdir -p ${SECRETS_ENCRYPTED_DIR}

	@for FILE in ${GENERATED_DIR}/**/**/secret-*ml; do \
		kubeseal <$${FILE} -o yaml >${SECRETS_ENCRYPTED_DIR}/$${FILE##*/}; \
		echo "REMOVING unsealed secret $${FILE}"; \
		rm -rf $${FILE}; \
	done

	rm -rf ${GENERATED_DIR}
# rm -rf ./shared/secrets-dont-push.json

setup:
	cp -R secrets.environment.example secrets.production.json
	# Then edit the file

# setup-cluster-production:	
create-sealed-secrets:	
	make generate_temporary
	ENVIRONMENT=production make seal

	# kubectl apply -R -f  ./manifests/local/secrets-encrypted



# setup-cluster-production:	
create-sealed-secrets:	
	make generate_temporary
	ENVIRONMENT=production make seal

	# kubectl apply -R -f  ./manifests/local/secrets-encrypted

# Does the generation and encryption
rotate-secret-local:
	# Example  usage: make setup ENVIRONMENT=production
	# k3d cluster delete local
	# k3d cluster create local --port 8080:80@loadbalancer --port 8443:443@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"
	
	make generate_production
	ENVIRONMENT=$(ENVIRONMENT) make seal

start-dev:
	k3d cluster delete local
	k3d cluster create local --port 8080:80@loadbalancer --port 8443:443@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"
	
	make generate_local
	# ENVIRONMENT=$(ENVIRONMENT) make seal
	ENVIRONMENT=local make seal

	kubectl apply -R -f  ./manifests/local/secrets-encrypted
	skaffold dev --trigger="manual" --no-prune=true --no-prune-children=true



# TODO: Parameterise environment 
seal_production:
	export ENVIRONMENT=production && \
	k3d cluster delete local
	k3d cluster create local --port 8080:80@loadbalancer --port 8443:443@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"
	make generate_production
	make rollout

	@for FILE in ${GENERATED_DIR}/**/**/secret-*ml; do \
		kubeseal <$${FILE} -o yaml >${SECRETS_ENCRYPTED_DIR}/$${FILE##*/}; \
		echo "REMOVING unsealed secret $${FILE}"; \
		rm -rf $${FILE}; \
	done

# @for FILE in ${BASEDIR}/argocd/1-manifest/secret-*ml; do \
# 	echo "sealing secret: $${FILE} into ./outputs directory"; \
# 	kubeseal <$${FILE} -o yaml >${BASEDIR}/secrets-encrypted/$${FILE##*/} --namespace applications; \
# 	\
# 	echo "REMOVING unsealed secret $${FILE}"; \
# 	rm -rf $${FILE}; \
# done


kk:
	@for FILE in ${BASEDIR}/**/**/secret-*ml; do \
		echo "sealing secret: $${FILE} into ./outputs directory"; \
	done

# kubeseal --scope cluster-wide <$${FILE} >./outputs/$${FILE##*/} --namespace applications; \
# Merge into an existing secret
# kubeseal --merge-into mysealedsecret.json


# seal:
# 	@for FILE in ./rendered/applications/1-manifest/secret-*ml; do \
# 		echo "sealing secret: $${FILE} into ./outputs directory"; \
# 		kubeseal <$${FILE} -o yaml > ./outputs/$${FILE##*/} --namespace applications; \
# 		echo "removing unsealed secret $${FILE}"; \
# 		rm -rf $${FILE};
# 	done
# 		# kubeseal --scope cluster-wide <$${FILE} >./outputs/$${FILE##*/} --namespace applications; \
# 		# Merge into an existing secret
# 		# kubeseal --merge-into mysealedsecret.json