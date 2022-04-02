#!/usr/bin/env node

/* 
TODO: ADD INSTRUCTION ON HOW THIS WORKS
*/
import path from "path";
import fs from "fs";
import glob from "glob";
import util from "util";
import {
  getEnvironmentVariables,
  ImageTags,
  imageTagsObjectValidator,
} from "../resources/shared/validations";

// TODO: Use prompt to ask for which cluster this should be used with for the sealed secrets controller
// npm i inquirer
const globAsync = util.promisify(glob);

import z from "zod";
const MANIFESTS_DIR = path.join(__dirname, "..", "manifests");
const IMAGE_TAGS_FILES = path.join(MANIFESTS_DIR, "image-tags", "*");

export async function getImageTagsFromDir(): Promise<ImageTags> {
  const imageTagsPaths = await globAsync(IMAGE_TAGS_FILES, {
    dot: true,
  });

  const imageTagsList = imageTagsPaths.map((x) => {
    const imageTagKey = path.basename(x);
    const imageTagValue = fs.readFileSync(x, { encoding: "utf-8" });
    return [imageTagKey, imageTagValue];
  });

  const imageTagsObject = imageTagsObjectValidator.parse(Object.fromEntries(imageTagsList));

  return imageTagsObject;
}
