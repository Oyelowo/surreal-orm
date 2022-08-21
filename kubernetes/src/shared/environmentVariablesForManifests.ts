import path from 'node:path';
import { getMainBaseDir } from './directoriesManager.js';
import { ServiceName, TServices } from '../types/ownTypes.js';
import * as z from 'zod';
import { appEnvironmentsSchema } from '../types/ownTypes.js';
import { SnakeCase } from 'type-fest';
import _ from 'lodash';
import sh from 'shelljs';

export const imageTagsDefault: Record<Uppercase<`${TServices}__${SnakeCase<ServiceName>}__IMAGE_TAG`>, 'latest'> = {
    SERVICES__GRAPHQL_MONGO__IMAGE_TAG: 'latest',
    SERVICES__GRPC_MONGO__IMAGE_TAG: 'latest',
    SERVICES__GRAPHQL_POSTGRES__IMAGE_TAG: 'latest',
    SERVICES__REACT_WEB__IMAGE_TAG: 'latest',
};

const imageTagsSchema: Record<
    Uppercase<`${TServices}__${SnakeCase<ServiceName>}__IMAGE_TAG`>,
    z.ZodString
> = _.mapValues(imageTagsDefault, (_v) => z.string().min(1));

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

const mainDir = getMainBaseDir();
const IMAGE_TAGS_DIR = path.join(mainDir, 'imageTags');

async function getImageTagsFromDir(): Promise<ImageTags> {
    const imageTagsList: string[][] = Object.entries(imageTagsDefault).map(
        ([imageTagName, defaultImageTag]): string[] => {
            const resourceImageTagPath = path.join(IMAGE_TAGS_DIR, imageTagName);
            const imageTag = sh.cat(resourceImageTagPath).stdout.trim();
            // Never been set, set latest
            if (_.isEmpty(imageTag)) {
                sh.exec(`echo ${defaultImageTag} > ${resourceImageTagPath} `);
            }
            const imageTagValue = _.isEmpty(imageTag) ? 'latest' : imageTag;
            return [imageTagName, imageTagValue];
        }
    );

    return imageTagsObjectValidator.parse(Object.fromEntries(imageTagsList));
}

export const imageTags = await getImageTagsFromDir();
