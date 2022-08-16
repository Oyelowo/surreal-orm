
import path from 'node:path';
import sh from 'shelljs';
import { getMainBaseDir } from '../src/resources/shared/directoriesManager.js';
import { ARGV_ENVIRONMENTS, tsConfigPath } from './utils/argv.js';
import { handleShellError } from './utils/shared.js';

function main() {
    const { environment } = ARGV_ENVIRONMENTS
    const baseDir = getMainBaseDir();
    const cloudManifestsDir = path.join(baseDir, 'generatedCloudInfra', environment);
    sh.mkdir('-p', cloudManifestsDir);
    // https://www.pulumi.com/docs/intro/concepts/state/#logging-into-the-local-filesystem-backend
    sh.exec(`pulumi login file://${cloudManifestsDir}`);

    handleShellError(
        sh.exec(
            `
        export PULUMI_CONFIG_PASSPHRASE="oyelowo"
        export PULUMI_NODEJS_TRANSPILE_ONLY=true
        export PULUMI_SKIP_CONFIRMATIONS=true
        export PULUMI_NODEJS_TSCONFIG_PATH=${tsConfigPath}
        export LINODE_TOKEN=c4ebdc329a825fd7b372e43f1de367dd2277aa39dc00536dfb9d63eb15b8d6c4
        pulumi up --yes --skip-preview --stack lke --cwd ${baseDir}
       `
        )
    );
}

main()