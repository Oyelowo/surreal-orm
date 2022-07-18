import c from 'chalk';
import fs from 'fs';
import sh, { ShellString } from 'shelljs';
import { Environment } from '../../resources/types/own-types';
import { ImageTags } from '../../resources/shared/validations';

const ENVIRONMENT_KEY = 'ENVIRONMENT';
export function getEnvVarsForScript(environment: Environment, imageTags: ImageTags) {
    const imageEnvVarSetterForPulumi = Object.entries(imageTags)
        .map(([k, v]) => `export ${k}=${v}`)
        .join(' ');
    return `
      ${imageEnvVarSetterForPulumi} 
      export ${ENVIRONMENT_KEY}=${environment}  
  `;
}

export function getKubernetesSecretsPaths({ environmentManifestsDir }: { environmentManifestsDir: string }) {
    const manifestMatcher = "*ml"
    const allManifests = sh.exec(`find ${environmentManifestsDir} -name "${manifestMatcher}"`, {
        silent: true,
    });

    const allManifestsArray = allManifests.stdout
        .trim()
        .split('\n')
        .map((s) => s.trim());

    const kubernetesSecrets = allManifestsArray.filter((path) => {
        // Find Kubernetes Secret Objects i.e with type => Kind: Secret
        // This matches the space between the ":" and Secret
        let secret = sh.exec(`grep "^kind: *Secret$" ${path}`, { silent: true }).stdout?.trim();
        const isSecret = !!secret;
        // Filter out non-empty secrets. Which means the manifest is secret type
        isSecret && console.log(`Grabbing secret path: ${path} \n`)
        return isSecret;
    });

    return kubernetesSecrets;
}


export function isFileEmpty(fileName: string, ignoreWhitespace = true): Promise<boolean> {
    return new Promise((resolve, reject) => {
        fs.readFile(fileName, (err, data) => {
            if (err) {
                reject(err);
                return;
            }

            resolve((!ignoreWhitespace && data.length === 0) || (ignoreWhitespace && !!String(data).match(/^\s*$/)));
        });
    });
}

export function handleShellError(shellCommand: ShellString) {
    if (shellCommand.stderr) {
        console.log(c.bgRedBright(shellCommand.stderr));
        // process.exit(-1)
        sh.exit(-1);
    }
    return shellCommand;
}
