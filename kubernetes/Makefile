install:
	npm i

manifests_local:
	npx ts-node scripts/script.ts -e=local -t=latest --gss --kuso --kiso
	# npx ts-node scripts/script.ts -environment=local --tag=latest --generate-sealed-secrets --delete

# make manifests_development tag=github-sha  
manifests_development:
	make manifests_generator tag=$(tag) environment=development

# make manifests_staging tag=github-sha  
manifests_staging:
	make manifests_generator tag=$(tag) environment=staging

# make manifests_production tag=github-sha  
manifests_production:
	make manifests_generator tag=$(tag) environment=production

# Example Usage:  make manifests_temporary tag=temporary
manifests_temporary:
	export TEMPORARY_DIR="temporary" && \
	make manifests_generator tag=$(tag) environment=local



clean:
	@if [ "test" = "test" ]; then\
		echo "Hello world";\
	fi

# Example usage: ENVIRONMENT="production" make seal
BASEDIR=./manifests/generated/$(environement)
# GENERATED_DIR=${BASEDIR}/temporary
# GENERATED_DIR=./manifests/generated/$(environement)
SECRETS_ENCRYPTED_DIR=${BASEDIR}/sealed-secrets
sealed_secrets:
	kubectl apply -R -f ${GENERATED_DIR}/namespaces
	kubectl apply -R -f  ${GENERATED_DIR}/cluster-setup  # Apply setups with sealed secret controller
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
create-sealed-secrets2:	
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