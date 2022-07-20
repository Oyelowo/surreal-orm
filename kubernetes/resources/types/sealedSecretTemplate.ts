import { Namespace } from './../infrastructure/namespaces/util';
import { v1alpha1 } from "../../crds-generated/bitnami";

type SealedSecretArguments = ConstructorParameters<typeof v1alpha1.SealedSecret>[1]
export type SealedSecretTemplate = SealedSecretArguments & {
    metadata: Welcome3Metadata;
    spec: Spec;
}

export type Welcome3Metadata = {
    name?: string;
    namespace?: Namespace;
}

export type Spec = {
    encryptedData?: Record<string, string>;
    template: Template;
}

export type Template = {
    data: null;
    metadata: TemplateMetadata;
    type: string;
}

export interface TemplateMetadata {
    annotations: Record<'sealedsecrets.bitnami.com/managed' | string, string>
    creationTimestamp: null;
    labels: Record<string, string>;
    name: string;
    namespace: string;
}


