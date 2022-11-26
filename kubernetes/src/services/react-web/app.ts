import { ServiceDeployment } from "../../shared/deployment.js";
import { reactWebSettings } from "./settings.js";

export const reactWeb = new ServiceDeployment("react-web", reactWebSettings);
