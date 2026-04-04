import { join, resolve } from "node:path";

/**
 * Workspace locator configuration
 */
export interface WorkspaceLocator {
	/**
	 * Marker file/directory to identify workspace root
	 */
	marker: string;
	/**
	 * Maximum number of parent directories to search
	 */
	maxDepth?: number;
}

/**
 * Default Kreuzberg workspace locator
 */
export const KREUZBERG_WORKSPACE_LOCATOR: WorkspaceLocator = {
	marker: "test_documents",
	maxDepth: 5,
};

/**
 * Finds the workspace root by searching for a marker file/directory
 */
export function findWorkspaceRoot(locator: WorkspaceLocator, startDir: string = process.cwd()): string {
	const { marker, maxDepth = 5 } = locator;
	let currentDir = resolve(startDir);

	for (let i = 0; i < maxDepth; i++) {
		try {
			const markerPath = join(currentDir, marker);
			// Check if marker exists (will throw if not)
			const fs = require("node:fs");
			if (fs.existsSync(markerPath)) {
				return currentDir;
			}
		} catch {
			// Continue searching
		}

		const parentDir = resolve(currentDir, "..");
		if (parentDir === currentDir) {
			// Reached filesystem root
			break;
		}
		currentDir = parentDir;
	}

	throw new Error(
		`Could not find workspace root with marker "${marker}" within ${maxDepth} parent directories from ${startDir}`,
	);
}

/**
 * Resolves a document path relative to the workspace test_documents directory
 */
export function resolveDocument(relativePath: string, workspaceRoot?: string): string {
	const root = workspaceRoot ?? findWorkspaceRoot(KREUZBERG_WORKSPACE_LOCATOR);
	return join(root, "test_documents", relativePath);
}
