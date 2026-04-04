#!/usr/bin/env node
/**
 * Post-build script to fix module references in tsc-generated .d.ts files.
 *
 * Problem: tsc generates .d.ts files with .js extension imports (e.g., from './types.js')
 * but these files don't exist as separate JS files - they're bundled by tsup into index.js.
 *
 * Solution: This script rewrites all .js imports/exports to use .d.ts extensions,
 * allowing Deno and other tools to resolve types correctly.
 */

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const distDir = path.join(__dirname, "..", "dist");

/**
 * Fix module references in a .d.ts file
 * @param {string} filePath - Path to the file to fix
 */
function fixModuleReferences(filePath) {
	if (!fs.existsSync(filePath)) {
		return false;
	}

	let content = fs.readFileSync(filePath, "utf-8");
	const originalContent = content;

	// Replace .js extensions with .d.ts in import/export statements
	// Handles: from './types.js' -> from './types.d.ts'
	// Handles: from '../types.js' -> from '../types.d.ts'
	// Handles: from './adapters/wasm-adapter.js' -> from './adapters/wasm-adapter.d.ts'
	content = content.replace(/(from\s+['"])(\.\.?\/[^'"]+)(\.js)(['"])/g, "$1$2.d.ts$4");

	// Also fix any import() type references
	content = content.replace(/(import\(['"])(\.\.?\/[^'"]+)(\.js)(['"]\))/g, "$1$2.d.ts$4");

	if (content !== originalContent) {
		fs.writeFileSync(filePath, content);
		return true;
	}

	return false;
}

/**
 * Recursively find all .d.ts files
 * @param {string} dir - Directory to search
 * @returns {string[]} Array of file paths
 */
function findDtsFiles(dir) {
	const files = [];
	const entries = fs.readdirSync(dir, { withFileTypes: true });

	for (const entry of entries) {
		const fullPath = path.join(dir, entry.name);
		if (entry.isDirectory()) {
			files.push(...findDtsFiles(fullPath));
		} else if (entry.name.endsWith(".d.ts")) {
			files.push(fullPath);
		}
	}

	return files;
}

console.log("Fixing module references in .d.ts files...\n");

const dtsFiles = findDtsFiles(distDir);
let fixedCount = 0;

for (const file of dtsFiles) {
	if (fixModuleReferences(file)) {
		console.log(`✓ Fixed ${path.relative(distDir, file)}`);
		fixedCount++;
	}
}

if (fixedCount === 0) {
	console.log("No fixes needed.");
} else {
	console.log(`\nFixed ${fixedCount} file(s).`);
}
