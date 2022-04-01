setup:
	npx ts-node scripts/setup.ts

install:
	npm i

##### NOTE: Commands related to generating sealed secrets requires that there is a running
# cluster. The script will generate a bitnami sealed secret controller
# and apply that automatically to the cluster if it does not yet exist
# Make sure your branch is up to date with master before this

# Generates sealed secrets from output plain unsealed kubernetes secretes which
# is also generated from the config created by you, the user at e.g: kubernetes/secretsManagement/secrets-unsealed/production.ts
manifests_local_with_sealed_secrets:
	npx ts-node scripts/script.ts -e=local -t=latest --gss
# npx ts-node scripts/script.ts -e=local -t=latest --gss

# If you want to keep unsealed secrets output(--kuso) or keep unsealed secrets input(--kiso).
# If you ever use these flags, make sure the files are cleared and sensitive secrets are not left behind
# npx ts-node scripts/script.ts -e=local -t=latest --gss --kuso --kiso
# npx ts-node scripts/script.ts -environment=local --tag=latest --generate-sealed-secrets --delete

manifests_development_with_sealed_secrets:
# Example usage: make manifests_development_with_sealed_secrets tag=github-sha  
	npx ts-node scripts/script.ts -e=development -t=latest --gss
	# npx ts-node scripts/script.ts -e=development -t=$(tag) --gss

manifests_staging_with_sealed_secrets:
# make manifests_staging_with_sealed_secrets tag=github-sha  
	npx ts-node scripts/script.ts -e=staging -t=latest --gss

manifests_production_with_sealed_secrets:
# make manifests_production_with_sealed_secrets tag=github-sha  
	npx ts-node scripts/script.ts -e=production -t=latest --gss


#####  NO SEALED OR PLAIN SECRETS GENERATED ####
#  This is good for the CI pipeline in github actions where we dont want to store input secret Ts config
# found at kubernetes/secretsManagement/secrets-unsealed/local.ts
manifests_development_no_sealed_secrets:
# Example usage make manifests_development_no_sealed_secrets tag=github-sha  
	npx ts-node scripts/script.ts tag=$(tag) environment=development

# make manifests_staging_no_sealed_secrets tag=github-sha  
manifests_staging_no_sealed_secrets:
	npx ts-node scripts/script.ts tag=$(tag) environment=staging

# make manifests_production_with_sealed_secrets tag=github-sha  
manifests_production_no_sealed_secrets:
	npx ts-node scripts/script.ts tag=$(tag) environment=production


dev:
	k3d cluster delete local
	k3d cluster create local --port 8080:80@loadbalancer --port 8443:443@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"
	kubectx k3d-local
		
	make manifests_local_with_sealed_secrets


	# kubectl apply -R -f  ./manifests/local/secrets-encrypted
	skaffold dev --trigger="manual" --no-prune=true --no-prune-children=true

