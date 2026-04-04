import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		globals: true,
		environment: "node",
		pool: "threads",
		poolOptions: {
			threads: {
				singleThread: true,
			},
		},
		include: ["src/**/*.{test,spec}.ts", "../tests/**/*.{test,spec}.ts"],
		testTimeout: 30000,
		hookTimeout: 10000,
	},
});
