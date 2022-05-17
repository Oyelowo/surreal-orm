import sh from 'shelljs';
// import { getMerged } from "./merge";
import { setupPlainSecretTSFiles, getContent, Secrets } from "./setupSecrets";


try {
    setupPlainSecretTSFiles()
    // sh.exec("npx ts-node ./scripts/secretsManagement/merge.ts")
} catch (error) {
    // const p = getContent("staging", getMerged("staging"))
    // const k: Secrets = getMerged("staging")
    // sh.exec(`echo ${p} > ./.secrets/staging`)
    // sh.exec("npx ts-node ./scripts/secretsManagement/merge.ts")
}

// // getMerged("staging")

// const k = sh.exec(`cat `).stdout
// const k = sh.exec(`[ -e ./scripts/secretsManagement/merge.ts ]`).stdout

// console.log("dfd", k)