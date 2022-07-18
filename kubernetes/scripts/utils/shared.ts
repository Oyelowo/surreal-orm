import c from 'chalk';
import fs from 'fs';
import inquirer from 'inquirer';
import sh, { ShellString } from 'shelljs';
import { Environment } from '../../resources/types/own-types';
import { ImageTags } from '../../resources/shared/validations';
import { namespaceSchema } from './../../resources/infrastructure/namespaces/util';
import { z } from 'zod';
import { getGeneratedEnvManifestsDir } from '../../resources/shared/manifestsDirectory';
import _ from 'lodash';

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

type ResourceType = 'Secret' | 'Deployment' | 'Service' | 'Configmap' | 'Pod' | 'SealedSecret' | (string & {});

const kubernetesResourceInfo = z.object({
    kind: z.string().nullable(),
    name: z.string().nullable(),
    namespace: namespaceSchema.nullable(),
    path: z.string().nullable(),
});

// We override the object kind type since it's a nonexhasutive list
interface KubeObjectInfo extends z.infer<typeof kubernetesResourceInfo> {
    kind: ResourceType;
}

function getAllManifestsPaths({ environment }: { environment: Environment }) {
    const environmentManifestsDir = getGeneratedEnvManifestsDir(environment);
    const manifestMatcher = '*ml';
    const allManifests = sh
        .exec(`find ${environmentManifestsDir} -name "${manifestMatcher}"`, {
            silent: true,
        })
        .stdout.trim()
        .split('\n')
        .map((p) => p.trim());
    return allManifests;
}

const exec = (cmd: string) => sh.exec(cmd, { silent: true }).stdout;

const getInfoFromManifests = _.memoize(
    (manifestsPaths: string[]) => {
        return manifestsPaths.map((p, i) => {
            console.log('Extracting info from manifest', i);
            const info = JSON.parse(
                exec(
                    `cat ${p.trim()} | yq '{"kind": .kind, "name": .metadata.name, "namespace": .metadata.namespace}' -o json`
                )
            );
            // let's mutate to make it a bit faster and should be okay since we only do it here
            info.path = p;
            console.log('Extracted info from', info);
            return kubernetesResourceInfo.parse(info);
        }) as KubeObjectInfo[];
    },
    // We are concatenating all the path names to get a stable memoization key
    // we could also JSON.stringify(paths)
    (paths) => paths.join('')
);

export const getAllKubeManifestsInfo = (environment: Environment) => {
    const m = getAllManifestsPaths({ environment });
    return getInfoFromManifests(m);
};

type InfoProps = {
    resourceType: ResourceType;
    environment: Environment;
};

export const getKubeResourceTypeInfo = ({ resourceType, environment }: InfoProps) => {
    return getAllKubeManifestsInfo(environment).filter(({ kind }) => kind === resourceType);
};
