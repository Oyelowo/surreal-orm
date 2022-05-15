import { promise } from "zod";
import { promptKubernetesClusterSwitch } from "./utils/promptKubernetesClusterSwitch";
import { promptEnvironmentSelection } from "./utils/sealedSecrets";



(async () => {
    console.log("edon happen1")
    const k = await promptEnvironmentSelection()
    console.log("edon happen2")
    await promptKubernetesClusterSwitch(k.environment)
    console.log("edon happen3")
    console.log("edon happen4")
})()

