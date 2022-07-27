[![Rust monorepo CICD](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/rust.yml/badge.svg)](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/rust.yml)

[![Typescript monorepo applications CICD](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/typescript.yml/badge.svg)](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/typescript.yml)
# modern-distributed-app-template
Template for highly scalable distributed web application with Rustlang as the server language, and React/Typescript/NextJS as the frontend, Kubernetes and pulumi for infra, ArgoCD for the gitops and many more


## Convention
To carry out certain tasks in any directory, there are standard commands
`make <command>`
e.g to start local kubernetes cluster with all apps running with live reloading
```sh
    make dev
```


| Commands   |      Are      
|----------|:-------------:
|  setup    |  when fresh clone | 
|  install  |    install packages   |   
|  upgrade  | upgrade packages |    
|  sync     | synchronize/generate local code e.g graphql queries, kubernetes configs etc |    
|  dev      | start cluster/app locally in live reloading mode |    
|  format   | format code |   
|  check    | check that code aligns with standard |    
|  test     | run automated tests |    
