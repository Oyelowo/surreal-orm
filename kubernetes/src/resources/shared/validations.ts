import * as z from 'zod';
import { appEnvironmentsSchema } from '../types/ownTypes.js';

// CONSIDER: Move this into helpers
export const imageTagsObjectValidator = z.object({

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
