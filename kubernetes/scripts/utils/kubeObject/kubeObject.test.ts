import { getMainBaseDir } from './../../../src/resources/shared/directoriesManager';
import path from 'node:path';
// jest.useFakeTimers()
import { KubeObject } from './kubeObject.js';
import type { TKubeObject } from './kubeObject.js';
import { expect, jest, test, describe } from '@jest/globals';

const mockManifestsPath = path.join('scripts', 'utils', '__tests__', 'generatedManifests', 'local');
const rootDir = getMainBaseDir();
const manifestDir = path.join(rootDir, mockManifestsPath);
const getManifestsDirMock = jest.spyOn(KubeObject.prototype, 'getManifestsDir').mockImplementation(() => manifestDir);

// const getResourceAbsolutePathMock = jest.spyOn(KubeObject.prototype, 'getResourceAbsolutePath').mockImplementation((props) => getResourceAbsolutePathForTest({
//     environment: props.environment,
//     resourceType: props.resourceType,
//     resourceName: props.resourceName,
//     manifestsDir: manifestDir
// }));

const diff = (diffMe: string, diffBy: string): string => diffMe.split(diffBy).join('');

describe('KubeObject', () => {
    test('Can sync resources', () => {
        const kubeInstance = new KubeObject('local');
        expect(kubeInstance.getManifestsDir()).toContain(mockManifestsPath);
        expect(getManifestsDirMock).toHaveBeenCalled();

        const removeNonDeterministicRootDir = (p: TKubeObject) => ({ ...p, path: diff(p.path, rootDir) });
        const inst = kubeInstance.getAll().map(removeNonDeterministicRootDir);
        expect(inst).toMatchSnapshot();
        expect(inst).toHaveLength(267);

        const inst2 = kubeInstance.getOfAKind('Deployment').map(removeNonDeterministicRootDir);
        expect(inst2).toMatchSnapshot();
        expect(inst2).toHaveLength(21);

        expect(kubeInstance.getOfAKind('Secret').map(removeNonDeterministicRootDir)).toMatchSnapshot();
        expect(kubeInstance.getOfAKind('SealedSecret').map(removeNonDeterministicRootDir)).toMatchSnapshot();
        expect(
            kubeInstance.getOfAKind('CustomResourceDefinition').map(removeNonDeterministicRootDir)
        ).toMatchSnapshot();
    });

    test.only('Can get kube objects for a resource', () => {
        const kubeInstance = new KubeObject('local');
        const graphqlMongo = kubeInstance.getForApp({ resourceType: 'services', resourceName: 'graphql-mongo' });
        expect(graphqlMongo).toMatchSnapshot();
        expect(graphqlMongo).toHaveLength(22);

        const reactWeb = kubeInstance.getForApp({ resourceType: 'services', resourceName: 'react-web' });
        expect(reactWeb).toMatchSnapshot();
        expect(reactWeb).toHaveLength(4);

        const argocd = kubeInstance.getForApp({ resourceType: 'infrastructure', resourceName: 'argocd' });
        expect(argocd).toMatchSnapshot();
        expect(argocd).toHaveLength(36);

        const linkerd = kubeInstance.getForApp({ resourceType: 'infrastructure', resourceName: 'linkerd' });
        expect(linkerd).toMatchSnapshot();
        expect(linkerd).toHaveLength(44);

        const certManager = kubeInstance.getForApp({ resourceType: 'infrastructure', resourceName: 'cert-manager' });
        expect(certManager).toMatchSnapshot();
        expect(certManager).toHaveLength(57);

        const nginxIngress = kubeInstance.getForApp({ resourceType: 'infrastructure', resourceName: 'nginx-ingress' });
        expect(nginxIngress).toMatchSnapshot();
        expect(nginxIngress).toHaveLength(12);

        const namespaces = kubeInstance.getForApp({ resourceType: 'infrastructure', resourceName: 'namespaces' });
        expect(namespaces).toMatchSnapshot();
        expect(namespaces).toHaveLength(7);
    });
});
