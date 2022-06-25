import c from 'chalk';
import p from 'path';
import sh from 'shelljs';
import { getGeneratedCrdsCodeDir, getGeneratedEnvManifestsDir, getMainBaseDir } from '../../resources/shared/manifestsDirectory';
import { ImageTags } from '../../resources/shared/validations';
import { GenSealedSecretsProps } from './generateAllSealedSecrets';
import { getEnvVarsForScript, handleShellError } from './shared';
/*
GENERATE ALL KUBERNETES MANIFESTS USING PULUMI
*/
interface GenerateManifestsProps extends Pick<GenSealedSecretsProps, 'environment'> {
    imageTags: ImageTags;
}

export async function generateManifests({ environment, imageTags }: GenerateManifestsProps) {
    const manifestsDirForEnv = getGeneratedEnvManifestsDir(environment);
    sh.exec('npm i');
    sh.rm('-rf', './login');
    sh.mkdir('./login');

    sh.exec('pulumi login file://login');

    sh.echo(c.blueBright(`First Delete old resources for" ${environment} at ${manifestsDirForEnv}`));

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
        throw new Error(c.redBright(exec.stderr));
    }

    const updatedCrds = getManifestsWithinDirName("0-crd");
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

    getGeneratedCrdsCodeDir()
    sh.exec(`npx prettier --write ${getGeneratedCrdsCodeDir()}`)
}

