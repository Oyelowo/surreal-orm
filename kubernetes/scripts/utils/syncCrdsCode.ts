import sh from 'shelljs';
import { helmChartsInfo } from '../../src/shared/helmChartInfo.js';
import { getGeneratedCrdsCodeDir } from '../../src/shared/directoriesManager.js';
import chalk from 'chalk';
import yaml from 'yaml';
import fs from 'node:fs';

export function syncCrdsCode() {
    const outDir = getGeneratedCrdsCodeDir();
    sh.rm('-rf', outDir);
    sh.exec(`mkdir -p '${outDir}'`, { silent: true });

    Object.entries(helmChartsInfo).forEach(([repoName, { repo, charts }]) => {
        sh.exec(`helm repo add ${repoName} ${repo}`, { silent: true });
        sh.exec(`helm repo update ${repoName}`, { silent: true });

        Object.values(charts).forEach(({ chart, version, externalCrds, skipCrdRender }) => {
            if (skipCrdRender === true) return;

            sh.echo(
                chalk.blueBright(`Syncing Crds from helm chart ${repoName}/${chart} version=${version} from ${repo}`)
            );
            const cmdRenderTemplateResources = `helm template ${chart}  --include-crds ${repoName}/${chart} --version ${version} --set installCRDs=true --set externalCA=true`;
            const renderedTemlate = sh.exec(cmdRenderTemplateResources, { silent: true });

            if (renderedTemlate.stderr) {
                throw new Error(
                    chalk.redBright`Problem rendering helm chart to kubernetes resources. Check that the chart name, repo and version are correct. Error: ${renderedTemlate.stderr}`
                );
            }

            const renderedFromHelmChart = yaml.parseAllDocuments(renderedTemlate.stdout);
            // Some helm charts e.g tikv/tidb don't include their crds into the chart.
            const renderedFromExternalCrds = externalCrds.flatMap((crdUrl) => {
                const rendered = sh.exec(`curl ${crdUrl}`, { silent: true });

                if (rendered.stderr) console.warn(chalk.yellowBright`${rendered.stderr}`);

                return yaml.parseAllDocuments(rendered.stdout);
            });

            const renderedCrds = [...renderedFromHelmChart, ...renderedFromExternalCrds].filter((t) =>
                t.toString().includes('kind: CustomResourceDefinition')
            );

            renderedCrds.forEach((parsedKubeResource, i) => {
                const data = yaml.parse(parsedKubeResource.toString(), (k, v) => {
                    // Crd2pulumi is not yet able to handle some values e.g in linkerd-crds. This parser helps with transformation
                    // to make it possible for crd2pulumi to handle
                    /* It appears to happen on fields where the default contains a nested value, like: status: default: observedGeneration: -1*/
                    // TODO: This can be removed when this issue is resolved: https://github.com/pulumi/crd2pulumi/issues/102
                    return typeof v === 'object' && k === 'default' ? undefined : v; // else return the value
                });

                fs.writeFile(`${outDir}/${repoName}${chart}${i}.yaml`, yaml.stringify(data), (err) => {
                    if (err) throw err;
                });
            });
        });
    });

    // const paths = sh.exec(`grep --include=\*.{yaml,yml} -Rnwl '${outDir}' -e 'kind: CustomResourceDefinition'`, { silent: true }).stdout
    // console.log("paths", paths)
    // // sh.exec(`crd2pulumi --nodejsPath ./xxx  ${outDir}/* --force`);
    // sh.exec(`crd2pulumi --nodejsPath ./xxx  ${paths.split("\n").join(" ")} --force`);
}
