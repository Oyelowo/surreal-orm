import sh from "shelljs";
import { KubeObject } from "./kubeObject.js";
import { expect, jest, test, describe } from "@jest/globals";
import { info } from "node:console";
import { MockSTDIN, stdin } from "mock-stdin";

// Key codes
const keys = {
	up: "\u001B\u005B\u0041",
	down: "\u001B\u005B\u0042",
	enter: "\u000D",
	space: "\u0020",
	a: "\u0041",
};
// helper function for timing
const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

jest
	.spyOn(KubeObject.prototype, "sealSecretValue")
	.mockImplementation(
		({ name, namespace, secretValue }) =>
			`lowo-test${name}${namespace}${"*".repeat(secretValue.length)}`,
	);

function deleteSealedSecrets() {
	new KubeObject("test").getOfAKind("SealedSecret").forEach((ss) => {
		sh.rm("-rf", ss.path);
	});
}

describe("KubeObject", () => {
	// Mock stdin so we can send messages to the CLI
	let io: MockSTDIN | undefined;
	afterAll(() => {
		io.restore();
		deleteSealedSecrets();
	});

	beforeAll(() => {
		io = stdin();
	});

	beforeEach(() => {
		deleteSealedSecrets();
	});

	test("Can sync resources", () => {
		const kubeInstance = new KubeObject("test");

		const inst = kubeInstance.getAll();
		expect(inst.length).toMatchSnapshot();

		const inst2 = kubeInstance.getOfAKind("Deployment");
		expect(inst2.length).toMatchSnapshot();

		info("Can get kube objects for a resource");
		const graphqlSurrealdb = kubeInstance.getForApp(
			"services/graphql-surrealdb",
		);
		expect(graphqlSurrealdb.length).toMatchSnapshot();

		const reactWeb = kubeInstance.getForApp("services/react-web");
		expect(reactWeb.length).toMatchSnapshot();

		const argocd = kubeInstance.getForApp("infrastructure/argocd");
		expect(argocd.length).toMatchSnapshot();

		const linkerd = kubeInstance.getForApp("infrastructure/linkerd");
		expect(linkerd.length).toMatchSnapshot();

		const certManager = kubeInstance.getForApp("infrastructure/cert-manager");
		expect(certManager.length).toMatchSnapshot();

		const nginxIngress = kubeInstance.getForApp("infrastructure/nginx-ingress");
		expect(nginxIngress.length).toMatchSnapshot();

		const namespaces = kubeInstance.getForApp("infrastructure/namespaces");
		expect(namespaces.length).toMatchSnapshot();
	});

	test("Can update sealed secrets", () => {
		const kubeInstance = new KubeObject("test");
		const sealedSecrets = kubeInstance.getOfAKind("SealedSecret");
		expect(sealedSecrets.length).toMatchSnapshot();

		kubeInstance.syncSealedSecrets();

		const sealedSecretsUpdated = kubeInstance.getOfAKind("SealedSecret");
		expect(sealedSecretsUpdated.length).toMatchSnapshot();
	});

	test("Can create sealed secrets from selected secrets", async () => {
		const sendKeystrokes = async () => {
			// Selection 1
			io.send(keys.down);
			io.send(keys.space);

			//  Selection 2
			io.send(keys.down);
			io.send(keys.space);

			//  Selection 3
			io.send(keys.down);
			io.send(keys.space);

			//  Selection 4
			io.send(keys.down);
			io.send(keys.down);
			io.send(keys.space);

			//  Selection 5
			io.send(keys.down);
			io.send(keys.space);

			// First Confirmation
			io.send(keys.enter);
			await delay(10);

			// Subselection for Selection 1
			io.send(keys.a);
			// io.send(keys.a, 'ascii')
			io.send(keys.enter);
			await delay(10);

			// Subselection for Selection 2
			io.send(keys.a);
			io.send(keys.enter);
			await delay(10);

			// Subselection for Selection 3
			io.send(keys.down);
			io.send(keys.space);

			io.send(keys.enter);
			await delay(10);

			// Subselection for Selection 4
			io.send(keys.down);
			io.send(keys.space);

			io.send(keys.enter);
			await delay(10);

			// Subselection for Selection 5
			io.send(keys.a);
			io.send(keys.enter);
			await delay(10);
		};
		setTimeout(() => sendKeystrokes().then(), 10);

		const kubeInstance = new KubeObject("test");
		await kubeInstance.syncSealedSecretsWithPrompt();

		const sealedecrets = kubeInstance.getOfAKind("SealedSecret");

		info(
			"Should have five selections cos we have updated only 5 sealed secrets",
		);
		expect(sealedecrets.length).toMatchSnapshot();
	});

	test("Can update sealed secrets after initial", async () => {
		const kubeInstance = new KubeObject("test");
		jest
			.spyOn(kubeInstance, "sealSecretValue")
			.mockImplementation(() => "inital-secrets");
		kubeInstance.syncSealedSecrets();

		const sealedSecrets = kubeInstance.getOfAKind("SealedSecret");
		const secrets = kubeInstance.getOfAKind("Secret");

		info("Should have 18 sealed secrets initially generated from 18 secrets");
		expect(secrets.length).toMatchSnapshot();
		expect(secrets).toHaveLength(18);

		expect(sealedSecrets.length).toMatchSnapshot();
		expect(sealedSecrets).toHaveLength(18);
		expect(
			Object.values(
				sealedSecrets.filter((ss) =>
					Object.values(ss.spec.encryptedData).includes("inital-secrets"),
				),
			).length,
		).toMatchSnapshot();

		const sendKeystrokes = async () => {
			// Select from the resources from which secrets will be selected
			/* Note: MAke sure space is odd number in case there is only one secret
 if the selection is even number, then the secret will be deselected the 2nd time */

			// Selection 1
			io.send(keys.space); // Select
			io.send(keys.down); // Bottom arrow

			//  Selection 2
			io.send(keys.space);
			io.send(keys.down);

			//  Selection 3
			io.send(keys.space);
			io.send(keys.down);
			io.send(keys.down);

			//  Selection 4
			io.send(keys.space);
			io.send(keys.down);
			io.send(keys.down);
			io.send(keys.down);

			//  Selection 5
			io.send(keys.space);

			//  Enter the secret selection phase
			io.send(keys.enter);
			await delay(10);

			// SUB-SELECTIONS
			// Subselection for Selection 1
			io.send(keys.a); // Select all for first resource's secrets
			// io.send(keys.a, 'ascii')
			io.send(keys.enter);
			await delay(10);

			// Subselection for Selection 2
			io.send(keys.a);
			io.send(keys.enter);
			await delay(10);

			// Subselection for Selection 3
			/* Note: MAke sure space is odd number in case there is only one secret
             if the selection is even number, then the secret will be deselected the 2nd time */
			io.send(keys.down);
			io.send(keys.space); // 1
			io.send(keys.down);
			io.send(keys.down);
			io.send(keys.space); // 2
			io.send(keys.down);
			io.send(keys.space); // 3
			io.send(keys.enter);
			await delay(10);

			// Subselection for Selection 4
			io.send(keys.space); // Select the first item
			io.send(keys.enter);
			await delay(10);

			// Subselection for Selection 5
			io.send(keys.a); // Selects all secrets for the last resource
			io.send(keys.enter);
			await delay(10);
		};
		setTimeout(() => sendKeystrokes().then(), 5);

		// ASSERT
		jest
			.spyOn(kubeInstance, "sealSecretValue")
			.mockImplementation(() => "updated-secrets");
		// Prompt user for selection of secrets to update
		await kubeInstance.syncSealedSecretsWithPrompt();
		const sealedSecretsSomeUpdated = kubeInstance.getOfAKind("SealedSecret");

		info(
			"Should update from 13 sealed secrets still, with specific secret data fields updated.",
		);
		expect(sealedSecretsSomeUpdated).toHaveLength(18);
		// 5 secrets have been updated
		expect(
			Object.values(
				sealedSecretsSomeUpdated.filter((ss) =>
					Object.values(ss.spec.encryptedData).includes("updated-secrets"),
				),
			),
		).toHaveLength(5);
	});
});
