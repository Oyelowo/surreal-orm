import sh from 'shelljs';
import { getMainBaseDir } from './../../../src/resources/shared/directoriesManager';
import path from 'node:path';
import { KubeObject } from './kubeObject.js';
import type { TKubeObject } from './kubeObject.js';
import { expect, test, describe } from '@jest/globals';

/* 
Remove teh absolute root to make the snapshot deterministic
// e.g
    from -> "path": "/Users/oyelowo/Desktop/dev/modern-distributed-app-template/kubernetes/generatedManifests/local/infrastructure/namespaces/1-manifest/v1-namespace-cert-manager-cert-manager.yaml",
    to   ->  "path": "generatedManifests/local/infrastructure/namespaces/1-manifest/v1-namespace-cert-manager-cert-manager.yaml",
*/
const removeNonDeterministicRootDir = (p: TKubeObject) => {
    return {
        ...p,
        path: path.relative(getMainBaseDir(), p.path),
        resourceBaseDir: path.relative(getMainBaseDir(), p.resourceBaseDir),
    } as TKubeObject;
}
describe('KubeObject', () => {
    beforeEach(() => {
        new KubeObject('test').getOfAKind("SealedSecret").forEach(ss => {
            sh.rm('-rf', ss.path)
        })
    })
    test('Can sync resources', () => {
        const kubeInstance = new KubeObject('test');

        const inst = kubeInstance.getAll().map(removeNonDeterministicRootDir);
        expect(inst).toMatchSnapshot();
        expect(inst).toHaveLength(255);

        const inst2 = kubeInstance.getOfAKind('Deployment').map(removeNonDeterministicRootDir);
        expect(inst2).toMatchSnapshot();
        expect(inst2).toHaveLength(21);

        expect(kubeInstance.getOfAKind('Secret').map(removeNonDeterministicRootDir)).toMatchSnapshot();
        expect(kubeInstance.getOfAKind('SealedSecret').map(removeNonDeterministicRootDir)).toMatchSnapshot();
        expect(
            kubeInstance.getOfAKind('CustomResourceDefinition').map(removeNonDeterministicRootDir)
        ).toMatchSnapshot();
    });

    test('Can get kube objects for a resource', () => {
        const kubeInstance = new KubeObject('test');
        const graphqlMongo = kubeInstance.getForApp('services/graphql-mongo').map(removeNonDeterministicRootDir);
        expect(graphqlMongo).toMatchSnapshot();
        expect(graphqlMongo).toHaveLength(19);

        const reactWeb = kubeInstance.getForApp('services/react-web').map(removeNonDeterministicRootDir);
        expect(reactWeb).toMatchSnapshot();
        expect(reactWeb).toHaveLength(4);

        const argocd = kubeInstance.getForApp('infrastructure/argocd').map(removeNonDeterministicRootDir);
        expect(argocd).toMatchSnapshot();
        expect(argocd).toHaveLength(34);

        const linkerd = kubeInstance.getForApp('infrastructure/linkerd').map(removeNonDeterministicRootDir);
        expect(linkerd).toMatchSnapshot();
        expect(linkerd).toHaveLength(41);

        const certManager = kubeInstance.getForApp('infrastructure/cert-manager').map(removeNonDeterministicRootDir);
        expect(certManager).toMatchSnapshot();
        expect(certManager).toHaveLength(57);

        const nginxIngress = kubeInstance.getForApp('infrastructure/nginx-ingress').map(removeNonDeterministicRootDir);
        expect(nginxIngress).toMatchSnapshot();
        expect(nginxIngress).toHaveLength(12);

        const namespaces = kubeInstance.getForApp('infrastructure/namespaces').map(removeNonDeterministicRootDir);
        expect(namespaces).toMatchSnapshot();
        expect(namespaces).toHaveLength(7);
    });
});
