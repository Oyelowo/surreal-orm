import { getGeneratedEnvManifestsDir } from '../../resources/shared/manifestsDirectory'
import sh from 'shelljs'
import { Environment } from '../../resources/shared/types/own-types'
import p from 'path'
import c from 'chalk'

import { getSecretManifestsPaths, getSecretPathsInfo } from './sealedSecrets'
import { getFilePathsThatMatch } from './shared'
import { SEALED_SECRETS_CONTROLLER_NAME } from './sealedSecrets'

/*
GENERATE BITNAMI'S SEALED SECRET FROM PLAIN SECRETS MANIFESTS GENERATED USING PULUMI.
These secrets are encrypted using the bitnami sealed secret controller running in the cluster
you are at present context
*/
export interface GenSealedSecretsProps {
    environment: Environment
}

// TODO: Prompt user to specify which apps secrets they're looking to update
export async function generateAllSealedSecrets({ environment }: GenSealedSecretsProps) {
    getSecretManifestsPaths(environment)?.forEach(generateSealedSecret)
}

function generateSealedSecret(unsealedSecretFilePath: string) {
    const { sealedSecretDir, sealedSecretFilePath } = getSecretPathsInfo({
        unsealedSecretFilePath,
    })

    sh.mkdir(sealedSecretDir)

    // TODO: Check the content of the file to confirm if it is actually a secret object
    sh.echo(c.blueBright(`Generating sealed secret ${unsealedSecretFilePath} \n to \n ${sealedSecretFilePath}`))

    // TODO: Should I delete old sealed secrets before creating new ones?
    const kubeSeal = sh.exec(
        `kubeseal --controller-name ${SEALED_SECRETS_CONTROLLER_NAME} < ${unsealedSecretFilePath} -o yaml >${sealedSecretFilePath}`,
        { silent: true }
    )

    sh.echo(c.greenBright(kubeSeal.stdout))
    if (kubeSeal.stderr) {
        sh.echo(`Error sealing secrets: ${c.redBright(kubeSeal.stderr)}`)
        sh.exit(1)
        return
    }

    sh.echo(c.greenBright('Successfully generated sealed secret at', unsealedSecretFilePath))
}
