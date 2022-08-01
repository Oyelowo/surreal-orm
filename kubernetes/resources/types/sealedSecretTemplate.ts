import { Namespace } from './../infrastructure/namespaces/util.js';
import { v1alpha1 } from '../../generatedCrdsTs/bitnami/index.js';

type SealedSecretArguments = NonNullable<ConstructorParameters<typeof v1alpha1.SealedSecret>[1]>;
export type SealedSecretTemplate = SealedSecretArguments & {
    metadata: Welcome3Metadata;
    spec: Spec;
};

export type Welcome3Metadata = {
    name?: string;
    namespace?: Namespace;
};

export type Spec = {
    encryptedData?: Record<string, string | null>;
    template: Template;
};

export type Template = {
    data: null;
    metadata: TemplateMetadata;
    type: string;
};

export interface TemplateMetadata {
    annotations: Record<string, string>;
    creationTimestamp: null;
    labels: Record<string, string>;
    name: string;
    namespace: string;
}
