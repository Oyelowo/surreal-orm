import { AppConfigs } from "../shared/types";

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
    replicaCount: 2,
    host: "0.0.0.0",
    image: "oyelowo/react-web",
  },

  envVars: {
    APP_ENVIRONMENT: "development",
    APP_HOST: "0.0.0.0",
    APP_PORT: "3000",
    GITHUB_ID: "89c19374f7e7b5b35164",
    GITHUB_SECRET: "129488cc92e2d2f91e3a5a024086396c48c65339",
    GOOGLE_ID:
      "855174209543-6m0f088e55d3mevhnr8bs0qjap8j6g0g.apps.googleusercontent.com",
    GOOGLE_SECRET: "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",
    NEXTAUTH_URL: "http://localhost:8080",
  },
  metadata: {
    name: "react-web",
    namespace: "development",
  },
};
