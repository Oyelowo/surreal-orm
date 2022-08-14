import sh from 'shelljs';
import { getMainBaseDir } from './../../../src/resources/shared/directoriesManager.js';
import path from 'node:path';
import { KubeObject } from './kubeObject.js';
import type { TKubeObject } from './kubeObject.js';
import { expect, jest, test, describe } from '@jest/globals';
import { selectSecretKubeObjectsFromPrompt } from './secretsSelectorPrompter.js';
import { MockSTDIN, stdin } from 'mock-stdin'
import { log } from 'node:console';

// jest.setTimeout(130_000)

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

// Key codes
const keys = {
    up: '\u001B\u005B\u0041',
    down: '\u001B\u005B\u0042',
    enter: '\u000D',
    // space: ' ',
    space: '\u0020',
    // a: 'a',
    a: '\u0041',
}
// helper function for timing
const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))
describe.only('KubeObject', () => {
    // Mock stdin so we can send messages to the CLI
    // eslint-disable-next-line unicorn/no-null
    let io: MockSTDIN | null = null
    beforeAll(() => (io = stdin()))
    afterAll(() => io.restore())


    beforeEach(() => {
        // jest.setTimeout(500_000);
    })

    test('All gone', async () => {

        // jest.setTimeout(1_000_000_000)
        const sendKeystrokes = async () => {
            // Selection 1
            io.send(keys.down)
            io.send(keys.space)

            //  Selection 2
            io.send(keys.down)
            io.send(keys.space)

            //  Selection 3
            io.send(keys.down)
            io.send(keys.space)

            //  Selection 4
            io.send(keys.down)
            io.send(keys.down)
            io.send(keys.space)

            //  Selection 5
            io.send(keys.down)
            io.send(keys.down)
            io.send(keys.down)
            io.send(keys.space)
            io.send(keys.enter)
            await delay(10)


            // Subselection for Selection 1
            io.send(keys.a)
            // io.send(keys.a, 'ascii')
            io.send(keys.enter)
            await delay(10)

            // Subselection for Selection 2
            io.send(keys.a)
            io.send(keys.enter)
            await delay(10)

            // Subselection for Selection 3
            io.send(keys.down)
            io.send(keys.space)

            io.send(keys.down)
            io.send(keys.down)
            io.send(keys.space)

            io.send(keys.down)
            io.send(keys.space)
            io.send(keys.enter)
            await delay(10)

            // Subselection for Selection 4
            io.send(keys.down)
            io.send(keys.space)

            io.send(keys.down)
            io.send(keys.space)

            io.send(keys.enter)
            await delay(10)

            // Subselection for Selection 5
            io.send(keys.a)
            io.send(keys.enter)
            await delay(10)
        }
        // await sendKeystrokes()
        setTimeout(() => sendKeystrokes().then(), 5)

        const kubeInstance = new KubeObject('test');
        await kubeInstance.syncSealedSecretsWithPrompt()
        // const kk = await selectSecretKubeObjectsFromPrompt(kubeInstance.getOfAKind('Secret'))

        expect(kubeInstance.getOfAKind("SealedSecret").map(removeNonDeterministicRootDir)).toHaveLength(13);
        expect(kubeInstance.getOfAKind("SealedSecret").map(removeNonDeterministicRootDir)).toMatchSnapshot();
        // done()
    });
});
