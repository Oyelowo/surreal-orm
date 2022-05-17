// export const devNamespaceName = devNamespace.metadata.name as unknown as string;
// type Keys = keyof typeof KeyToVal;
// type Values = typeof KeyToVal[Keys];

export type NamespaceName =
  | "applications"
  | "argocd"
  | "cert-manager"
  | "linkerd"
  | "linkerd-viz"
  | "default"
  | "kube-system";

type CamelCase<S extends string> =
  S extends `${infer P1}-${infer P2}${infer P3}`
    ? `${Lowercase<P1>}${Uppercase<P2>}${CamelCase<P3>}`
    : Lowercase<S>;

export const namespaceNames: Record<CamelCase<NamespaceName>, NamespaceName> = {
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
