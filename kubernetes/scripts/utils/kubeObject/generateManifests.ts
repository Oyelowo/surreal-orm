import { getMainBaseDir } from '../../../src/resources/shared/directoriesManager.js';
import c from 'chalk';
import p from 'node:path';
import sh from 'shelljs';
import { getEnvVarsForScript, handleShellError } from '../shared.js';
import { KubeObject } from './kubeObject.js';
import type { TKubeObject } from './kubeObject.js';
import { getImageTagsFromDir } from '../getImageTagsFromDir.js';
import path from 'node:path';

/*
GENERATE ALL KUBERNETES MANIFESTS USING PULUMI
*/

export async function generateManifests(kubeObject: KubeObject) {
    sh.exec('npm i');
    sh.rm('-rf', './login');
    sh.mkdir('./login');

    sh.exec('pulumi login file://login');

    sh.echo(c.blueBright(`DELETE EXISTING RESOURCES(except sealed secrets)`));

    const removeNonSealedSecrets = (obj: TKubeObject) => {
        const isSealedSecret = obj.kind === 'SealedSecret';
        !isSealedSecret && sh.rm('-rf', obj.path);
    };

    kubeObject.getAll().forEach(removeNonSealedSecrets);

    handleShellError(sh.rm('-rf', `${p.join(getMainBaseDir(), 'Pulumi.dev.yaml')}`));
    handleShellError(sh.exec("export PULUMI_CONFIG_PASSPHRASE='not-needed' && pulumi stack init --stack dev"));

    const imageTags = await getImageTagsFromDir();
    const mainDir = getMainBaseDir();
    const tsConfigPath = path.join(mainDir, 'tsconfig.pulumi.json');
    // Pulumi needs some environment variables set for generating deployments with image tag
    /* `export ${IMAGE_TAG_REACT_WEB}=tag-web export ${IMAGE_TAG_GRAPHQL_MONGO}=tag-mongo`
     */
    handleShellError(
        sh.exec(
            `
        ${getEnvVarsForScript(kubeObject.getEnvironment(), imageTags)}
        export PULUMI_CONFIG_PASSPHRASE="not-needed"
        export PULUMI_NODEJS_TRANSPILE_ONLY=true
        export PULUMI_SKIP_CONFIRMATIONS=true
        export PULUMI_NODEJS_TSCONFIG_PATH=${tsConfigPath}
        pulumi up --yes --skip-preview --stack dev
       `
        )
    );
    sh.rm('-rf', './login');
}
