import sh from 'shelljs';
import { helmChartsInfo } from '../../src/shared/helmChartInfo.js';
import { getGeneratedCrdsCodeDir } from '../../src/shared/directoriesManager.js';
import chalk from 'chalk';
import yaml from 'yaml';

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
            const cmdCrd2pulumi = `crd2pulumi --nodejsPath ${outDir} - --force`;

            const renderedTemlate = sh.exec(cmdRenderTemplateResources, { silent: true });
            if (renderedTemlate.stderr) {
                throw new Error(
                    chalk.redBright(
                        `Problem rendering helm chart to kubernetes resources. Check that the chart name, repo and version are correct. Error: ${renderedTemlate.stderr}`
                    )
                );
            }

            // Try 1 for best effort. This does not work for linkerd-crd due to crd2pulumi cli not being
            // able to handle things like complex default values. So, Try 2 takes care of that.
            sh.exec(`${cmdRenderTemplateResources} | ${cmdCrd2pulumi}`, { silent: true });

            // Try 2.
            // Crd2pulumi is not yet able to handle some values e.g in linkerd-crds. This parser helps with transformation
            // to make it possible for crd2pulumi to handle
            // TODO: This and one later below can be removed when this issue is resolved: https://github.com/pulumi/crd2pulumi/issues/102
            yaml.parseAllDocuments(renderedTemlate.stdout).forEach((parsedKubeResource) => {
                sh.exec(`echo '${parsedKubeResource}' | ${cmdCrd2pulumi}`, { silent: true });
            });

            // Some helm charts e.g tikv/tidb don't include their crds into the chart.
            if (externalCrds) {
                externalCrds.forEach((crdUrl) => {
                    // Try1
                    sh.exec(`curl ${crdUrl} | ${cmdCrd2pulumi}`, { silent: true });

                    // Try2
                    const remoteCrd = sh.exec(`curl ${crdUrl}`).stdout;
                    yaml.parseAllDocuments(remoteCrd).forEach((parsedKubeResource) => {
                        sh.exec(`echo '${parsedKubeResource}' | ${cmdCrd2pulumi}`, { silent: true });
                    });
                });
            }
        });
    });
}
