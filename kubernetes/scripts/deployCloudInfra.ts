import path from 'node:path';
import sh from 'shelljs';
import yargs from 'yargs';
import { getMainBaseDir } from '../src/resources/shared/directoriesManager.js';
import { ARGV_ENVIRONMENTS, tsConfigPath } from './utils/argv.js';
import { handleShellError } from './utils/shared.js';

export const Argv = yargs(process.argv.slice(2))
    .options({
        'skip-preview': {
            alias: 'sp',
            boolean: true,
            describe: 'Skip showing preview. Useful for CI(Continuous Integration) pipeline.',
            // demandOption: true,
        },
        LINODE_TOKEN: {
            alias: 'lt',
            string: true,
            describe: 'Skip showing preview. Useful for CI(Continuous Integration) pipeline.',
            demandOption: true,
        },
        'pulumi-passphrase': {
            alias: 'pp',
            string: true,
            describe: 'Skip showing preview. Useful for CI(Continuous Integration) pipeline.',
            demandOption: true,
        },
    })
    .parseSync();

function main() {
    const { environment } = ARGV_ENVIRONMENTS;

    const baseDir = getMainBaseDir();
    const cloudManifestsDir = path.join(baseDir, 'generatedCloudInfra');
    sh.mkdir('-p', cloudManifestsDir);
    // https://www.pulumi.com/docs/intro/concepts/state/#logging-into-the-local-filesystem-backend
    sh.exec(`pulumi login file://${cloudManifestsDir}`);

    try {
        handleShellError(sh.rm('-rf', `${path.join(baseDir, `Pulumi.${environment}.yaml`)}`));
        handleShellError(
            sh.exec(`export PULUMI_CONFIG_PASSPHRASE='not-needed' && pulumi stack init --stack ${environment}`)
        );
    } catch {
        console.log(`Already created`);
    }

    handleShellError(
        sh.exec(
            `
        export PULUMI_CONFIG_PASSPHRASE="not-needed"
        export PULUMI_NODEJS_TRANSPILE_ONLY=true
        export PULUMI_SKIP_CONFIRMATIONS=true
        export PULUMI_NODEJS_TSCONFIG_PATH=${tsConfigPath}
        export LINODE_TOKEN=${Argv.LINODE_TOKEN}
        pulumi up --yes --skip-preview=${Argv.skipPreview} --stack ${environment}
       `
        )
    );
}

main();
