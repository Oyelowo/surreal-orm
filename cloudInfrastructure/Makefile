#############
## STANDARD
#############
install:
	npm install

upgrade:
	npm update
	make sync

sync:
	# npx ts-node scripts/syncConfigs.ts
	make format

dev:
	# npx ts-node scripts/bootstrapLocalCluster.ts

format:
	npm run prettier-fix
	npm run lint-fix
	
check:
	npm run prettier-check
	npm run lint-check

test:
	npm test



#############
## INFRA SPECIFIC
#############
# make deploy environment=production 
# e.g make deploy environment=development LINODE_TOKEN=c4ebdc329a825fd7b372e43f1de367dd2277aa39dc00536dfb9d63eb15b8d6c4 pulumi-passphrase=oyelowo skip-preview=false
deploy:
	npx ts-node scripts/deployCloudInfra.ts --environment=$(environment) --LINODE_TOKEN=${LINODE_TOKEN} --pulumi-passphrase=$(pulumi-passphrase) --skip-preview=${skip-preview}
