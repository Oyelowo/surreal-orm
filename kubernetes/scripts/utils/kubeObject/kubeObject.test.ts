import sh from 'shelljs';
import { getMainBaseDir } from './../../../src/resources/shared/directoriesManager.js';
import path from 'node:path';
import { KubeObject } from './kubeObject.js';
import type { TKubeObject } from './kubeObject.js';
import { expect, jest, test, describe } from '@jest/globals';
import { info } from 'node:console';
import { MockSTDIN, stdin } from 'mock-stdin';
import { faker } from '@faker-js/faker';
import _ from 'lodash';

// jest.setTimeout(130_000)

// Key codes
const keys = {
    up: '\u001B\u005B\u0041',
    down: '\u001B\u005B\u0042',
    enter: '\u000D',
    // space: ' ',
    space: '\u0020',
    // a: 'a',
    a: '\u0041',
};
// helper function for timing
const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
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

faker.seed(1);
jest.spyOn(KubeObject.prototype, 'sealSecretValue').mockImplementation(
    ({ name, namespace, secretValue }) =>
        'lowo-test' + name + namespace + faker.internet.password() + '*'.repeat(secretValue.length)
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
        faker.seed(1);
        io = stdin();
        deleteSealedSecrets();
    });
    afterEach(() => {
        deleteSealedSecrets();
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
            ADMIN_PASSWORD: 'lowo-testargocd-applications-secretargocdHey7F2EAFyTqHbR********',
            password: 'lowo-testargocd-applications-secretargocd4Kt6SwHLVIz3jme********',
            type: 'lowo-testargocd-applications-secretargocdmbRtsuoo1tYIHS3***',
            url: 'lowo-testargocd-applications-secretargocdbbkpHh_hKk6KMwv**********************************************************',
            username: 'lowo-testargocd-applications-secretargocdRax5wOXVWX9c6SH*******',
        });
        expect(kubeInstance.getOfAKind('SealedSecret')[12].spec.encryptedData).toEqual({
            APP_ENVIRONMENT: 'lowo-testreact-webapplicationsLi05LARh9MsPbas********',
            APP_EXTERNAL_BASE_URL: 'lowo-testreact-webapplicationsjBFrRymKcbhKWwm****************************',
            APP_HOST: 'lowo-testreact-webapplicationsOrVoLanW8IVmUB8************',
            APP_PORT: 'lowo-testreact-webapplicationsr38g8j9IRmEB5qi********',
        });
        expect(kubeInstance.getOfAKind('SealedSecret').map(removeNonDeterministicRootDir)).toMatchSnapshot();
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

        const kubeInstance = new KubeObject('test');
        await kubeInstance.syncSealedSecretsWithPrompt();

        const sealedecrets = kubeInstance.getOfAKind('SealedSecret');

        info('Should have five selections cos we have updated only 5 sealed secrets');
        expect(sealedecrets).toHaveLength(5);
        expect(sealedecrets.filter((ss) => !_.isEmpty(ss.spec.encryptedData))).toHaveLength(5);
        expect(sealedecrets.map(removeNonDeterministicRootDir)).toMatchSnapshot();
    });

    test('Can update sealed secrets after initial', async () => {
        const kubeInstance = new KubeObject('test');
        jest.spyOn(kubeInstance, 'sealSecretValue').mockImplementation(
            ({ name, namespace, secretValue }) =>
                'inital-secrets' + name + namespace + faker.internet.password() + '*'.repeat(secretValue.length)
        );

        kubeInstance.syncSealedSecrets();
        const sealedSecrets = kubeInstance.getOfAKind('SealedSecret');
        const secrets = kubeInstance.getOfAKind('SealedSecret');

        info('Should have 13 sealed secrets initially generated from 13 secrets');
        expect(secrets).toHaveLength(13);
        expect(sealedSecrets).toHaveLength(13);
        expect(sealedSecrets.filter((ss) => !_.isEmpty(ss.spec.encryptedData))).toHaveLength(13);
        expect(sealedSecrets.map(removeNonDeterministicRootDir)).toMatchSnapshot();

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
        jest.spyOn(kubeInstance2, 'sealSecretValue').mockImplementation(
            ({ name, namespace, secretValue }) =>
                'updated-secrets' + name + namespace + faker.internet.password() + '*'.repeat(secretValue.length)
        );
        await kubeInstance2.syncSealedSecretsWithPrompt();
        const sealedSecrets2 = kubeInstance2.getOfAKind('SealedSecret');

        info('Should update from 13 sealed secrets still, with specific secret data fields updated.');
        expect(sealedSecrets2).toHaveLength(13);
        expect(sealedSecrets2.filter((ss) => !_.isEmpty(ss.spec.encryptedData))).toHaveLength(13);
        expect(sealedSecrets2.map(removeNonDeterministicRootDir)).toMatchSnapshot();
    });
});
