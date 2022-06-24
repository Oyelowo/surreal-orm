setup:
	make sync
	# npx ts-node scripts/syncConfigs.ts

install:
	npm i

sync:
	npx ts-node scripts/syncConfigs.ts
	make generate_manifests_ci environment=local 
	make format

# EXAMPLE USAGE: make generate_manifests_ci environment=local
# environment options: local, development, staging, production
generate_manifests_ci:
	# make sync
	npx ts-node scripts/generateManifestsCI.ts --environment=$(environment)


bootstrap_cluster:
	npx ts-node scripts/bootstrapCluster.ts

generate_sealed_secrets:
	npx ts-node scripts/generateSealedSecrets.ts

dev:
	k3d cluster delete local
	k3d cluster create local --port 8080:80@loadbalancer --port 8443:443@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"
	kubectx k3d-local
		
	make generate_manifests_ci environment=local


	# kubectl apply -R -f  ./manifests/local/secrets-encrypted
	skaffold dev --cleanup=false
	# skaffold run --trigger="manual" --no-prune=true --no-prune-children=true


merge-kube-config:
# Example Usage:  make merge-kube-config path=config.yaml  
	cp ~/.kube/config ~/.kube/config.bak && KUBECONFIG=~/.kube/config:$(path) kubectl config view --flatten > /tmp/config && mv /tmp/config ~/.kube/config
# cp ~/.kube/config ~/.kube/config.bak && KUBECONFIG=~/.kube/config:/path/to/new/config kubectl config view --flatten > /tmp/config && mv /tmp/config ~/.kube/config


format:
	npm run prettier