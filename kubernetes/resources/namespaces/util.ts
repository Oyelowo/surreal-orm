// export const devNamespaceName = devNamespace.metadata.name as unknown as string;
export const namespaceNames = {
    applications: "applications",
    argocd: "argocd",
    certManager: "cert-manager",
    linkerd: "linkerd",
    linkerdViz: "linkerd-viz",
    default: "default",
    // Default namespace that comes with the deployment
    kubeSystem: "kube-system",
    // infrastructure: "infrastructure",
} as const;