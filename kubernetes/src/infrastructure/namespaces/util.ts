import { z } from 'zod';
import { CamelCase } from 'type-fest';
export const namespaceSchema = z.union([
    z.literal('applications'),
    z.literal('argocd'),
    z.literal('cert-manager'),
    z.literal('linkerd'),
    z.literal('linkerd-viz'),
    z.literal('default'),
    z.literal('kube-system'),
]);

export type Namespace = z.infer<typeof namespaceSchema>;

export const namespaces: Record<CamelCase<Namespace>, Namespace> = {
    applications: 'applications',
    argocd: 'argocd',
    certManager: 'cert-manager',
    linkerd: 'linkerd',
    linkerdViz: 'linkerd-viz',
    default: 'default',
    // Default namespace that comes with the deployment
    kubeSystem: 'kube-system',
    // infrastructure: "infrastructure",
} as const;
