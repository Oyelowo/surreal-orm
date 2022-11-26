import crds from "../../../generatedCrdsTs/index.js";
import { graphqlSurrealdbSettings } from "./settings.js";

export const graphqlSurrealDbUserLocationStream =
	new crds.jetstream.v1beta2.Stream("", {
		metadata: graphqlSurrealdbSettings.metadata,
		spec: {
			name: "user-locations",
			noAck: false,
			account: "",
			maxConsumers: 1000,
			maxAge: "",
			// maxMsgsPerSubject:
		},
	});
