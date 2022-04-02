#!/usr/bin/env node

/* 
TODO: ADD INSTRUCTION ON HOW THIS WORKS
*/
import { ArgumentTypes } from "./../../typescript/apps/web/utils/typescript";

import path from "path";
import fs from "fs";
import glob from "glob";
import prompt from "prompt";
import sh from "shelljs";
import util from "util";
import yargs from "yargs/yargs";
import c from "chalk";

import { Environment } from "./../resources/shared/types/own-types";
import { getEnvironmentVariables, imageTagsObjectValidator } from "../resources/shared/validations";
import {
  clearUnsealedInputTsSecretFilesContents,
  setupUnsealedSecretFiles,
} from "../secretsManagement/setupSecrets";
import { getManifestsOutputDirectory } from "../resources/shared";
// TODO: Use prompt to ask for which cluster this should be used with for the sealed secrets controller
// npm i inquirer
type EnvName = keyof typeof getEnvironmentVariables;
const globAsync = util.promisify(glob);

const promptGetAsync = util.promisify(prompt.get);
import z from "zod";
const MANIFESTS_DIR = path.join(__dirname, "..", "manifests");
const SEALED_SECRETS_BASE_DIR = path.join(MANIFESTS_DIR, "sealed-secrets");

const IMAGE_TAGS_DIR = path.join(MANIFESTS_DIR, "image-tags");
const IMAGE_TAGS_FILES = path.join(MANIFESTS_DIR, "image-tags", "*");

export async function getImageTagsFromDir(): Promise<z.infer<typeof imageTagsObjectValidator>> {
  const imageTagsPaths = await globAsync(IMAGE_TAGS_FILES, {
    dot: true,
  });
  //   const imageTags = sh.exec("ls" + IMAGE_TAGS_DIR);
  const imageTagsList = imageTagsPaths.map((x) => {
    const imageTagKey = path.basename(x);
    const imageTagValue = fs.readFileSync(x, { encoding: "utf-8" });
    return [imageTagKey, imageTagValue];
  });

  const imageTagsObject = imageTagsObjectValidator.parse(Object.fromEntries(imageTagsList));

  return imageTagsObject;
}

getImageTagsFromDir();
