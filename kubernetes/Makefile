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

start-cluster:
    # if you wan to disable traefik as the default ingress controller for the cluster
	k3d cluster create local --k3s-arg "--no-deploy=traefik@server:*"
	# k3d cluster create local

delete-cluster:
	k3d cluster delete local

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