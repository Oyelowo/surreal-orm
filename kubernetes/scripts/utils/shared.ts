import c from 'chalk';
import fs from 'node:fs';
import inquirer from 'inquirer';
import sh, { ShellString } from 'shelljs';
import { Environment } from '../../src/resources/types/ownTypes.js';
import { getEnvVarsForKubeManifestGenerator } from '../../src/resources/types/environmentVariables.js';

export function getEnvVarsForScript() {
    const env = getEnvVarsForKubeManifestGenerator();
    const imageEnvVarSetterForPulumi = Object.entries(env)
        .map(([k, v]) => `export ${k}=${v}`)
        .join(' ');
    return `
      ${imageEnvVarSetterForPulumi}
  `;
}

export function isFileEmpty(fileName: string, ignoreWhitespace = true): Promise<boolean> {
    return new Promise((resolve, reject) => {
        fs.readFile(fileName, (err, data) => {
            if (err) {
                reject(err);
                return;
            }

            resolve((!ignoreWhitespace && data.length === 0) || (ignoreWhitespace && /^\s*$/.test(String(data))));
        });
    });
}

export function handleShellError(shellCommand: ShellString) {
    if (shellCommand.stderr) {
        console.log(c.bgRedBright(shellCommand.stderr));
        sh.exit(-1);
    }
    return shellCommand;
}

export const ENVIRONMENTS_ALL: Environment[] = ['local', 'production', 'staging', 'development'];
export async function promptEnvironmentSelection() {
    const choices = ENVIRONMENTS_ALL.flatMap((env) => [env, new inquirer.Separator()]);

    const name = 'environment';
    const answers: Record<typeof name, Environment> = await inquirer.prompt([
        {
            type: 'list',
            name,
            message: c.greenBright('🆘Select the environment ‼️‼️‼️‼️'),
            choices,
            default: ENVIRONMENTS_ALL[0],
            pageSize: 20,
        } as const,
    ]);

    return answers;
}
