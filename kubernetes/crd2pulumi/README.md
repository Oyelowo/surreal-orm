How to create a new CRD pulumi typescript from CRD

INFO: This is probably not worth fully automating for now since it's something that rarely changes and can be updated with a single command
```sh
# Install crd2Pulumi 
brew install pulumi/tap/crd2pulumi


# Local and url paths work
crd2pulumi --nodejsPath ./crd2pulumi/argocd  ./path/to/crd.yaml           


# Example
crd2pulumi --nodejsPath ./crd2pulumi/argocd  https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml           
```