gen:
	export PULUMI_CONFIG_PASSPHRASE="" 
	rm -rf ./login
	mkdir ./login
	rm -rf ./rendered
	pulumi login file://login 
	pulumi stack init --stack dev
	# pulumi destroy --stack dev  --yes --skip-preview
	export ENVIRONMENT="development" && pulumi update --yes --skip-preview --stack dev

	# export ENVIRONMENT="staging" && pulumi update --yes --skip-preview
	# export ENVIRONMENT="production" && pulumi update --yes --skip-preview

	# kubectl apply -f rendered/development/1-manifest/namespace-*
	# kubectl apply -f rendered/development/1-manifest/

start-web:
	(cd ../typescript && make dev)

start-k8s-apps:
	make gen
	k3d cluster delete local
	k3d cluster create local --port 8080:80@loadbalancer --port 8443:443@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"
	skaffold dev --trigger="manual"
	# skaffold dev
	
start-app-build:
	sh -c 'make start-web & make start-k8s-apps & wait'

start-app:
	skaffold dev --trigger="manual"
	# skaffold dev

start-cluster:
    # if you wan to disable traefik as the default ingress controller for the cluster
	 k3d cluster create local --port 8080:80@loadbalancer --port 8443:443@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"
	# k3d cluster create local --k3s-arg "--no-deploy=traefik@server:*"
	# k3d cluster create local --k3s-arg '--flannel-backend=none@server:*' '--disable-network-policy'
	# k3d cluster create local --k3s-arg '--flannel-backend=none@server:*' '--no-deploy=network-policy@server:*'
	# k3d cluster create local --api-port 6550 -p "8081:80@loadbalancer" --agents 2 --k3s-arg "--no-deploy=traefik@server:*"
	# k3d cluster create local
	# k3d cluster create localcluster \
	# 	--api-port 127.0.0.1:6443 \
	# 	-p 80:80@loadbalancer \
	# 	-p 443:443@loadbalancer \
	# 	--k3s-server-arg "--no-deploy=traefik"

	#  k3d cluster create local --api-port 127.0.0.1:6443 -p 8080:80@loadbalancer -p 443:443@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"
	# k3d cluster create local --api-port 6550 -p 80:80@loadbalancer -p 443:443@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"
	# k3d cluster create local --api-port 6550 --k3s-arg "--no-deploy=traefik@server:*" --k3s-arg "--disable=servicelb@server:*" --no-lb --wait
	# k3d cluster create local --api-port 6550  -p 8080:80@agent:* --k3s-arg "--no-deploy=traefik@server:*" --k3s-arg "--disable=servicelb@server:*" --no-lb --wait
	# k3d cluster create dev --port 8080:80@loadbalancer --port 8443:443@loadbalancer
	# k3d cluster create local --port 8080:80@loadbalancer --port 8443:443@loadbalancer 
# kubectl port-forward $(kubectl get pods --selector "app.kubernetes.io/name=traefik" --output=name) 9000:9000

# k3d cluster create nginx --k3s-server-arg '--no-deploy=traefik' --volume "$(pwd)/helm-ingress-nginx.yaml:/var/lib/rancher/k3s/server/manifests/helm-ingress-nginx.yaml"
# k3d cluster create local --k3s-arg '--no-deploy=traefik@server:*' --volume "$(pwd)/rendered/development/1-manifest/ingress-development-nginx-ingress.yaml:/var/lib/rancher/k3s/server/manifests/ingress-development-nginx-ingress.yaml"

# with 3 master(servers) and 3 worker(agent) nodes
# k3d cluster create test --port 8080:80@loadbalancer --port 8443:443@loadbalancer --api-port 6443 --servers 3 --agents 3

# k3d cluster create k3s-demo-cluster --api-port 6550 --agents 1 --k3s-arg "--disable=traefik@server:0" --k3s-arg "--disable=servicelb@server:0" --no-lb --wait
	skaffold dev

delete-cluster:
	k3d cluster delete local

start:
	skaffold dev

dev-manual:
	skaffold dev --build-concurrency=0 --trigger="manual"

dev:
	pulumi update --yes --skip-preview

devv:
	pulumi login --local
	pulumi stack rm --stack dev --force --yes
	pulumi stack init --stack dev
	pulumi update --logtostderr --stack dev --yes
	pulumi stack rm --stack dev --force --yes

# build:
# 	yarn run tsc

# publish: build
# 	yarn version --prerelease
# 	cp README.md LICENSE package.json ./bin/
# 	yarn publish --non-interactive --access public ./bin/

# test:
#   #!/usr/bin/env sh
#   export PULUMI_CONFIG_PASSPHRASE=abc

#   pulumi login --local
#   pulumi plugin install resource kubernetes 2.7.0

#   for testDir in ./tests/*; do
#     if [ -d "${testDir}" ]; then
#       pulumi --cwd ${testDir} stack init --stack test
#       pulumi --cwd ${testDir} update --logtostderr --stack test --yes --cwd ${testDir}
#       pulumi --cwd ${testDir} stack rm --stack test --force --yes
#       conftest test --policy ${testDir}/test.rego ${testDir}/rendered-yaml
#     fi
#   done

# # kubernetes-manifests:


# # 	pulumi update --yes