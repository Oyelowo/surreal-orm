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
import { getEnvVarsForScript, handleShellError } from './shared';
import path from 'path';
/*
GENERATE ALL KUBERNETES MANIFESTS USING PULUMI
*/

type GenerateManifestsProp = {
    environment: Environment;
    imageTags: ImageTags;
}

export async function generateManifests({ environment, imageTags }: GenerateManifestsProp) {
    const manifestsDirForEnv = getGeneratedEnvManifestsDir(environment);
    // Create a cache tracker with a timestamp suffix
    // This helps the memoized/cached function that get all yaml manifest resources'
    // info/metatadata. The function uses all the paths of the yamls for
    // deciding if to cache, we want to create a new "hash" to bust the cache
    // if we ever generate updated manifefsts in between. This should come first.
    const cacheTrackerDir = path.join(manifestsDirForEnv, "cache-tracker")
    sh.rm("-rf", cacheTrackerDir)
    sh.mkdir(cacheTrackerDir)
    const yamlManifestsCacheTracker = path.join(cacheTrackerDir, `${Date.now()}.yaml`)
    sh.touch(yamlManifestsCacheTracker)

    sh.exec('npm i');
    sh.rm('-rf', './login');
    sh.mkdir('./login');

    sh.exec('pulumi login file://login');

    sh.echo(c.blueBright(`First Delete old resources for" ${environment} at ${manifestsDirForEnv}`));
    // TODO: Could use the helper function that gets all manifests and just filter out
    // SealedSecrets but delete other resource.
    // getAllManifests().filter(({kind}) => Kind !== "SealedSecret").forEach((f) => sh.rm('-rf', f.trim())) ...
    const getManifestsWithinDirName = (dirName: '1-manifest' | '0-crd') =>
        sh
            .exec(`find ${manifestsDirForEnv} -type d -name "${dirName}"`, {
                silent: true,
            })
            .stdout.trim()
            .split('\n');

    const manifestsNonCrds = getManifestsWithinDirName('1-manifest');
    const manifestsCrds = getManifestsWithinDirName('0-crd');
    manifestsNonCrds.concat(manifestsCrds).forEach((f) => sh.rm('-rf', f.trim()));

    handleShellError(sh.rm('-rf', `${p.join(getMainBaseDir(), 'Pulumi.dev.yaml')}`));
    handleShellError(sh.exec("export PULUMI_CONFIG_PASSPHRASE='not-needed' && pulumi stack init --stack dev"));

    // Pulumi needs some environment variables set for generating deployments with image tag
    /* `export ${IMAGE_TAG_REACT_WEB}=tag-web export ${IMAGE_TAG_GRAPHQL_MONGO}=tag-mongo`
     */
    const exec = sh.exec(
        `
        ${getEnvVarsForScript(environment, imageTags)}
        export PULUMI_CONFIG_PASSPHRASE="not-needed"
        pulumi up --yes --skip-preview --stack dev
       `
    );

    if (exec.stderr) {
        throw new Error(c.redBright(`Something went wrong with pulumi. Error: ${exec.stderr}`));
    }

    const updatedCrds = getManifestsWithinDirName('0-crd');
    syncCrdsCode(updatedCrds);

    sh.rm('-rf', './login');
}

function syncCrdsCode(updatedCrds: string[]) {
    const manifestsCrdsFilesUpdated = updatedCrds.flatMap((dir) => {
        const crds = sh.ls(dir).stdout.trim().split('\n');
        const isNotEmptyFile = (f: string) => Boolean(f.trim());
        const getFullPathForFile = (f: string) => p.join(dir, f.trim());

        return crds.filter(isNotEmptyFile).map(getFullPathForFile);
    });

    sh.exec(
        ` crd2pulumi --nodejsPath ${getMainBaseDir()}/crds-generated ${manifestsCrdsFilesUpdated.join(' ')} --force`
    );

    getGeneratedCrdsCodeDir();
    sh.exec(`npx prettier --write ${getGeneratedCrdsCodeDir()}`);
}
