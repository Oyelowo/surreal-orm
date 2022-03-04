import { provider } from "./../shared/cluster";
import { ServiceDeployment } from "../shared/deployment";

// export * from "./postgresHAdb";
export * from "./postgres";

import { AppConfigs } from "../shared/types";
import { graphqlPostgresSettings } from "./settings";

export const graphqlPostgres = new ServiceDeployment(
  "graphql-postgres",
  graphqlPostgresSettings,
  { provider }
);
