setup:
	npx ts-node scripts/setup.ts

install:
	npm i

# EXAMPLE USAGE: make generate_manifests_ci environment=local
# environment options: local, development, staging, production
generate_manifests_ci:
	npx ts-node scripts/generateManifestsCI.ts --environment=$(environment)


bootstrap_cluster:
	npx ts-node scripts/bootstrapCluster.ts

generate_all_sealed_secrets:
	npx ts-node scripts/generateSealedSecrets.ts

generate_applications_sealed_secrets:
	npx ts-node scripts/generateApplicationsSealedSecrets.ts

manifests_development_with_sealed_secrets:
# Example usage: make manifests_development_with_sealed_secrets tag=github-sha  
# Change to yes if you dont want the input plain secret typescript file to be cleared after the run
	npx ts-node scripts/bootstrap.ts --e=development --gss=yes --kusi=yes --kuso=no
# npx ts-node scripts/bootstrap.ts --e=development --gss=yes --kusi=yes --kuso=no

manifests_staging_with_sealed_secrets:
# make manifests_staging_with_sealed_secrets tag=github-sha  
	npx ts-node scripts/bootstrap.ts --e=staging --gss=yes --kusi=yes --kuso=no

manifests_production_with_sealed_secrets:
# make manifests_production_with_sealed_secrets tag=github-sha  
	npx ts-node scripts/bootstrap.ts --e=production --gss=yes --kusi=yes --kuso=yes


#####  NO SEALED OR PLAIN SECRETS GENERATED ####
#  This is good for the CI pipeline in github actions where we dont want to store input secret Ts config
# found at kubernetes/secretsManagement/secrets-unsealed/local.ts
manifests_development_no_sealed_secrets:
# Example usage make manifests_development_no_sealed_secrets
	npx ts-node scripts/bootstrap.ts --environment=development --gss=no --kusi=no
# Short version: npx ts-node scripts/bootstrap.ts --e=development

# make manifests_staging_no_sealed_secrets tag=github-sha  
manifests_staging_no_sealed_secrets:
	npx ts-node scripts/bootstrap.ts --environment=staging --gss=no --kusi=no

# make manifests_production_with_sealed_secrets tag=github-sha  
manifests_production_no_sealed_secrets:
	npx ts-node scripts/bootstrap.ts --environment=production --gss=no --kusi=no


dev:
	k3d cluster delete local
	k3d cluster create local --port 8080:80@loadbalancer --port 8443:443@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"
	kubectx k3d-local
		
	make manifests_local_with_sealed_secrets


	# kubectl apply -R -f  ./manifests/local/secrets-encrypted
	skaffold dev --trigger="manual"
	# skaffold dev --trigger="manual" --no-prune=true --no-prune-children=true


merge-kube-config:
# Example Usage:  make merge-kube-config path=config.yaml  
	cp ~/.kube/config ~/.kube/config.bak && KUBECONFIG=~/.kube/config:$(path) kubectl config view --flatten > /tmp/config && mv /tmp/config ~/.kube/config
# cp ~/.kube/config ~/.kube/config.bak && KUBECONFIG=~/.kube/config:/path/to/new/config kubectl config view --flatten > /tmp/config && mv /tmp/config ~/.kube/config