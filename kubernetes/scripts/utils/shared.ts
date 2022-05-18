import c from 'chalk';
import fs from 'fs';
import sh, { ShellString } from 'shelljs';
import { Environment } from '../../resources/shared/types/own-types';
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

export function getFilePathsThatMatch({ contextDir, pattern }: { contextDir: string; pattern: string }) {
    const UNSEALED_SECRETS_MANIFESTS_FOR_ENV = sh.exec(`find ${contextDir} -name "${pattern}"`, {
        silent: true,
    });
    const unsealedSecretsFilePathsForEnv = UNSEALED_SECRETS_MANIFESTS_FOR_ENV.stdout
        .trim()
        .split('\n')
        .map((s) => s.trim());
    return unsealedSecretsFilePathsForEnv;
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
