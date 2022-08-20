import c from 'chalk';
import fs from 'node:fs';
import inquirer from 'inquirer';
import sh, { ShellString } from 'shelljs';
import yargs from 'yargs';
import { Environment } from '../../src/resources/types/ownTypes.js';
import { getEnvVarsForKubeManifestGenerator } from '../../src/resources/types/environmentVariables.js';

const env = getEnvVarsForKubeManifestGenerator();
const ENVIRONMENT_KEY: keyof Pick<typeof env, "ENVIRONMENT"> = "ENVIRONMENT"
// const envx: Extract<keyof typeof env, "ENVIRONMENT"> = "ENVIRONMENT"
export function getEnvVarsForScript({ environment }: { environment: Environment }) {

    const imageEnvVarSetterForPulumi = Object.entries(env)
        .map(([k, v]) => `export ${k}=${v}`)
        .join(' ');
    return `
      ${imageEnvVarSetterForPulumi}
      export ${ENVIRONMENT_KEY}=${environment}
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

;

export const ARGV_ENVIRONMENTS = yargs(process.argv.slice(2))
    .options({
        environment: {
            alias: 'e',
            choices: ENVIRONMENTS_ALL,
            describe: "The environment you're generating the manifests for.",
            demandOption: true,
        },
    })
    .parseSync();
