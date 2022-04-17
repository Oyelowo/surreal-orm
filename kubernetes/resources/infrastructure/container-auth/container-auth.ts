import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";

const aa = new kx.Secret(
  "wew",
  {
    stringData: {},
  },
  {}
);

const l = {
  auths: {
    "ghcr.io/oyelowo": {
      username: "oyelowo",
      password: "token",
      email: "oyelowooyedayo@gmail.com",
      auth: "b3llbG93bzpnaHBfdjNZSG1ob2VTUE1MUFR6R084alk2NDNNa1oxYnhBNERqS3R6",
    },
  },
};

const p = {
  auths: {
    "ghcr.io": {
      username: "oyelowo",
      password: "token",
      email: "oyelowooyedayo@gmail.com",
      auth: "b3llbG93bzpnaHBfdjNZSG1ob2VTUE1MUFR6R084alk2NDNNa1oxYnhBNERqS3R6",
    },
  },
};
/* 
{"auths":{"ghcr.io/oyelowo":{"username":"oyelowo","password":"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","email":"oyelowooyedayo@gmail.com","auth":"b3llbG93bzpnaHBfdjNZSG1ob2VTUE1MUFR6R084alk2NDNNa1oxYnhBNERqS3R6"}}}%   
*/
