// type Repo = "bitnami" | "jetspack" | "linkerd"

// type BitnamiRepoCharts = {
//     repo: "https://charts.bitnami.com/bitnami";
//     certManager: {
//         chart: "cert-manager";
//         version: "0.4.17";
//     };
// }

// type JetspackRepoCharts = {
//     repo: "https://charts.jetstack.io";
//     certManager: {
//         chart: "cert-manager";
//         version: "1.8.0";
//     };
//     certManagerTrust: {
//         chart: "cert-manager-trust";
//         version: "0.1.1";
//     };
// }

// type LinkerdRepoCharts = {
//     repo: "https://helm.linkerd.io/stable",
//     linkerd2: {
//         chart: "linkerd2";
//         version: "2.11.2";
//     }
// }

// type Props = Record<`${Repo}Repo`, BitnamiRepoCharts | JetspackRepoCharts | LinkerdRepoCharts>

export const helmChartsInfo = {
    bitnamiRepo: {
        repo: "https://charts.bitnami.com/bitnami",
        certManager: {
            chart: "cert-manager",
            version: "0.5.0"
        },
        nginxIngress: {
            chart: "nginx-ingress-controller",
            version: "9.1.26",
        },
        argocd: {
            chart: "argo-cd",
            version: "3.1.16",
        }
    },
    jetspackRepo: {
        repo: "https://charts.jetstack.io",
        certManager: {
            chart: "cert-manager",
            version: "v1.8.0",
        },
        certManagerTrust: {
            chart: "cert-manager-trust",
            version: "v0.1.1"
        },

    },
    linkerdRepo: {
        repo: "https://helm.linkerd.io/stable",
        linkerd2: {
            chart: "linkerd2",
            version: "2.11.2",
        },
        linkerdViz: {
            chart: "linkerd-viz",
            version: "2.11.2",
        }
    }
} as const


// export const helmChartsMetadata = {
//     certManager: {
//         bitnami: {
//             repo: "https://charts.bitnami.com/bitnami",
//             chart: "cert-manager",
//             version: "0.4.17",
//         },
//         jetspack: {
//             repo: "https://charts.jetstack.io",
//             chart: "cert-manager",
//             version: "1.8.0",
//         },
//     },
//     certManagerTrust: {
//         bitnami: {
//             repo: "https://charts.bitnami.com/bitnami",
//             chart: "cert-manager",
//             version: "0.4.17",
//         },
//         jetspack: {
//             repo: "https://charts.jetstack.io",
//             chart: "cert-manager",
//             version: "1.8.0",
//         },
//     },
//     linked2: {
//         linkerd: {
//             repo: "https://helm.linkerd.io/stable",
//             chart: "linkerd2",
//             version: "2.11.2",
//         },
//     },
// } as const;

// // repo/chart
