import { defineConfig } from "tsup";

export default defineConfig({
	entry: [
		"src/index.ts",
		// config-mapping module
		"src/config-mapping/index.ts",
		"src/config-mapping/types.ts",
		"src/config-mapping/field-mappers.ts",
		"src/config-mapping/build-config.ts",
		// assertions module
		"src/assertions/index.ts",
		"src/assertions/types.ts",
		"src/assertions/factory.ts",
		"src/assertions/vitest-adapter.ts",
		"src/assertions/deno-adapter.ts",
		// fixtures module
		"src/fixtures/index.ts",
		"src/fixtures/types.ts",
		"src/fixtures/skip-handler.ts",
		// paths module
		"src/paths/index.ts",
		"src/paths/workspace.ts",
	],
	format: ["esm", "cjs"],
	bundle: false,
	dts: {
		compilerOptions: {
			skipLibCheck: true,
			skipDefaultLibCheck: true,
		},
	},
	splitting: false,
	sourcemap: true,
	clean: true,
	shims: false,
	platform: "neutral",
});
