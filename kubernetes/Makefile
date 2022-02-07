gen:
	export ENVIRONMENT="development" && pulumi update --yes --skip-preview
	# export ENVIRONMENT="staging" && pulumi update --yes --skip-preview
	# export ENVIRONMENT="production" && pulumi update --yes --skip-preview
	
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