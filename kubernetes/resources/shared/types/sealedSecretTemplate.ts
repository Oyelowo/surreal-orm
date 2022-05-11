export interface SealedSecretTemplate {
    apiVersion: string;
    kind: string;
    metadata: Welcome3Metadata;
    spec: Spec;
}

export interface Welcome3Metadata {
    creationTimestamp: null;
    name: string;
    namespace: string;
}

export interface Spec {
    encryptedData?: Record<string, string>;
    template: Template;
}

export interface Template {
    data: null;
    metadata: TemplateMetadata;
    type: string
}

export interface TemplateMetadata {
    // annotations: Annotations;
    annotations: Record<string | "sealedsecrets.bitnami.com/managed", string>;
    creationTimestamp: null;
    labels: Record<string, string>;
    name: string;
    namespace: string;
}

