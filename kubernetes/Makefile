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
	npx ts-node scripts/script.ts --e=local --t=latest --gss=yes --kusi=yes --kuso=no
# npx ts-node scripts/script.ts --e=local --t=latest --gss=yes

# If you want to keep unsealed secrets output(-kuso) or keep unsealed secrets input(--kiso).
# If you ever use these flags, make sure the files are cleared and sensitive secrets are not left behind
# npx ts-node scripts/script.ts --e=local --t=latest -gss -kuso -kusi
# npx ts-node scripts/script.ts -environment=local --tag=latest --generate-sealed-secrets --keep-unsealed-secrets-output --keep-unsealed-secrets-input

manifests_development_with_sealed_secrets:
# Example usage: make manifests_development_with_sealed_secrets tag=github-sha  
# Change to yes if you dont want the input plain secret typescript file to be cleared after the run
	npx ts-node scripts/script.ts --e=development --t=latest --gss=yes --kusi=no --kuso=no
# npx ts-node scripts/script.ts --e=development --t=$(tag) --gss=yes --kusi=no --kuso=no

manifests_staging_with_sealed_secrets:
# make manifests_staging_with_sealed_secrets tag=github-sha  
	npx ts-node scripts/script.ts --e=staging --t=latest --gss=yes --kusi=no --kuso=no

manifests_production_with_sealed_secrets:
# make manifests_production_with_sealed_secrets tag=github-sha  
	npx ts-node scripts/script.ts --e=production --t=latest --gss=yes --kusi=no --kuso=no


#####  NO SEALED OR PLAIN SECRETS GENERATED ####
#  This is good for the CI pipeline in github actions where we dont want to store input secret Ts config
# found at kubernetes/secretsManagement/secrets-unsealed/local.ts
manifests_development_no_sealed_secrets:
# Example usage make manifests_development_no_sealed_secrets tag=github-sha  
	npx ts-node scripts/script.ts --tag=$(tag) --environment=development --gss=no --kusi=no
# Short version: npx ts-node scripts/script.ts --t=$(tag) --e=development

# make manifests_staging_no_sealed_secrets tag=github-sha  
manifests_staging_no_sealed_secrets:
	npx ts-node scripts/script.ts --tag=$(tag) --environment=staging --gss=no --kusi=no

# make manifests_production_with_sealed_secrets tag=github-sha  
manifests_production_no_sealed_secrets:
	npx ts-node scripts/script.ts --tag=$(tag) --environment=production --gss=no --kusi=no


dev:
	k3d cluster delete local
	k3d cluster create local --port 8080:80@loadbalancer --port 8443:443@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"
	kubectx k3d-local
		
	make manifests_local_with_sealed_secrets


	# kubectl apply -R -f  ./manifests/local/secrets-encrypted
	skaffold dev --trigger="manual" --no-prune=true --no-prune-children=true

