#!/usr/bin/env node
import fs from 'node:fs';
import glob from 'glob';
import path from 'node:path';
import util from 'node:util';
import { getMainBaseDir } from '../../src/resources/shared/directoriesManager.js';
import { ImageTags, imageTagsObjectValidator } from '../../src/resources/types/environmentVariables.js';


const globAsync = util.promisify(glob);

const MANIFESTS_DIR = getMainBaseDir();
const IMAGE_TAGS_FILES = path.join(MANIFESTS_DIR, 'imageTags', '*');

export async function getImageTagsFromDir(): Promise<ImageTags> {
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
