
type charts = "certManager"
interface Props {
    bitnami: {
        repoUrl: "https://charts.bitnami.com/bitnami",
        certManager: {
            name: "cert-manager",
            version: "0.4.17",
        }
    }
}

// export const helmChartsMetadata: Props = {
//     bitnami: {
//         repoUrl: "https://charts.bitnami.com/bitnami",
//         certManager: {
//             name: "cert-manager",
//             version: "0.4.17"
//         }
//     }
// }
export const helmChartsMetadata = {
    certManager: {
        bitnami: {
            repoUrl: "https://charts.bitnami.com/bitnami",
            name: "cert-manager",
            version: "0.4.17",
        },
        jetspack: {
            repoUrl: "https://charts.jetstack.io",
            name: "cert-manager",
            version: "1.8.0",
        },
    }
} as const