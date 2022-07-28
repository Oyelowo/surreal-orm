#############
## STANDARD
#############
install:
	yarn install
	make sync

upgrade:
	yarn install
	make sync

sync:
	npx ts-node scripts/syncConfigs.ts
	make format

dev:
	npx ts-node scripts/bootstrapLocalCluster.ts

format:
	yarn prettier
	yarn lint --fix
	
check:
	yarn lint

test:
	# yarn test


#############
## INFRA SPECIFIC
#############
bootsrap_cluster:
	npx ts-node scripts/bootstrapCluster.ts

# EXAMPLE USAGE: make generate_manifests_ci environment=local
# environment options: local, development, staging, production
generate_manifests_ci:
	npx ts-node scripts/generateManifestsCI.ts --environment=$(environment)

generatae_sealed_secrets:
	npx ts-node scripts/generateSealedSecrets.ts

delete_local_clusters:
	npx ts-node scripts/deleteLocalClusters.ts

merge-kube-config:
# Example Usage:  make merge-kube-config path=config.yaml  
	cp ~/.kube/config ~/.kube/config.bak && KUBECONFIG=~/.kube/config:$(path) kubectl config view --flatten > /tmp/config && mv /tmp/config ~/.kube/config
# cp ~/.kube/config ~/.kube/config.bak && KUBECONFIG=~/.kube/config:/path/to/new/config kubectl config view --flatten > /tmp/config && mv /tmp/config ~/.kube/config
	echo "Successfully merged configs. See new config!!!!!! 🎉"
	cat ~/.kube/config
