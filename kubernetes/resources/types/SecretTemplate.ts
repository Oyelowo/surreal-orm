export interface SecretTemplate {
    apiVersion: string;
    kind: string;
    metadata: Metadata;
    type: string;
    data: Record<string, string>;
    stringData?: Record<string, string>;
}

export interface Metadata {
    annotations: Record<'sealedsecrets.bitnami.com/managed', string>;
    // annotations: Annotations;
    labels: Record<string, string>;
    // labels: Labels;
    name: string;
    namespace: string;
    creationTimestamp: null;
}
