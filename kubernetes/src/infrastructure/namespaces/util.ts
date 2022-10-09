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
    z.literal('tikv-admin'),
    z.literal('seaweedfs'),
    z.literal('fluvio-sys'),
    z.literal('rook-ceph'),
    z.literal('metalb'),
]);

export type Namespace = z.infer<typeof namespaceSchema>;

// TODO: Change to a function getNameSpace()
export const namespaces: Record<CamelCase<Namespace>, Namespace> = {
    applications: 'applications',
    argocd: 'argocd',
    certManager: 'cert-manager',
    linkerd: 'linkerd',
    linkerdViz: 'linkerd-viz',
    default: 'default',
    // Default namespace that comes with the deployment
    kubeSystem: 'kube-system',
    tikvAdmin: 'tikv-admin',
    seaweedfs: 'seaweedfs',
    fluvioSys: 'fluvio-sys',
    rookCeph: 'rook-ceph',
    metalb: 'metalb',
    // infrastructure: "infrastructure",
} as const;
