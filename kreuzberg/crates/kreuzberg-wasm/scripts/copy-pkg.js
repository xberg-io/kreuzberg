#!/usr/bin/env node
/**
 * Post-build script to copy pkg directory to dist, copy PDFium Emscripten
 * module from Cargo build output, and fix import paths.
 */

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const pkg = path.join(__dirname, "..", "pkg");
const dist = path.join(__dirname, "..", "dist");
const distPkg = path.join(dist, "pkg");

// Step 1: Copy wasm-pack output (pkg/) to dist/pkg/
if (fs.existsSync(pkg)) {
	fs.cpSync(pkg, distPkg, { recursive: true, force: true });
	console.log("Copied pkg directory to dist/pkg");

	// Remove .gitignore files created by wasm-pack to prevent npm from excluding WASM binaries
	const gitignorePath = path.join(distPkg, ".gitignore");
	if (fs.existsSync(gitignorePath)) {
		fs.unlinkSync(gitignorePath);
		console.log("Removed .gitignore from dist/pkg to allow npm publishing");
	}
} else {
	console.warn("pkg directory not found");
	process.exit(1);
}

// Step 2: Copy PDFium Emscripten module from Cargo build output to dist/
// During wasm-pack build, Cargo downloads PDFium and places the Emscripten
// module at: target/wasm32-unknown-unknown/release/build/kreuzberg-{hash}/out/pdfium/release/node/
// The hash varies per build, so we search for it.
const targetDir = path.join(__dirname, "..", "..", "..", "target");
const wasmBuildDir = path.join(targetDir, "wasm32-unknown-unknown", "release", "build");

/**
 * Find the PDFium Emscripten module directory in the Cargo build output.
 * Returns the path to the directory containing pdfium.js and pdfium.wasm,
 * or null if not found.
 */
function findPdfiumBuildDir() {
	if (!fs.existsSync(wasmBuildDir)) {
		return null;
	}

	const entries = fs.readdirSync(wasmBuildDir);
	// Look for kreuzberg-* directories that contain PDFium output
	// Pick the most recently modified one in case there are multiple build hashes
	let bestDir = null;
	let bestMtime = 0;

	for (const entry of entries) {
		if (!entry.startsWith("kreuzberg-")) continue;
		const pdfiumNodeDir = path.join(wasmBuildDir, entry, "out", "pdfium", "release", "node");
		const pdfiumJs = path.join(pdfiumNodeDir, "pdfium.js");
		if (fs.existsSync(pdfiumJs)) {
			const stat = fs.statSync(pdfiumJs);
			if (stat.mtimeMs > bestMtime) {
				bestMtime = stat.mtimeMs;
				bestDir = pdfiumNodeDir;
			}
		}
	}

	return bestDir;
}

const pdfiumDir = findPdfiumBuildDir();
if (pdfiumDir) {
	// Use the ESM variant (pdfium.esm.js) which has `export default PDFiumModule`
	// and works with dynamic import() in Deno, browsers, and Node.js ESM.
	// The non-ESM pdfium.js uses `var PDFiumModule = ...` which doesn't export anything.
	const pdfiumEsmJs = path.join(pdfiumDir, "pdfium.esm.js");
	const pdfiumEsmWasm = path.join(pdfiumDir, "pdfium.esm.wasm");

	if (fs.existsSync(pdfiumEsmJs)) {
		// Copy pdfium.esm.js as dist/pdfium.js (the name the loader imports)
		let pdfiumContent = fs.readFileSync(pdfiumEsmJs, "utf-8");

		// Fix Deno compatibility: Emscripten uses bare `import("module")` which Deno
		// doesn't support. Deno requires the `node:` prefix for Node.js built-in modules.
		pdfiumContent = pdfiumContent.replace(/import\("module"\)/g, 'import("node:module")');

		fs.writeFileSync(path.join(dist, "pdfium.js"), pdfiumContent);
		const size = Buffer.byteLength(pdfiumContent);
		console.log(`Copied PDFium Emscripten ESM module to dist/pdfium.js (${(size / 1024).toFixed(0)} KB)`);
	} else {
		console.warn("pdfium.esm.js not found in build output, PDF extraction may not work");
	}

	if (fs.existsSync(pdfiumEsmWasm)) {
		// The ESM module references "pdfium.esm.wasm" internally, so keep the original name
		fs.copyFileSync(pdfiumEsmWasm, path.join(dist, "pdfium.esm.wasm"));
		const size = fs.statSync(pdfiumEsmWasm).size;
		console.log(`Copied PDFium WASM binary to dist/pdfium.esm.wasm (${(size / 1024 / 1024).toFixed(1)} MB)`);
	} else {
		console.warn("pdfium.esm.wasm not found in build output, PDF extraction may not work");
	}
} else {
	console.warn("PDFium build output not found in target/wasm32-unknown-unknown/release/build/");
	console.warn("PDF extraction will not be available. Run wasm-pack build first.");
}

// Step 3: Fix import paths in bundled output
const files = [path.join(dist, "index.js"), path.join(dist, "index.cjs")];

for (const file of files) {
	if (fs.existsSync(file)) {
		let content = fs.readFileSync(file, "utf-8");
		const original = content;

		// Fix both single-line and multi-line import() statements
		// Handles: import("../pkg/kreuzberg_wasm.js")
		content = content.replace(/import\("\.\.\/pkg\/kreuzberg_wasm\.js"\)/g, 'import("./pkg/kreuzberg_wasm.js")');

		// Handles multi-line: import(\n  /* comment */\n  "../pkg/kreuzberg_wasm.js"\n)
		content = content.replace(/"\.\.\/pkg\/kreuzberg_wasm\.js"/g, '"./pkg/kreuzberg_wasm.js"');

		// Fix pdfium.js import path: ../pdfium.js -> ./pdfium.js (since bundled code is in dist/)
		content = content.replace(/import\("\.\.\/pdfium\.js"\)/g, 'import("./pdfium.js")');
		content = content.replace(/"\.\.\/pdfium\.js"/g, '"./pdfium.js"');

		if (content !== original) {
			fs.writeFileSync(file, content);
			console.log(`Fixed import paths in ${path.basename(file)}`);
		}
	}
}

console.log("Copy and path fixing complete!");
