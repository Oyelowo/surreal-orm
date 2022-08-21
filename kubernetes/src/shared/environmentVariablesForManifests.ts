import fs from 'node:fs';
import glob from 'glob';
import path from 'node:path';
import util from 'node:util';
import { getMainBaseDir } from './directoriesManager.js';
import { ServiceName, TServices } from '../types/ownTypes.js';
import * as z from 'zod';
import { appEnvironmentsSchema } from '../types/ownTypes.js';
import { SnakeCase } from 'type-fest';

const globAsync = util.promisify(glob);
const mainDir = getMainBaseDir();
const IMAGE_TAGS_FILES = path.join(mainDir, 'imageTags', '*');

async function getImageTagsFromDir(): Promise<ImageTags> {
    const imageTagsPaths = await globAsync(IMAGE_TAGS_FILES, {
        dot: true,
    });

    const imageTagsList = imageTagsPaths.map((x) => {
        const imageTagKey = path.basename(x);
        const imageTagValue = fs.readFileSync(x, { encoding: 'utf8' });
        return [imageTagKey, imageTagValue];
    });

    const imageTagsObject = imageTagsObjectValidator.parse(Object.fromEntries(imageTagsList));

    return imageTagsObject;
}

export const imageTags = await getImageTagsFromDir();

const imageTagsSchema: Record<Uppercase<`${TServices}__${SnakeCase<ServiceName>}__IMAGE_TAG`>, z.ZodString> = {
    SERVICES__GRAPHQL_MONGO__IMAGE_TAG: z.string().min(1),
    SERVICES__GRPC_MONGO__IMAGE_TAG: z.string().min(1),
    SERVICES__GRAPHQL_POSTGRES__IMAGE_TAG: z.string().min(1),
    SERVICES__REACT_WEB__IMAGE_TAG: z.string().min(1),
};

// This is provided fro, within the CI pipeline where the manifests are generated and pushed to the repo
export const imageTagsObjectValidator = z.object(imageTagsSchema);

export type ImageTags = z.infer<typeof imageTagsObjectValidator>;

const environmentVariablesValidator = z
    .object({
        ENVIRONMENT: appEnvironmentsSchema,
        // TEMPORARY_DIR: z.string().min(1).optional(),
    })
    .and(imageTagsObjectValidator);

export type EnvironmentVariables = z.infer<typeof environmentVariablesValidator>;

export const getEnvVarsForKubeManifests = () => environmentVariablesValidator.parse(process.env);
