import c from 'chalk';
import fs from 'fs';
import inquirer from 'inquirer';
import sh, { ShellString } from 'shelljs';
import { Environment } from '../../resources/types/own-types';
import { ImageTags } from '../../resources/shared/validations';
import { namespaceSchema } from './../../resources/infrastructure/namespaces/util';
import { z } from 'zod';
import { getGeneratedEnvManifestsDir } from '../../resources/shared/manifestsDirectory';

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

export function getKubernetesSecretsPaths({ environment }: { environment: Environment }) {
    const environmentManifestsDir = getGeneratedEnvManifestsDir(environment);
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





type ObjectType =
    'Secret' |
    'Deployment' |
    'Service' |
    'Configmap' |
    'Pod' |
    'SealedSecret';

const kubernetesObjectInfo = z.object({
    // kind: z.union([z.literal("nane"), z.intersection([z.string(), z.object({z})])]),
    kind: z.string(),
    name: z.string(),
    namespace: namespaceSchema,
    path: z.string()

}).required()

// We override the object kind type since it's a nonexhasutive list
interface KubeObjectInfo extends Omit<z.infer<typeof kubernetesObjectInfo>, "kind"> { kind: ObjectType | (string & {}) }

export function getKubernetesManifestInfo({ environmentManifestsDir }: { environmentManifestsDir: string }): KubeObjectInfo[] {
    const manifestMatcher = "*ml"
    const allManifests = sh.exec(`find ${environmentManifestsDir} -name "${manifestMatcher}"`, {
        silent: true,
    });

    const allManifestsArray = allManifests.stdout
        .trim()
        .split('\n')
        .map((p) => {
            const info = JSON.parse(sh.exec(`
        cat ${p.trim()} | yq '{"kind": .kind, "name": .metadata.name, "namespace": .metadata.namespace}' -o json
        `).stdout);

            return kubernetesObjectInfo.parse(
                {
                    ...info,
                    path: p
                })
        }
        ) as KubeObjectInfo[];
    return allManifestsArray
}
