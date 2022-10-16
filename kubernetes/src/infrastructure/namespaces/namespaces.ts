import * as pulumi from '@pulumi/kubernetes';
import { namespaces } from '../../types/ownTypes.js';
import { namespacesNamesProvider } from './settings.js';

export const resourceNamespaces = Object.values(namespaces).map((namespace) => {
    const resourceNamespace = new pulumi.core.v1.Namespace(
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
