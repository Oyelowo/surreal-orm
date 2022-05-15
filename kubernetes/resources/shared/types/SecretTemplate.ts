import { APPLICATION_AUTOMERGE_ANNOTATION } from './../constants';

// TODO: Use zod to parse this
export interface SecretTemplate {
    apiVersion: string;
    kind: string;
    metadata: Metadata;
    type: string
    stringData?: Record<string, string>;
    data?: Record<string, string>;
}

export interface Metadata {
    annotations: Record<"sealedsecrets.bitnami.com/managed", string> & typeof APPLICATION_AUTOMERGE_ANNOTATION;
    // annotations: Annotations;
    labels: Record<string, string>;
    // labels: Labels;
    name: string;
    namespace: string;
    creationTimestamp: null
}
