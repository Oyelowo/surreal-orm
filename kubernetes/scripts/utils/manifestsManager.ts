import sh from 'shelljs';
import _ from "lodash";
import { z } from "zod";
import { namespaceSchema } from "../../resources/infrastructure/namespaces/util";
import { getGeneratedEnvManifestsDir, getResourceAbsolutePath } from "../../resources/shared/manifestsDirectory";
import { ResourceName, Environment } from "../../resources/types/own-types";
import { handleShellError } from './shared';


type ResourceKind =
    | 'Secret'
    | 'Deployment'
    | 'Service'
    | 'Configmap'
    | 'Pod'
    | 'SealedSecret'
    | 'CustomResourceDefinition';

// An app resource can comprise of multiple kubernetes manifests
type Props = {
    resourceName: ResourceName;
    // environment: Environment;
    allManifestsInfo: KubeObjectInfo[];
}

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

type InfoProps = {
    kind: ResourceKind;
    allManifestsInfo: KubeObjectInfo[];
};


export class ManifestsManager {
    #allManifestsInfo: KubeObjectInfo[];

    constructor(private environment: Environment) {
        this.#allManifestsInfo = this.syncManifestsInfo().getAllKubeManifestsInfo()
    }


    getAppResourceManifestsInfo = ({
        resourceName,
        allManifestsInfo,
    }: Props): KubeObjectInfo[] => {
        const envDir = getResourceAbsolutePath(resourceName, this.environment);
        // const manifests = getManifestsWithinDir(envDir);
        // return getInfoFromManifests(manifests);
        return allManifestsInfo.filter((m) => {
            const manifestIsWithinDir = (demarcator: '/' | '\\') => m.path.startsWith(`${envDir}${demarcator}`);
            return manifestIsWithinDir('/') || manifestIsWithinDir('\\');
        });
    };

    getAllKubeManifestsInfo = () => {
        return this.#allManifestsInfo;
    }

    /** Extract information from all the manifests for an environment(local, staging etc)  */
    syncManifestsInfo = () => {
        const envDir = getGeneratedEnvManifestsDir(this.environment);
        const manifestsPaths = this.#getManifestsPathWithinDir(envDir);
        const exec = (cmd: string) => handleShellError(sh.exec(cmd, { silent: true })).stdout;

        this.#allManifestsInfo = manifestsPaths.reduce<KubeObjectInfo[]>((acc, path, i) => {
            if (!path) return acc;
            console.log('Extracting info from manifest', i);
            const info = JSON.parse(exec(`cat ${path.trim()} | yq '.' -o json`));

            if (_.isEmpty(info)) return acc;
            // let's mutate to make it a bit faster and should be okay since we only do it here
            info.path = path;
            const updatedPath = kubernetesResourceInfo.parse(info) as KubeObjectInfo;

            acc.push(updatedPath);
            return acc;
        }, []);
        return this
    }

    /** Gets all the yaml manifests for an environment(local, staging etc)  */
    #getManifestsPathWithinDir = (environmentManifestsDir: string): string[] => {
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

    /** Get information for all specific kind of kubernetes object (e.g Deployment, Secrets)  */
    getKubeObjectManifestsInfo = ({ kind, allManifestsInfo }: InfoProps): KubeObjectInfo[] => {
        return allManifestsInfo.filter((info) => info.kind === kind);
    };

    // getKubeManifestsPaths = ({ kind, allManifestsInfo }: InfoProps): string[] => {

    //     return this.getKubeObjectManifestsInfo({
    //         kind,
    //         allManifestsInfo
    //     }).map(({ path }) => path);
    // }



}