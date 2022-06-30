import { Namespace } from '@pulumi/kubernetes/core/v1';
import { namespacesNamesProvider } from './settings';
import { namespaces } from './util';

export const resourceNamespaces = Object.entries(namespaces).map(([_key, namespace]) => {
    const resourceNamespace = new Namespace(
        namespace,
        {
            metadata: {
                name: namespace,
                namespace,
                labels: {
                    'config.linkerd.io/admission-webhooks': namespace === 'linkerd' ? 'disabled' : '',
                },
                annotations: {
                    'linkerd.io/inject': 'enabled',
                },
            },
        },
        { provider: namespacesNamesProvider }
    );
    return resourceNamespace;
});
