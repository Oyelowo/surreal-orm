import path from 'path';
import { ResourceName } from './../../resources/types/own-types';
import c from 'chalk';
import fs from 'fs';
import inquirer from 'inquirer';
import sh, { ShellString } from 'shelljs';
import { Environment } from '../../resources/types/own-types';
import { ImageTags } from '../../resources/shared/validations';
import { namespaceSchema } from './../../resources/infrastructure/namespaces/util';
import { z } from 'zod';
import { getGeneratedEnvManifestsDir, getResourceAbsolutePath } from '../../resources/shared/manifestsDirectory';
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

type ResourceKind =
    | 'Secret'
    | 'Deployment'
    | 'Service'
    | 'Configmap'
    | 'Pod'
    | 'SealedSecret'
    | 'CustomResourceDefinition';

const kubernetesResourceInfo = z.object({
    kind: z.string(),
    apiVersion: z.string(),
    type: z.string().optional(),
    path: z.string(),
    metadata: z.object({
        name: z.string(),
        // CRDS have namespace as null
        namespace: namespaceSchema.optional(),
        annotations: z.record(z.string()).transform((p) => p),
    }),
    spec: z
        .object({
            encryptedData: z.record(z.string().nullable()).optional(), // For sealed secrets
            // CRDS have namespace as null
            template: z.any().optional(), //Dont care about this yet
        })
        .optional(),
    data: z.record(z.string().nullable()).optional(),
    stringData: z.record(z.string().nullable()).optional(),
});

type kubernetesResourceInfoZod = z.infer<typeof kubernetesResourceInfo>;
export interface KubeObjectInfo extends kubernetesResourceInfoZod {
    // We override the object kind type since it's a nonexhasutive list
    // We also want to allow allow other string types here
    kind: ResourceKind;
    // kind: ResourceKind | (string & {});
}

function getManifestsPathWithinDir(environmentManifestsDir: string): string[] {
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

const getInfoFromManifests = (manifestsPaths: string[]): KubeObjectInfo[] => {
    return manifestsPaths.reduce<KubeObjectInfo[]>((acc, path, i) => {
        if (!path) return acc;
        console.log('Extracting info from manifest', i);

        const info = JSON.parse(exec(`cat ${path.trim()} | yq '.' -o json`));
        if (_.isEmpty(info)) return acc;

        // let's mutate to make it a bit faster and should be okay since we only do it here
        info.path = path;

        const updatedPath = kubernetesResourceInfo.parse(info) as KubeObjectInfo;
        console.log('Extracted info from', updatedPath.path);

        acc.push(updatedPath);
        return acc;
    }, []);
};

export const getAllKubeManifestsInfo = (environment: Environment): KubeObjectInfo[] => {
    const envDir = getGeneratedEnvManifestsDir(environment);
    const paths = getManifestsPathWithinDir(envDir);
    return getInfoFromManifests(paths);
};

// An app resource can comprise of multiple kubernetes manifests
type Props = {
    resourceName: ResourceName;
    environment: Environment;
    allManifestsInfo: KubeObjectInfo[];
}
export const getAppResourceManifestsInfo = ({
    resourceName,
    environment,
    allManifestsInfo,
}: Props): KubeObjectInfo[] => {
    const envDir = getResourceAbsolutePath(resourceName, environment);
    // const manifests = getManifestsWithinDir(envDir);
    // return getInfoFromManifests(manifests);
    return allManifestsInfo.filter((m) => {
        const manifestIsWithinDir = (demarcator: '/' | '\\') => m.path.startsWith(`${envDir}${demarcator}`);
        return manifestIsWithinDir('/') || manifestIsWithinDir('\\');
    });
};

type InfoProps = {
    kind: ResourceKind;
    allManifestsInfo: KubeObjectInfo[];
};

export const getKubeManifestsInfo = ({ kind, allManifestsInfo }: InfoProps): KubeObjectInfo[] => {
    return allManifestsInfo.filter((info) => info.kind === kind);
};

export function getKubeManifestsPaths({ kind, allManifestsInfo }: InfoProps): string[] {
    const filterTypeSafely = (f: KubeObjectInfo) => (f.path ? [f.path] : []);

    return getKubeManifestsInfo({
        kind,
        allManifestsInfo
    }).flatMap(filterTypeSafely);
}
