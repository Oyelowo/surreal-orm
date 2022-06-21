import { getHelmChartTypesDir } from './../resources/shared/manifestsDirectory';
import { helmChartsInfo } from './../resources/shared/helmChartInfo';
import chalk from 'chalk';
import inquirer from 'inquirer';
import sh from 'shelljs';
import fs from "fs";
import z from "zod";
import JsonToTS from "json-to-ts";
// const { json2ts } = require('json-ts');

import { compile, compileFromFile } from 'json-schema-to-typescript'
import _ from 'lodash';
helmChartsInfo
// helm repo add sealed-secrets https://bitnami-labs.github.io/sealed-secrets
//  helm show values bitnami/mongodb --version 12

const k = z


const helmChartsDir = getHelmChartTypesDir();
sh.exec(`rm -rf ${helmChartsDir}`)
sh.exec(`mkdir -p ${helmChartsDir}`)
Object.entries(helmChartsInfo).map(([repoName, repoValues]) => {
    const { repo: repoUrl } = repoValues;
    // console.log("[repoName, repoValues]", [repoName, repoValues]);
    sh.exec(`helm repo add ${repoName} ${repoUrl}`);

    // const getChart = (chartName: string) => `${repoKey}/${chartName}`;
    // type k = typeof repoValues.charts[keyof typeof repoValues.charts];
    // console.log("Object.values(repoValues.charts)", Object.values(repoValues.charts))
    Object.values(repoValues.charts).forEach(({ chart, version }) => {
        // console.log("[chart, version]", [chart, version]);
        let valuesJson = sh.exec(`helm show values ${repoName}/${chart} --version ${version} | yq -o=json -I=0`, { silent: true }).stdout;
        // let valuesJson = sh.exec(`${valuesYaml} | yq -o=json -I=0`);
        // console.log("valuesJson", valuesJson)
        // compile(JSON.stringify(valuesJson), "InterM").then(ts => {
        //     console.log("ts", ts)
        // })
        const typeFileName = _.camelCase(`${chart}${_.capitalize(repoName)}`);

        let tsDec = JsonToTS(JSON.parse(valuesJson), {
            rootName: `I${_.capitalize(typeFileName)}`,

        }).map((typeInterface, i) => {
            // console.log("typeInterface", i, typeInterface)
            return i==0 ? `export ${typeInterface}` : typeInterface
            // sh.exec(`echo ${typeInterface} > kubernetes/resources/types/helmCharts/${chart}${repoName}`)
        }).join("\n");
        // console.log("tsDec", tsDec)
        // console.log("NEWWWWW",)
        // sh.exec(`touch -r ${helmChartsDir}/${chart}${repoName}`)

        sh.exec(`echo ${JSON.stringify(tsDec)} > ${helmChartsDir}/${typeFileName}.ts`)

    });
})


// compile from file
// compileFromFile('foo.json')
//     .then(ts => fs.writeFileSync('foo.d.ts', ts))

// // or, compile a JS object
// let mySchema = {
//     properties: [...]
// }
// compile(mySchema, 'MySchema')
//     .then(ts => ...)