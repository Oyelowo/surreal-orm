#!/bin/sh

docker build . -f Dockerfile.prod --target graphql-mongo  -t oyelowo/graphql-mongo-prod

docker-slim build --include-path=/app oyelowo/graphql-mongo-prod