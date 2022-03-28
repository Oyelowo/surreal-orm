import * as z from "zod";

const environmentVariablesValidator = z.object({
  // TAG_REACT_WEB: z.string().nonempty().or(z.undefined()),
  TAG_REACT_WEB: z.string().nonempty(),
  TAG_GRAPHQL_MONGO: z.string().nonempty(),
  TAG_GRPC_MONGO: z.string().nonempty(),
  TAG_GRAPHQL_POSTGRES: z.string().nonempty(),
});

export const environmentVariables = environmentVariablesValidator.parse(
  process.env
);
