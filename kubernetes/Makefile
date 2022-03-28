
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
	rm -rf ./rendered
	pulumi login file://login 
	export PULUMI_CONFIG_PASSPHRASE="" && pulumi stack init --stack dev
	# pulumi destroy --stack dev  --yes --skip-preview
	export IMAGE_TAG_REACT_WEB="local" && \
	export IMAGE_TAG_GRAPHQL_MONGO="local" && \
	export IMAGE_TAG_GRPC_MONGO="local" && \
	export IMAGE_TAG_GRAPHQL_POSTGRES="local" && \
	export ENVIRONMENT="development" && \
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

rollout:
	# Apply namespace first
	kubectl apply -R -f ./rendered/namespaces

	# Apply setups with sealed secret controller
	kubectl apply -R -f  ./rendered/cluster-setup  

	# Wait for bitnami sealed secrets controller to be in running phase so that we can use it to encrypt secrets
	kubectl rollout status deployment/sealed-secrets-controller -n=kube-system


# BASEDIR = ./rendered/applications/1-manifest
# MYDIR = ${BASEDIR}
# all:
#     @for f in $(shell ls ${MYDIR}); do echo $${f}; done

# NOTE
####  ##*/ gets the file name out of the path;
seal:
	make generate_local
	k3d cluster delete local
	k3d cluster create local --port 8080:80@loadbalancer --port 8443:443@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"
	make rollout

	@for FILE in ./rendered/applications/1-manifest/secret-*ml; do \
		echo "sealing secret: $${FILE} into ./outputs directory"; \
		kubeseal <$${FILE} -o yaml >./outputs/$${FILE##*/} --namespace applications; \
		\
		echo "REMOVING unsealed secret $${FILE}"; \
		rm -rf $${FILE}; \
	done
	rm -rf ./shared/secrets-dont-push.json
	
	@for FILE in ./rendered/argocd/1-manifest/secret-*ml; do \
		echo "sealing secret: $${FILE} into ./outputs directory"; \
		kubeseal <$${FILE} -o yaml >./outputs/$${FILE##*/} --namespace applications; \
		\
		echo "REMOVING unsealed secret $${FILE}"; \
		rm -rf $${FILE}; \
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