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
	npx ts-node scripts/startAppInLocalCluster.ts

merge-kube-config:
# Example Usage:  make merge-kube-config path=config.yaml  
	cp ~/.kube/config ~/.kube/config.bak && KUBECONFIG=~/.kube/config:$(path) kubectl config view --flatten > /tmp/config && mv /tmp/config ~/.kube/config
# cp ~/.kube/config ~/.kube/config.bak && KUBECONFIG=~/.kube/config:/path/to/new/config kubectl config view --flatten > /tmp/config && mv /tmp/config ~/.kube/config


format:
	npm run prettier