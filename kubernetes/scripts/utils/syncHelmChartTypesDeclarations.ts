import { getHelmChartTypesDir } from '../../resources/shared/manifestsDirectory';
import { helmChartsInfo } from '../../resources/shared/helmChartInfo';
import chalk from 'chalk';
import sh from 'shelljs';
import JsonToTS from 'json-to-ts';
import _ from 'lodash';

export function syncHelmChartTypesDeclarations() {
    const helmChartsDir = getHelmChartTypesDir();
    sh.exec(`rm -rf ${helmChartsDir}`);
    sh.exec(`mkdir -p ${helmChartsDir}`);

    Object.entries(helmChartsInfo).map(([repoName, repoValues]) => {
        const { repo: repoUrl } = repoValues;
        sh.echo(chalk.blueBright(`Syncing helm chart - ${repoName} from ${repoUrl}`));

        sh.exec(`helm repo add ${repoName} ${repoUrl}`);
        sh.exec(`helm repo update ${repoName}`);

        Object.values(repoValues.charts).forEach(({ chart, version }) => {
            let { stdout: valuesJson, stderr } = sh.exec(
                `helm show values ${repoName}/${chart} --version ${version} | yq -o=json -I=0`,
                {
                    silent: true,
                }
            );

            if (stderr) {
                throw new Error(chalk.redBright(`Problem happened. Error: ${stderr}`));
            }

            const typeFileName = _.camelCase(`${chart}${_.capitalize(repoName)}`);

            let tsDec = JsonToTS(JSON.parse(valuesJson), {
                rootName: `I${_.capitalize(typeFileName)}`,
            })
                .map((typeInterface, i) => {
                    return i == 0 ? `export ${typeInterface}` : typeInterface;
                })
                .join('\n');

            sh.exec(`echo ${JSON.stringify(tsDec)} > ${helmChartsDir}/${typeFileName}.ts`);
        });
    });
}
