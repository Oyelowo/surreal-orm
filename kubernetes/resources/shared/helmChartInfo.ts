
type charts = "certManager"
interface Props {
    bitnami: {
        repo: "https://charts.bitnami.com/bitnami",
        certManager: {
            chart: "cert-manager",
            version: "0.4.17",
        }
    }
}

// export const helmChartsMetadata: Props = {
//     bitnami: {
//         repo: "https://charts.bitnami.com/bitnami",
//         certManager: {
//             chart: "cert-manager",
//             version: "0.4.17"
//         }
//     }
// }
export const helmChartsMetadata = {
    certManager: {
        bitnami: {
            repo: "https://charts.bitnami.com/bitnami",
            chart: "cert-manager",
            version: "0.4.17",
        },
        jetspack: {
            repo: "https://charts.jetstack.io",
            chart: "cert-manager",
            version: "1.8.0",
        },
    }
} as const