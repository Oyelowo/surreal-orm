import { Namespace } from '@pulumi/kubernetes/core/v1';
import { namespacesNamesProvider } from './settings';
import { namespaces } from './util';

export const resourceNamespaces = Object.values(namespaces).map(namespace => {
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
                    // Let's start with meshing only application deployments which is done elsewhere
                    // 'linkerd.io/inject': 'enabled',
                },
            },
        },
        { provider: namespacesNamesProvider }
    );
    return resourceNamespace;
});
