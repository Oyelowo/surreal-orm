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
    getResourceManifestsPaths,
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
};

export async function generateManifests({ environment, imageTags }: GenerateManifestsProp) {
    const manifestsDirForEnv = getGeneratedEnvManifestsDir(environment);

    sh.echo(c.blueBright(`UPDATE CACHE TRACKER FILE`));
    // Create a cache tracker with a timestamp suffix
    // This helps the memoized/cached function that get all yaml manifest resources'
    // info/metatadata. The function uses all the paths of the yamls for
    // deciding if to cache, we want to create a new "hash" to bust the cache
    // if we ever generate updated manifefsts in between. This should come first.
    const cacheTrackerDir = path.join(manifestsDirForEnv, 'cache-tracker');
    sh.rm('-rf', cacheTrackerDir);
    sh.mkdir(cacheTrackerDir);
    const yamlManifestsCacheTracker = path.join(cacheTrackerDir, `${Date.now()}.yaml`);
    sh.touch(yamlManifestsCacheTracker);

    sh.exec('npm i');
    sh.rm('-rf', './login');
    sh.mkdir('./login');

    sh.exec('pulumi login file://login');

    sh.echo(c.blueBright(`DELETE EXISTING RESOURCES(except sealed secrets) at ${manifestsDirForEnv}`));
    const removeNonSealedSecrets = (info: KubeObjectInfo) => {
        const isSealedSecret = info.kind === 'SealedSecret';
        !isSealedSecret && sh.rm('-rf', info.path);
    };
    getAllKubeManifestsInfo(environment).forEach(removeNonSealedSecrets);

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
    syncCrdsCode(environment);

    sh.rm('-rf', './login');
}

function syncCrdsCode(environment: Environment) {
    const manifestsCrdsFiles = getResourceManifestsPaths({ kind: 'CustomResourceDefinition', environment });
    const outDir = path.join(getMainBaseDir(), 'crds-generated');

    sh.exec(` crd2pulumi --nodejsPath ${outDir} ${manifestsCrdsFiles.join(' ')} --force`);
    sh.exec(`npx prettier --write ${getGeneratedCrdsCodeDir()}`);
}
