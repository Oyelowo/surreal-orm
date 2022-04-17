import { graphqlMongoSettings } from "../graphql-mongo/settings";
import { namespaceNames } from "../../shared/namespaces";
import { AppConfigs } from "../../shared/types/own-types";
import { getEnvironmentVariables } from "../../shared/validations";
import { getFQDNFromSettings } from "../../shared/helpers";

export const reactWebSettings: AppConfigs<"react-web", "doesNotHaveDb", "applications"> = {
  kubeConfig: {
    requestMemory: "70Mi",
    requestCpu: "100m",
    limitMemory: "200Mi",
    limitCpu: "100m",
    replicaCount: 2,
    host: "0.0.0.0",
    image: `ghcr.io/oyelowo/react-web:${getEnvironmentVariables().IMAGE_TAG_REACT_WEB}`,
  },

  envVars: {
    APP_ENVIRONMENT: getEnvironmentVariables().ENVIRONMENT,
    APP_HOST: "0.0.0.0",
    APP_PORT: "3000",
    // TODO: Add from environment variables
    GITHUB_CLIENT_ID: "89c19374f7e7b5b35164",
    GITHUB_CLIENT_SECRET: "129488cc92e2d2f91e3a5a024086396c48c65339",
    GOOGLE_CLIENT_ID: "855174209543-6m0f088e55d3mevhnr8bs0qjap8j6g0g.apps.googleusercontent.com",
    GOOGLE_CLIENT_SECRET: "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",
    NEXTAUTH_URL: "http://localhost:8080",
    GRAPHQL_MONGO_URL: getFQDNFromSettings(graphqlMongoSettings), // Get Url mongoFQDN
  },
  metadata: {
    name: "react-web",
    namespace: namespaceNames.applications,
  },
};
