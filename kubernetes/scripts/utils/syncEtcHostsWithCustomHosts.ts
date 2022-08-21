import { ingressHostsLocal } from '../../src/infrastructure/ingress/hosts.js';
import sh from 'shelljs';
import _ from 'lodash';
import chalk from 'chalk';

/**
 For syncing all local ETC hosts on macbook
 */
export function syncEtcHostsWithCustomHosts() {
    const hostsFileContent = sh.cat('/etc/hosts');

    if (hostsFileContent.stderr) {
        console.error(chalk.blueBright(`Something went wrong. Error: ${hostsFileContent.stderr}`));
    }

    const existingIpHosts = hostsFileContent.stdout
        .split('\n')
        .filter((l) => !(l.startsWith('#') || _.isEmpty(l.trim())))
        .map((l) => l.split(/\s+/).slice(0, 2) as [string, string]);

    const LOCAL_IP = '127.0.0.1';
    function updateIpHost(customHost: string) {
        const found = existingIpHosts.find(
            ([existingIp, existingHost]) => existingIp === LOCAL_IP && existingHost === customHost
        );
        if (found) return;

        console.info(chalk.blueBright(`Updating Ip and host: ${LOCAL_IP} - ${customHost}`));
        const ipHost = `${LOCAL_IP}\t${customHost}`;

        sh.exec(`echo "${ipHost}" | sudo tee -a /etc/hosts`);
    }

    ingressHostsLocal.forEach(updateIpHost);
}

// syncEtcHostsWithCustomHosts();
