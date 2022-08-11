import { getMainBaseDir } from './../../../src/resources/shared/directoriesManager';
import path from 'node:path';
// jest.useFakeTimers()
import { KubeObject, TKubeObject } from './kubeObject.js';
import { expect, jest, test } from '@jest/globals';

const mockManifestsPath = path.join('scripts', 'utils', '__tests__', 'generatedManifests', 'local');
const rootDir = getMainBaseDir();
const manifestDir = path.join(rootDir, mockManifestsPath);
const privateFunc = jest.spyOn(KubeObject.prototype, 'getManifestsDir').mockImplementation(() => manifestDir);

const diff = (diffMe: string, diffBy: string): string => diffMe.split(diffBy).join('');

test('Test of life', () => {
    const kubeInstance = new KubeObject('local');
    expect(kubeInstance.getManifestsDir()).toContain(mockManifestsPath);
    expect(privateFunc).toHaveBeenCalled();

    // kubeInstance.syncSealedSecrets()
    const removeNonDeterministicRootDir = (p: TKubeObject) => ({ ...p, path: diff(p.path, rootDir) });
    const inst = kubeInstance.getAll().map(removeNonDeterministicRootDir)
    expect(inst).toMatchSnapshot();
    expect(inst).toHaveLength(267);

    // expect(kubeInstance.getAll()).toStrictEqual([]);
    // expect(kubeInstance).toMatchSnapshot();

    // Nas correct enviroment
    // expect(kubeInstance.getEnvironment()).toBe('local');
    // expect(kubeInstance.generateManifests()).toBe('local');
    // expect(kubeInstance.getAll()).toBe('local');
});
