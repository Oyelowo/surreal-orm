import crds from "../../../generatedCrdsTs/index.js";
import { graphqlSurrealdbSettings } from "./settings.js";

// For example purpose only
export const graphqlSurrealDbNatsClusterExample =
	new crds.nats.v1alpha2.NatsCluster("", {
		metadata: graphqlSurrealdbSettings.metadata,
		spec: {
			// maxMsgsPerSubject:
		},
	});
