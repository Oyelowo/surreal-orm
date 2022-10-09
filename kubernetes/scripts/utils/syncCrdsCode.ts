import sh from 'shelljs';
import { helmChartsInfo } from '../../src/shared/helmChartInfo.js';
import { getGeneratedCrdsCodeDir } from '../../src/shared/directoriesManager.js';
import chalk from 'chalk';
import yaml from 'yaml';
import fs from 'node:fs';
import path from 'node:path';
import waitOn from 'wait-on';

export async function syncCrdsCode() {
    const outDir = getGeneratedCrdsCodeDir();
    sh.rm('-rf', outDir);
    const crdPathName = '@oyelowo-crds';
    const tempCrdDir = path.join(outDir, crdPathName);
    sh.exec(`mkdir -p '${tempCrdDir}'`, { silent: true });

    sh.exec(`echo '${crdPathName}' >> ${path.join(outDir, '.gitignore')}`);

    const crdFilesPaths: string[] = [];

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

                const path = `${tempCrdDir}/${repoName}${chart}${i}.yaml`;
                crdFilesPaths.push(path);
                fs.writeFile(path, yaml.stringify(data), (err) => {
                    if (err) throw err;
                });
            });
        });
    });

    try {
        await waitOn({
            resources: crdFilesPaths,
            delay: 1000, // initial delay in ms, default 0
            interval: 100, // poll interval in ms, default 250ms
        });
        // once here, all resources are available
        sh.exec(`crd2pulumi --nodejsPath ${outDir}  ${tempCrdDir}/* --force`);
    } catch (error) {
        throw new Error(chalk.redBright`Problem generating crd codes. Error: ${error}`);
    }

    const typesPaths = path.join(outDir, 'types');
    sh.echo(`Sanitizing generated pulumi types within the path - ${typesPaths}.`);
    // TODO: Can remove after this resolved: https://github.com/pulumi/crd2pulumi/issues/101
    const fileMatcher = '*ts';
    sh.exec(`find ${typesPaths} -name "${fileMatcher}"`, { silent: true })
        .trim()
        .split('\n')
        .forEach((path) => {
            const data = fs.readFileSync(path, 'utf8');
            const sanitized = sanitizePulumiTypeDefinitions({ data });
            fs.writeFileSync(path, sanitized, 'utf8');
        });

    sh.exec(`rm -rf ${tempCrdDir}`);

    sh.echo(chalk.blueBright`Crd code generation done`);
}

export function sanitizePulumiTypeDefinitions({ data }: { data: string }): string {
    const replacer = (origChar: string) => origChar.split('-').join('');
    return (
        data
            // Wrap quote around the key with `?` coming after the quota
            // auto-scaler?: pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusAuto-ScalerArgs>;
            // to
            // "auto-scaler"?: pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusAuto-ScalerArgs>;
            .replace(/([a-z]+-.*[a-z]-*)(\?)?:/g, '"$1"$2:')
            // Remove the hyphen in the value here
            // "auto-scaler"?: pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusAuto-ScalerArgs>;
            // to
            // "auto-scaler"?: pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusAutoScalerArgs>;
            .replace(/(:.*)(-)(.*;)/g, replacer)
            // Removes hyphen from interface definition e.g
            // export interface TidbClusterStatusAuto-ScalerArgs {
            // to
            // export interface TidbClusterStatusAutoScalerArgs {
            .replace(/(interface.*)(-)(.*{)/g, replacer)
    );
}
