import * as z from 'zod';
import { appEnvironmentsSchema } from '../types/ownTypes.js';

// CONSIDER: Move this into helpers
export const imageTagsObjectValidator = z.object({
    // This is provided fro, within the CI pipeline where the manifests are generated and pushed to the repo
    IMAGE_TAG_REACT_WEB: z.string().min(1),
    IMAGE_TAG_GRAPHQL_MONGO: z.string().min(1),
    IMAGE_TAG_GRPC_MONGO: z.string().min(1),
    IMAGE_TAG_GRAPHQL_POSTGRES: z.string().min(1),
});

export type ImageTags = z.infer<typeof imageTagsObjectValidator>;

const environmentVariablesValidator = z
    .object({
        ENVIRONMENT: appEnvironmentsSchema,
        TEMPORARY_DIR: z.string().min(1).optional(),
    })
    .and(imageTagsObjectValidator);

export type EnvironmentVariables = z.infer<typeof environmentVariablesValidator>;

export const getEnvironmentVariables = () => environmentVariablesValidator.parse(process.env);
