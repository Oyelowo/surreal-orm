import sh from "shelljs";
import path from "path";

const forenv = path.join(__dirname, "..", "manifests", "generated", "production");
const secrets = sh.exec(`find ${forenv} -name "secret-*ml"`, { silent: true });
// const secrets = sh.exec(`find . -name "secret-*ml"`, { silent: true });

// console.log(secrets.stdout.trim().split("\n").map((s) => s.trim()));
const sealedSecretsForEnv = secrets.stdout
  .trim()
  .split("\n")
  .map((s) => s.trim());

for (const ss of sealedSecretsForEnv) {
  console.log(path.basename(ss));
  console.log(path.dirname(ss));
}
