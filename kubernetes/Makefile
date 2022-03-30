install:
	npm i

# Used as base function for other generator functions for various environments
manifests_generator:
	make install
	rm -rf ./login
	mkdir ./login
	pulumi login file://login 
	export PULUMI_CONFIG_PASSPHRASE="" && pulumi stack init --stack dev
	export PULUMI_CONFIG_PASSPHRASE="" && \
	export IMAGE_TAG_REACT_WEB="$(tag)" && \
	export IMAGE_TAG_GRAPHQL_MONGO="$(tag)" && \
	export IMAGE_TAG_GRPC_MONGO="$(tag)" && \
	export IMAGE_TAG_GRAPHQL_POSTGRES="$(tag)" && \
	export ENVIRONMENT="$(environment)" && \
	pulumi update --yes --skip-preview --stack dev


# make manifests_local tag=local
manifests_local:
	make manifests_generator tag=local environment=local

# make manifests_development tag=github-sha  
manifests_development:
	make manifests_generator tag=$(tag) environment=development

# make manifests_staging tag=github-sha  
manifests_staging:
	make manifests_generator tag=$(tag) environment=staging

# make manifests_production tag=github-sha  
manifests_production:
	make manifests_generator tag=$(tag) environment=production

# Example Usage:  make manifests_temporary tag=temporary
manifests_temporary:
	export TEMPORARY_DIR="temporary" && \
	make manifests_generator tag=$(tag) environment=local




clean:
    @ifeq[ "test" = "test" ]; then\
        echo "Hello world";\
    fi
