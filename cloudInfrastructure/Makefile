#############
## STANDARD
#############
install:
	pnpm install

upgrade:
	pnpm update
	make sync

sync:
	make format

dev:
	make format

format:
	pnpm rome check . --apply-suggested
	pnpm rome format . --write
	
check:
	pnpm rome check .

test:
	pnpm test

run:
	pnpm --filter "*" start

cleanup:
	rm -rf ~/Library/pnpm/store/v3/files/  
	pnpm store prune 


#############
## INFRA SPECIFIC
#############
# make deploy environment=production 
# e.g make deploy environment=development LINODE_TOKEN=c4ebdc329a825fd7b372e43f1de367dd2277aa39dc00536dfb9d63eb15b8d6c4 pulumi-passphrase=oyelowo skip-preview=false
deploy:
	npx ts-node scripts/deployCloudInfra.ts --environment=$(environment) --LINODE_TOKEN=${LINODE_TOKEN} --pulumi-passphrase=$(pulumi-passphrase) --skip-preview=${skip-preview}
