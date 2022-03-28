import * as z from "zod";

const environmentVariablesValidator = z.object({
  // TAG_REACT_WEB: z.string().nonempty().or(z.undefined()),
  // This is provided fro, within the CI pipeline where the manifests are generated and pushed to the repo
  IMAGE_TAG_REACT_WEB: z.string().nonempty(),
  IMAGE_TAG_GRAPHQL_MONGO: z.string().nonempty(),
  IMAGE_TAG_GRPC_MONGO: z.string().nonempty(),
  IMAGE_TAG_GRAPHQL_POSTGRES: z.string().nonempty(),
});

export const environmentVariables = environmentVariablesValidator.parse(
  process.env
);
