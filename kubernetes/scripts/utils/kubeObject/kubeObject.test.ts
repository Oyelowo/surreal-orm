import { getMainBaseDir } from './../../../src/resources/shared/directoriesManager';
import path from 'node:path';
// jest.useFakeTimers()
import { KubeObject } from './kubeObject.js';
import type { TKubeObject } from './kubeObject.js';
import { expect, jest, test, describe } from '@jest/globals';

const mockManifestsPath = path.join('scripts', 'utils', '__tests__', 'generatedManifests', 'local');
const rootDir = getMainBaseDir();
const manifestDir = path.join(rootDir, mockManifestsPath);
const privateFunc = jest.spyOn(KubeObject.prototype, 'getManifestsDir').mockImplementation(() => manifestDir);

const diff = (diffMe: string, diffBy: string): string => diffMe.split(diffBy).join('');


describe('KubeObject', () => { 
    test('Can sync resources', () => {
        const kubeInstance = new KubeObject('local');
        expect(kubeInstance.getManifestsDir()).toContain(mockManifestsPath);
        expect(privateFunc).toHaveBeenCalled();

        const removeNonDeterministicRootDir = (p: TKubeObject) => ({ ...p, path: diff(p.path, rootDir) });
        const inst = kubeInstance.getAll().map(removeNonDeterministicRootDir);
        expect(inst).toMatchSnapshot();
        expect(inst).toHaveLength(267);

        const inst2 = kubeInstance.getOfAKind('Deployment').map(removeNonDeterministicRootDir);
        expect(inst2).toMatchSnapshot();
        expect(inst2).toHaveLength(21);

        expect(kubeInstance.getOfAKind('Secret').map(removeNonDeterministicRootDir)).toMatchSnapshot();
        expect(kubeInstance.getOfAKind('SealedSecret').map(removeNonDeterministicRootDir)).toMatchSnapshot();
        expect(kubeInstance.getOfAKind('CustomResourceDefinition').map(removeNonDeterministicRootDir)).toMatchSnapshot()
    });

    test.only('Can get kube objects for a resource', () => {
        const kubeInstance = new KubeObject('local');
        expect(kubeInstance.getForApp("graphql-mongo")).toMatchSnapshot();
        expect(kubeInstance.getForApp("react-web")).toMatchSnapshot();
        expect(kubeInstance.getForApp("argocd")).toMatchSnapshot();
        expect(kubeInstance.getForApp("linkerd")).toMatchSnapshot();
        expect(kubeInstance.getForApp("cert-manager")).toMatchSnapshot();
        expect(kubeInstance.getForApp("nginx-ingress")).toMatchSnapshot();
        expect(kubeInstance.getForApp("namespaces")).toMatchSnapshot();
    });

 })