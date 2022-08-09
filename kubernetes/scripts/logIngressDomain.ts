import { getIngressUrl } from '../src/resources/infrastructure/ingress/hosts.js';

import sh from 'shelljs';

import yargs from 'yargs';
import { ENVIRONMENTS_ALL } from './utils/shared.js';

export const ARGV = yargs(process.argv.slice(2))
    .options({
        environment: {
            alias: 'e',
            choices: ENVIRONMENTS_ALL,
            describe: 'environment',
            demandOption: true,
        },
    })
    .parseSync();

sh.exec(`echo ${getIngressUrl({ environment: ARGV.environment })}`);
