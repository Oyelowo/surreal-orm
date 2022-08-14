import sh from 'shelljs';
import { KubeObject } from './kubeObject.js';
import { expect, jest, test, describe } from '@jest/globals';
import { info } from 'node:console';
import { MockSTDIN, stdin } from 'mock-stdin';

// Key codes
const keys = {
    up: '\u001B\u005B\u0041',
    down: '\u001B\u005B\u0042',
    enter: '\u000D',
    space: '\u0020',
    a: '\u0041',
};
// helper function for timing
const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

jest.spyOn(KubeObject.prototype, 'sealSecretValue').mockImplementation(
    ({ name, namespace, secretValue }) => 'lowo-test' + name + namespace + '*'.repeat(secretValue.length)
);

function deleteSealedSecrets() {
    new KubeObject('test').getOfAKind('SealedSecret').forEach((ss) => {
        sh.rm('-rf', ss.path);
    });
}

describe('KubeObject', () => {
    // Mock stdin so we can send messages to the CLI

    let io: MockSTDIN | undefined;
    afterAll(() => io.restore());

    beforeAll(() => {
        io = stdin();
        deleteSealedSecrets();
    });
    afterEach(() => {
        deleteSealedSecrets();
    });

    test('Can sync resources', () => {
        const kubeInstance = new KubeObject('test');

        const inst = kubeInstance.getAll();
        expect(inst).toHaveLength(255);

        const inst2 = kubeInstance.getOfAKind('Deployment');
        expect(inst2).toHaveLength(21);

        info('Can get kube objects for a resource');
        const graphqlMongo = kubeInstance.getForApp('services/graphql-mongo');
        expect(graphqlMongo).toHaveLength(19);

        const reactWeb = kubeInstance.getForApp('services/react-web');
        expect(reactWeb).toHaveLength(4);

        const argocd = kubeInstance.getForApp('infrastructure/argocd');
        expect(argocd).toHaveLength(34);

        const linkerd = kubeInstance.getForApp('infrastructure/linkerd');
        expect(linkerd).toHaveLength(41);

        const certManager = kubeInstance.getForApp('infrastructure/cert-manager');
        expect(certManager).toHaveLength(57);

        const nginxIngress = kubeInstance.getForApp('infrastructure/nginx-ingress');
        expect(nginxIngress).toHaveLength(12);

        const namespaces = kubeInstance.getForApp('infrastructure/namespaces');
        expect(namespaces).toHaveLength(7);
    });

    test('Can update sealed secrets', () => {
        const kubeInstance = new KubeObject('test');
        const sealedSecrets = kubeInstance.getOfAKind('SealedSecret');
        expect(sealedSecrets).toHaveLength(0);

        kubeInstance.syncSealedSecrets();

        const sealedSecretsUpdated = kubeInstance.getOfAKind('SealedSecret');
        expect(sealedSecretsUpdated).toHaveLength(13);
    });

    test('Can create sealed secrets from selected secrets', async () => {
        const sendKeystrokes = async () => {
            // Selection 1
            io.send(keys.down);
            io.send(keys.space);

            //  Selection 2
            io.send(keys.down);
            io.send(keys.space);

            //  Selection 3
            io.send(keys.down);
            io.send(keys.space);

            //  Selection 4
            io.send(keys.down);
            io.send(keys.down);
            io.send(keys.space);

            //  Selection 5
            io.send(keys.down);
            io.send(keys.space);

            // First Confirmation
            io.send(keys.enter);
            await delay(10);

            // Subselection for Selection 1
            io.send(keys.a);
            // io.send(keys.a, 'ascii')
            io.send(keys.enter);
            await delay(10);

            // Subselection for Selection 2
            io.send(keys.a);
            io.send(keys.enter);
            await delay(10);

            // Subselection for Selection 3
            io.send(keys.down);
            io.send(keys.space);

            io.send(keys.enter);
            await delay(10);

            // Subselection for Selection 4
            io.send(keys.down);
            io.send(keys.space);

            io.send(keys.enter);
            await delay(10);

            // Subselection for Selection 5
            io.send(keys.a);
            io.send(keys.enter);
            await delay(10);
        };
        setTimeout(() => sendKeystrokes().then(), 10);

        const kubeInstance = new KubeObject('test');
        await kubeInstance.syncSealedSecretsWithPrompt();

        const sealedecrets = kubeInstance.getOfAKind('SealedSecret');

        info('Should have five selections cos we have updated only 5 sealed secrets');
        expect(sealedecrets).toHaveLength(5);
    });

    test('Can update sealed secrets after initial', async () => {
        const kubeInstance = new KubeObject('test');
        jest.spyOn(kubeInstance, 'sealSecretValue').mockImplementation(() => 'inital-secrets');

        kubeInstance.syncSealedSecrets();
        const sealedSecrets = kubeInstance.getOfAKind('SealedSecret');
        const secrets = kubeInstance.getOfAKind('Secret');

        info('Should have 13 sealed secrets initially generated from 13 secrets');
        expect(secrets).toHaveLength(13);
        expect(sealedSecrets).toHaveLength(13);
        expect(
            Object.values(sealedSecrets.filter((ss) => Object.values(ss.spec.encryptedData).includes('inital-secrets')))
        ).toHaveLength(13);

        const sendKeystrokes = async () => {
            // Selection 1
            io.send(keys.down);
            io.send(keys.space);

            //  Selection 2
            io.send(keys.down);
            io.send(keys.space);

            //  Selection 3
            io.send(keys.down);
            io.send(keys.space);

            //  Selection 4
            io.send(keys.down);
            io.send(keys.down);
            io.send(keys.space);

            //  Selection 5
            io.send(keys.down);
            io.send(keys.down);
            io.send(keys.down);
            io.send(keys.space);
            io.send(keys.enter);
            await delay(10);

            // Subselection for Selection 1
            io.send(keys.a);
            // io.send(keys.a, 'ascii')
            io.send(keys.enter);
            await delay(10);

            // Subselection for Selection 2
            io.send(keys.a);
            io.send(keys.enter);
            await delay(10);

            // Subselection for Selection 3
            io.send(keys.down);
            io.send(keys.space);

            io.send(keys.down);
            io.send(keys.down);
            io.send(keys.space);

            io.send(keys.down);
            io.send(keys.space);
            io.send(keys.enter);
            await delay(10);

            // Subselection for Selection 4
            io.send(keys.down);
            io.send(keys.space);

            io.send(keys.down);
            io.send(keys.space);

            io.send(keys.enter);
            await delay(10);

            // Subselection for Selection 5
            io.send(keys.a);
            io.send(keys.enter);
            await delay(10);
        };
        setTimeout(() => sendKeystrokes().then(), 5);

        const kubeInstance2 = new KubeObject('test');
        jest.spyOn(kubeInstance2, 'sealSecretValue').mockImplementation(() => 'updated-secrets');
        await kubeInstance2.syncSealedSecretsWithPrompt();
        const sealedSecrets2 = kubeInstance2.getOfAKind('SealedSecret');

        info('Should update from 13 sealed secrets still, with specific secret data fields updated.');
        expect(sealedSecrets2).toHaveLength(13);
        // 5 secrets have been updated
        expect(
            Object.values(
                sealedSecrets2.filter((ss) => Object.values(ss.spec.encryptedData).includes('updated-secrets'))
            )
        ).toHaveLength(5);
    });
});
