[![Rust monorepo CICD](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/rust.yaml/badge.svg?branch=master)](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/rust.yaml)

[![Build typescript images](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/typescript.yaml/badge.svg)](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/typescript.yaml)          [![kubernetes](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/kubernetes.yaml/badge.svg)](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/kubernetes.yaml)         [![Generate Kubernetes manifests](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/generate-kubernetes-manifests.yaml/badge.svg)](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/generate-kubernetes-manifests.yaml)               [![cleanup old images](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/delete-old-images.yaml/badge.svg)](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/delete-old-images.yaml)



## Convention
To carry out certain tasks in any directory, these are the standard commands:


| Commands   |      Purpose      
|----------|:-------------
| `make setup`    | To setup the codebase for development| 
| `make install`  | install packages   |   
| `make upgrade`  | upgrade packages |    
| `make sync`     | synchronize/generate local code e.g graphql queries, kubernetes configs etc |    
| `make dev`      | start cluster/app locally in live reloading mode |    
| `make format`   | format code |   
| `make check`    | check that code aligns with standard |    
| `make test`     | run automated tests |    

