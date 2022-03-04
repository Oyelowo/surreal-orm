import { AppConfigs } from '../shared/types';

export const reactWebSettings: AppConfigs<
  "react-web",
  "doesNotHaveDb",
  "development"
> = {
  kubeConfig: {
    requestMemory: "70Mi",
    requestCpu: "100m",
    limitMemory: "200Mi",
    limitCpu: "100m",
    host: "0.0.0.0",
    image: "oyelowo/react-web",
  },

  envVars: {
    APP_ENVIRONMENT: "development",
    APP_HOST: "0.0.0.0",
    APP_PORT: "3000",
  },
  metadata: {
    name: "react-web",
    namespace: "development",
  },
};
