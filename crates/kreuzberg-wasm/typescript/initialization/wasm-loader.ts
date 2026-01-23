/**
 * WASM Module Loader
 *
 * Handles WASM module loading, initialization, and state management.
 * Provides a clean interface for loading the Kreuzberg WASM module
 * with support for concurrent initialization calls.
 */

import { wrapWasmError } from "../adapters/wasm-adapter.js";
import { hasWasm, isBrowser, isNode } from "../runtime.js";
import { initializePdfiumAsync } from "./pdfium-loader.js";

/**
 * Load WASM binary from file system in Node.js environment.
 * Returns undefined in browser environments (fetch will be used instead).
 */
async function loadWasmBinaryForNode(): Promise<Uint8Array | undefined> {
	if (!isNode()) {
		return undefined;
	}

	try {
		// Dynamic import to avoid bundling Node.js modules
		const fs = await import(/* @vite-ignore */ "node:fs/promises");
		const path = await import(/* @vite-ignore */ "node:path");
		const url = await import(/* @vite-ignore */ "node:url");

		// Resolve the WASM file path relative to this module
		// The module is in dist/initialization/wasm-loader.js
		// The WASM file is in dist/pkg/kreuzberg_wasm_bg.wasm
		const __dirname = path.dirname(url.fileURLToPath(import.meta.url));
		const wasmPath = path.join(__dirname, "..", "pkg", "kreuzberg_wasm_bg.wasm");

		const wasmBuffer = await fs.readFile(wasmPath);
		return new Uint8Array(wasmBuffer);
	} catch {
		// Fall back to fetch-based loading if file system access fails
		return undefined;
	}
}

import {
	getInitializationError,
	getInitializationPromise,
	getWasmModule,
	isInitialized,
	type ModuleInfo,
	setInitializationError,
	setInitializationPromise,
	setInitialized,
	setWasmModule,
	type WasmModule,
} from "./state.js";

export type { WasmModule, ModuleInfo };

/**
 * Get the loaded WASM module
 *
 * @returns The WASM module instance or null if not loaded
 * @internal
 */
export { getWasmModule };

/**
 * Check if WASM module is initialized
 *
 * @returns True if WASM module is initialized, false otherwise
 */
export { isInitialized };

/**
 * Get initialization error if module failed to load
 *
 * @returns The error that occurred during initialization, or null if no error
 * @internal
 */
export { getInitializationError };

/**
 * Get WASM module version
 *
 * @throws {Error} If WASM module is not initialized
 * @returns The version string of the WASM module
 */
export function getVersion(): string {
	if (!isInitialized()) {
		throw new Error("WASM module not initialized. Call initWasm() first.");
	}

	const wasmModule = getWasmModule();
	if (!wasmModule) {
		throw new Error("WASM module not loaded. Call initWasm() first.");
	}

	return wasmModule.version();
}

/**
 * Initialize the WASM module
 *
 * This function must be called once before using any extraction functions.
 * It loads and initializes the WASM module in the current runtime environment,
 * automatically selecting the appropriate WASM variant for the detected runtime.
 *
 * Multiple calls to initWasm() are safe and will return immediately if already initialized.
 *
 * @throws {Error} If WASM module fails to load or is not supported in the current environment
 *
 * @example Basic Usage
 * ```typescript
 * import { initWasm } from '@kreuzberg/wasm';
 *
 * async function main() {
 *   await initWasm();
 *   // Now you can use extraction functions
 * }
 *
 * main().catch(console.error);
 * ```
 *
 * @example With Error Handling
 * ```typescript
 * import { initWasm, getWasmCapabilities } from '@kreuzberg/wasm';
 *
 * async function initializeKreuzberg() {
 *   const caps = getWasmCapabilities();
 *   if (!caps.hasWasm) {
 *     throw new Error('WebAssembly is not supported in this environment');
 *   }
 *
 *   try {
 *     await initWasm();
 *     console.log('Kreuzberg initialized successfully');
 *   } catch (error) {
 *     console.error('Failed to initialize Kreuzberg:', error);
 *     throw error;
 *   }
 * }
 * ```
 */
export async function initWasm(): Promise<void> {
	if (isInitialized()) {
		return;
	}

	let currentPromise = getInitializationPromise();
	if (currentPromise) {
		return currentPromise;
	}

	currentPromise = (async () => {
		try {
			if (!hasWasm()) {
				throw new Error("WebAssembly is not supported in this environment");
			}

			let wasmModule: unknown;
			// Use const variables to make imports dynamic and bypass TypeScript's static module resolution.
			// This allows typecheck to pass when the WASM module hasn't been built yet (e.g., in CI).
			// Use URL-based resolution for cross-platform compatibility (especially Windows).
			const baseUrl = new URL(import.meta.url);
			const pkgUrl = new URL("../pkg/kreuzberg_wasm.js", baseUrl).href;
			const fallbackUrl = new URL("./kreuzberg_wasm.js", baseUrl).href;
			try {
				wasmModule = await import(/* @vite-ignore */ pkgUrl);
			} catch {
				wasmModule = await import(/* @vite-ignore */ fallbackUrl);
			}
			const loadedModule = wasmModule as unknown as WasmModule;
			setWasmModule(loadedModule);

			if (loadedModule && typeof loadedModule.default === "function") {
				// In Node.js, load WASM binary from file system to avoid fetch issues
				// In browsers/Workers, the default() function uses fetch with import.meta.url
				const wasmBinary = await loadWasmBinaryForNode();
				if (wasmBinary) {
					await loadedModule.default(wasmBinary);
				} else {
					await loadedModule.default();
				}
			}

			if (isBrowser() && loadedModule && typeof loadedModule.initialize_pdfium_render === "function") {
				initializePdfiumAsync(loadedModule).catch((error) => {
					console.warn("PDFium auto-initialization failed (PDF extraction disabled):", error);
				});
			}

			setInitialized(true);
			setInitializationError(null);
		} catch (error) {
			setInitializationError(error instanceof Error ? error : new Error(String(error)));
			throw wrapWasmError(error, "initializing Kreuzberg WASM module");
		}
	})();

	setInitializationPromise(currentPromise);
	return currentPromise;
}
