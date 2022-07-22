import { Environment } from './../../resources/types/own-types';
import c from 'chalk';
import p from 'path';
import sh from 'shelljs';
import {
    getGeneratedCrdsCodeDir,
    getGeneratedEnvManifestsDir,
    getMainBaseDir,
} from '../../resources/shared/manifestsDirectory';
import { ImageTags } from '../../resources/shared/validations';
import {
    getAllKubeManifestsInfo,
    getEnvVarsForScript,
    getKubeManifestsPaths,
    handleShellError,
    KubeObjectInfo,
} from './shared';
import path from 'path';
/*
GENERATE ALL KUBERNETES MANIFESTS USING PULUMI
*/

type GenerateManifestsProp = {
    environment: Environment;
    imageTags: ImageTags;
    allManifestsInfo: KubeObjectInfo[]
};

export async function generateManifests({ environment, imageTags, allManifestsInfo }: GenerateManifestsProp) {
    const manifestsDirForEnv = getGeneratedEnvManifestsDir(environment);

    sh.exec('npm i');
    sh.rm('-rf', './login');
    sh.mkdir('./login');

    sh.exec('pulumi login file://login');

    sh.echo(c.blueBright(`DELETE EXISTING RESOURCES(except sealed secrets) at ${manifestsDirForEnv}`));
    const removeNonSealedSecrets = (info: KubeObjectInfo) => {
        const isSealedSecret = info.kind === 'SealedSecret';
        !isSealedSecret && sh.rm('-rf', info.path);
    };

    allManifestsInfo.forEach(removeNonSealedSecrets);

    handleShellError(sh.rm('-rf', `${p.join(getMainBaseDir(), 'Pulumi.dev.yaml')}`));
    handleShellError(sh.exec("export PULUMI_CONFIG_PASSPHRASE='not-needed' && pulumi stack init --stack dev"));

    // Pulumi needs some environment variables set for generating deployments with image tag
    /* `export ${IMAGE_TAG_REACT_WEB}=tag-web export ${IMAGE_TAG_GRAPHQL_MONGO}=tag-mongo`
     */
    handleShellError(sh.exec(
        `
        ${getEnvVarsForScript(environment, imageTags)}
        export PULUMI_CONFIG_PASSPHRASE="not-needed"
        pulumi up --yes --skip-preview --stack dev
       `
    ));

    sh.echo(c.blueBright(`SYNC CRDS CODE`));
    // syncCrdsCode(environment);

    sh.rm('-rf', './login');
}

function syncCrdsCode(environment: Environment, allManifestsInfo: KubeObjectInfo[]) {
    const manifestsCrdsFiles = getKubeManifestsPaths({ kind: 'CustomResourceDefinition', environment, allManifestsInfo });
    const outDir = path.join(getMainBaseDir(), 'crds-generated');

    sh.exec(` crd2pulumi --nodejsPath ${outDir} ${manifestsCrdsFiles.join(' ')} --force`);
    sh.exec(`npx prettier --write ${getGeneratedCrdsCodeDir()}`);
}
