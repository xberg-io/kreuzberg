import { defineWorkersConfig } from "@cloudflare/vitest-pool-workers/config";
import { resolve, dirname } from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export default defineWorkersConfig({
	test: {
		globals: true,
		poolOptions: {
			workers: {
				main: "./tests/index.ts",
				wrangler: {
					configPath: "./wrangler.toml",
				},
			},
		},
		testTimeout: 60000,
		env: {
			KREUZBERG_WORKSPACE_ROOT: resolve(__dirname, "../.."),
		},
	},
});
