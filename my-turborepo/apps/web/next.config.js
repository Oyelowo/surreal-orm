const withTM = require("next-transpile-modules")(["echarts", "zrender", "ui"]);

/** @type {import('next').NextConfig} */
module.exports = withTM({
  reactStrictMode: true,

});


// const withTM = require("next-transpile-modules")(["echarts"]);
// const withTM = require("next-transpile-modules")(["echarts", "zrender"]);
// const withPlugins = require("next-compose-plugins");

// /** @type {import('next').NextConfig} */
// const nextConfig = {,
//   reactStrictMode: true,
//   webpack: config => {
//     // Unset client-side javascript that only works server-side
//     config.resolve.fallback = { fs: false, module: false, path: false, os: false };
//     return config;
//   },
//   experimental: {
//     externalDir: true,
//   },
// };

// module.exports = withPlugins([withTM], nextConfig);
