/** @type {import('ts-jest/dist/types').InitialOptionsTsJest} */
module.exports = {
	transform: {
		"^.+\\.(t|j)sx?$": ["@swc/jest"],
	},
	extensionsToTreatAsEsm: [".ts", ".tsx"],
	moduleNameMapper: {
		"^(\\.{1,2}/.*)\\.js$": "$1",
		"#(.*)": "<rootDir>/node_modules/$1",
	},
	// ts-jest config
	// preset: 'ts-jest/presets/default-esm', // or other ESM presets
	// globals: {
	//     'ts-jest': {
	//         useESM: true,
	//         diagnostics: false,
	//     },
	// },
	// testEnvironment: 'node',
};
