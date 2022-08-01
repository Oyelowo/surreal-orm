import c from 'chalk';
import p from 'path';
import sh from 'shelljs';
import { getMainBaseDir } from '../../../resources/shared/manifestsDirectory.js';
import { getEnvVarsForScript, handleShellError } from '../shared.js';
import { TKubeObject, KubeObject } from './kubeObject.js';
import { getImageTagsFromDir } from '../getImageTagsFromDir.js';

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
    // Pulumi needs some environment variables set for generating deployments with image tag
    /* `export ${IMAGE_TAG_REACT_WEB}=tag-web export ${IMAGE_TAG_GRAPHQL_MONGO}=tag-mongo`
     */
    handleShellError(
        sh.exec(
            `
        ${getEnvVarsForScript(kubeObject.getEnvironment(), imageTags)}
        export PULUMI_CONFIG_PASSPHRASE="not-needed"
        pulumi up --yes --skip-preview --stack dev
       `
        )
    );

    sh.rm('-rf', './login');
}
