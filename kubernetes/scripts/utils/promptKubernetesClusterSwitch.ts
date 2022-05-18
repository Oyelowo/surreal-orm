import chalk from 'chalk'
import inquirer from 'inquirer'
import sh from 'shelljs'
import { Environment } from '../../resources/shared/types/own-types'
// import { ARGV } from "./bootstrap";

/*
Prompt cluster selection
*/
export async function promptKubernetesClusterSwitch (environment: Environment) {
  const kubernetesContexts = sh.exec('kubectl config get-contexts --output=name', { silent: true })
  const choices = kubernetesContexts.stdout
    .trim()
    .split('\n')
    .flatMap((ctx) => [ctx, new inquirer.Separator()])

  const name = 'cluster'
  const answers: Record<typeof name, string> = await inquirer.prompt([
    {
      type: 'list',
      name,
      message: chalk.greenBright(`🆘Select the ${environment.toLocaleUpperCase()} cluster ‼️‼️‼️‼️`),
      choices,
      default: choices.find((s) => ['local', 'K3d', 'minikube'].includes(String(s))),
      pageSize: 20
    }
  ])

  const selectContext = sh.exec(`kubectl config use-context ${answers.cluster}`, { silent: true })
  sh.echo(chalk.greenBright(`${selectContext.stdout} 🎉`))
}
