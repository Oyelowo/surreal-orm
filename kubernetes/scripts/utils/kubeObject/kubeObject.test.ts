import sh from 'shelljs';
import { getMainBaseDir } from './../../../src/resources/shared/directoriesManager.js';
import path from 'node:path';
import { KubeObject } from './kubeObject.js';
import type { TKubeObject } from './kubeObject.js';
import { expect, jest, test, describe } from '@jest/globals';
import { info } from 'node:console';

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
};

jest.spyOn(KubeObject.prototype, 'sealSecretValue').mockImplementation(
    ({ name, namespace, secretValue }) => 'lowo-test' + name + namespace + '*'.repeat(secretValue.length)
);

describe.skip('KubeObject', () => {
    beforeAll(() => {
        new KubeObject('test').getOfAKind('SealedSecret').forEach((ss) => {
            sh.rm('-rf', ss.path);
        });
    });
    afterEach(() => {
        new KubeObject('test').getOfAKind('SealedSecret').forEach((ss) => {
            sh.rm('-rf', ss.path);
        });
    });

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

        info('Can get kube objects for a resource');
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

    test('Can update sealed secrets', () => {
        const kubeInstance = new KubeObject('test');
        expect(kubeInstance.getOfAKind('SealedSecret')).toHaveLength(0);
        kubeInstance.syncSealedSecrets();
        expect(kubeInstance.getOfAKind('SealedSecret')).toHaveLength(13);

        expect(kubeInstance.getOfAKind('SealedSecret')[0].spec.encryptedData).toEqual({
            ADMIN_PASSWORD: 'lowo-testargocd-applications-secretargocd********',
            password: 'lowo-testargocd-applications-secretargocd********',
            type: 'lowo-testargocd-applications-secretargocd***',
            url: 'lowo-testargocd-applications-secretargocd**********************************************************',
            username: 'lowo-testargocd-applications-secretargocd*******',
        });
        expect(kubeInstance.getOfAKind('SealedSecret')[12].spec.encryptedData).toEqual({
            APP_ENVIRONMENT: 'lowo-testreact-webapplications********',
            APP_EXTERNAL_BASE_URL: 'lowo-testreact-webapplications****************************',
            APP_HOST: 'lowo-testreact-webapplications************',
            APP_PORT: 'lowo-testreact-webapplications********',
        });
        expect(kubeInstance.getOfAKind('SealedSecret').map(removeNonDeterministicRootDir)).toMatchSnapshot();
    });
});
