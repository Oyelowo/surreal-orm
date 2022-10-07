import sh from 'shelljs';
import { helmChartsInfo } from '../../src/shared/helmChartInfo.js';
import { getGeneratedCrdsCodeDir } from '../../src/shared/directoriesManager.js';
// import { TKubeObject } from './kubeObject/kubeObject.js';
import chalk from 'chalk';
import yaml from 'yaml';
import z from 'zod';

// export function syncCrdsCode(crdKubeObjects: TKubeObject<'CustomResourceDefinition'>[]) {
//     const manifestsCrdsFiles = crdKubeObjects.map(({ path }) => path);
//     const outDir = getGeneratedCrdsCodeDir();
//     sh.mkdir('-p', outDir);

//     sh.exec(`crd2pulumi --nodejsPath ${outDir} ${manifestsCrdsFiles.join(' ')} --force`);
// }

export function syncCrdsCode() {
    // const helmChartsDir = getHelmChartTypesDir();
    const outDir = getGeneratedCrdsCodeDir();
    sh.rm("-rf", outDir);
    sh.mkdir('-p', outDir);

    Object.entries(helmChartsInfo).forEach(([repoName, repoValues]) => {
        const { repo: repoUrl, charts } = repoValues;

        sh.exec(`helm repo add ${repoName} ${repoUrl}`);
        sh.exec(`helm repo update ${repoName}`);

        Object.values(charts).forEach((ch) => {
            const { chart, version, externalCrds } = z
                .object({
                    chart: z.string(),
                    version: z.string(),
                    externalCrds: z.string().array().optional(),
                })
                .parse(ch);

            sh.echo(
                chalk.blueBright(`Syncing Crds from helm chart ${repoName}/${chart} version=${version} from ${repoUrl}`)
            );
            // const chartInfo = `${repoName}/${chart} --version ${version}`
            const renderTemplateResources = `helm template  --include-crds ${repoName}/${chart} --version ${version} --set installCRDs=true --set externalCA=true`;
            const crd2pulumi = `crd2pulumi --nodejsPath ${outDir}  - --force`;
            // if (!valuesYaml.includes("kind: CustomResourceDefinition")) {
            //     return
            // }
            // console.log("ccxxxxxx", repoName)

            // if (stderr) {
            //     console.warn(
            //         chalk.redBright(`Problem happened while rendering crds from helm file. Error: ${stderr}`)
            //     );
            // }


       

            const template = sh.exec(`${renderTemplateResources} | ${crd2pulumi}`);

            if (template.stderr) {
            // linkerd-crds fail due to https://github.com/pulumi/crd2pulumi/issues/102
            // Use this to retry to circumvent that
            const temp = sh.exec(renderTemplateResources).stdout;
            yaml.parseAllDocuments(temp).forEach((node) => {
                sh.exec(`echo '${node}' | ${crd2pulumi}`);
            });
            }

            if (externalCrds) {
                externalCrds.forEach((crdUrl) => {
                    const template = sh.exec(`curl ${crdUrl} | ${crd2pulumi}`);
                    if (template.stderr) {
                        // linkerd-crds fail due to https://github.com/pulumi/crd2pulumi/issues/102
                        // Use this to retry to circumvent that
                        const ch = sh.exec(`curl ${crdUrl}`).stdout;
                        yaml.parseAllDocuments(ch).forEach((node) => {
                            sh.exec(`echo '${node}' | ${crd2pulumi}`);
                        });
                    }
                });
            }
        });
    });
    // sh.exec(`npx prettier --write ${outDir}`);
}

// function removeTroublesomeKey(json: string): object {
//     // TODO: This can be removed when this issue is resolved
//     // https://github.com/pulumi/crd2pulumi/issues/102
//     // See: https://github.com/pulumi/crd2pulumi/issues/68
//     // https://github.com/pulumi/crd2pulumi/issues/68#issuecomment-1185164731
//     const res = yaml.parse(json, (k, v) => {
//         /* It appears to happen on fields where the default contains a nested value, like: status: default: observedGeneration: -1
//          It specifically happens for linkerd-crds with complex default values e.g {default: {type: "something"}}*/
//         return (typeof v === 'object' && k === "default")
//             ? undefined : v // else return the value
//     });

//     return res
// }
