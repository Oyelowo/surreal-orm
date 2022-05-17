

// NOTE: I initially was encoding the secrets in base64 but it turns out
// that bitnami sealed secrets not only handles encryption but base64 encoding of the
// secrets before encrypting them
export const secretsSample = {
  "graphql-mongo": {
    MONGODB_USERNAME: "",
    MONGODB_PASSWORD: "",
    MONGODB_ROOT_USERNAME: "",
    MONGODB_ROOT_PASSWORD: "",
    REDIS_USERNAME: "",
    REDIS_PASSWORD: "",
  },
  "grpc-mongo": {
    MONGODB_USERNAME: "",
    MONGODB_PASSWORD: "",
    MONGODB_ROOT_USERNAME: "",
    MONGODB_ROOT_PASSWORD: "",
  },
  "graphql-postgres": {
    POSTGRES_USERNAME: "",
    POSTGRES_PASSWORD: "",
  },
  "react-web": {
    GITHUB_CLIENT_ID: "",
    GITHUB_CLIENT_SECRET: "",
    GOOGLE_CLIENT_ID: "",
    GOOGLE_CLIENT_SECRET: "",
  },
  argocd: {
    ADMIN_PASSWORD: "",
    type: "git",
    url: "https://github.com/Oyelowo/modern-distributed-app-template",
    username: "Oyelowo",
    password: "my-password-or-personal-access-token",
  },
  "argocd-applications-children-infrastructure": {},
  "argocd-applications-children-services": {},
  "argocd-applications-parents": {},
  "cert-manager": {},
  "linkerd": {},
  "linkerd-viz": {},
  "namespace-names": {},
  "nginx-ingress": {},
  "sealed-secrets": {}
};


// Record<ResourceName, Record<string, string>>