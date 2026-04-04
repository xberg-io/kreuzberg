import { defineConfig } from "tsup";

export default defineConfig({
	entry: ["typescript/index.ts", "typescript/cli.ts", "typescript/errors.ts", "typescript/types.ts"],
	format: ["esm", "cjs"],
	bundle: true,
	dts: {
		compilerOptions: {
			skipLibCheck: true,
			skipDefaultLibCheck: true,
			ignoreDeprecations: "6.0",
		},
	},
	splitting: false,
	sourcemap: true,
	clean: true,
	shims: false,
	platform: "node",
	target: "node22",
	external: ["sharp", /\.node$/, /@kreuzberg\/node-.*/, "./index.js", "../index.js", "../../index.js"],
	esbuildOptions: (options) => {
		// Suppress direct eval warning - intentionally used for CJS/ESM compatibility
		options.logOverride = {
			...options.logOverride,
			"direct-eval": "silent",
		};
	},
});
