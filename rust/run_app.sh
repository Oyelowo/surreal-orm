#!/bin/sh

docker build . -f Dockerfile.prod --target backend-main  -t oyelowo/backend-main-prod

docker-slim build --include-path=/app oyelowo/backend-main-prod