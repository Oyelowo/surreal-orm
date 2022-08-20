import { getMainBaseDir } from '../../../src/resources/shared/directoriesManager.js';
import c from 'chalk';
import p from 'node:path';
import sh from 'shelljs';
import { getEnvVarsForScript, handleShellError } from '../shared.js';
import { KubeObject } from './kubeObject.js';
import type { TKubeObject } from './kubeObject.js';
import { getImageTagsFromDir } from '../getImageTagsFromDir.js';
import path from 'node:path';
import { randomUUID } from 'node:crypto';

/*
GENERATE ALL KUBERNETES MANIFESTS USING PULUMI
*/

const mainDir = getMainBaseDir();
export const tsConfigPath = path.join(mainDir, 'tsconfig.pulumi.json');
export async function generateManifests(kubeObject: KubeObject) {
    sh.exec('make install');
    const loginDir = path.join(mainDir, `.login-${randomUUID()}`);
    sh.rm('-rf', loginDir);
    sh.mkdir('-p', loginDir);

    // https://www.pulumi.com/docs/intro/concepts/state/#logging-into-the-local-filesystem-backend
    sh.exec(`pulumi login file://${loginDir}`);

    sh.echo(c.blueBright(`DELETE EXISTING RESOURCES(except sealed secrets)`));

    const removeNonSealedSecrets = (obj: TKubeObject) => {
        const isSealedSecret = obj.kind === 'SealedSecret';
        !isSealedSecret && sh.rm('-rf', obj.path);
    };

    kubeObject.getAll().forEach(removeNonSealedSecrets);

    handleShellError(sh.rm('-rf', `${p.join(getMainBaseDir(), 'Pulumi.dev.yaml')}`));
    handleShellError(sh.exec("export PULUMI_CONFIG_PASSPHRASE='not-needed' && pulumi stack init --stack dev"));

    const imageTags = await getImageTagsFromDir();
    // Pulumi needs some environment variables set for generating deployments with image tag
    /* `export ${IMAGE_TAG_REACT_WEB}=tag-web export ${IMAGE_TAG_GRAPHQL_MONGO}=tag-mongo`
     */
    handleShellError(
        sh.exec(
            `
        ${getEnvVarsForScript()}
        export PULUMI_CONFIG_PASSPHRASE="not-needed"
        export PULUMI_NODEJS_TRANSPILE_ONLY=true
        export PULUMI_SKIP_CONFIRMATIONS=true
        export PULUMI_NODEJS_TSCONFIG_PATH=${tsConfigPath}
        pulumi up --yes --skip-preview --stack dev
       `
        )
    );
    sh.rm('-rf', loginDir);
}
