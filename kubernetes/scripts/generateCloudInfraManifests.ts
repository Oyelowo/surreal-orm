
import path from 'node:path';
import sh from 'shelljs';
import { ARGV_ENVIRONMENTS } from './generateManifestsCi';
import { tsConfigPath } from './utils/kubeObject/generateManifests';
import { handleShellError } from './utils/shared';

function main() {
    const { environment } = ARGV_ENVIRONMENTS
    const cloudManifestsDir = path.normalize(`generatedCloudInfra/${environment}`);
    sh.rm('-rf', `./${cloudManifestsDir}`);
    sh.mkdir(`./${cloudManifestsDir}`);

    // https://www.pulumi.com/docs/intro/concepts/state/#logging-into-the-local-filesystem-backend
    sh.exec(`pulumi login file://${cloudManifestsDir}`);
    handleShellError(
        sh.exec(
            `
        export PULUMI_CONFIG_PASSPHRASE="not-needed"
        export PULUMI_NODEJS_TRANSPILE_ONLY=true
        export PULUMI_SKIP_CONFIRMATIONS=true
        export PULUMI_NODEJS_TSCONFIG_PATH=${tsConfigPath}
        export LINODE_TOKEN=<token>
        pulumi up --yes --skip-preview --stack dev
       `
        )
    );
}

main()