/**
 * Configuration helper utilities for ExtractionConfig manipulation.
 *
 * These functions provide standalone utilities for serializing, accessing,
 * and merging extraction configurations.
 */

import type { ExtractionConfig } from "../types/config.js";

/**
 * Serialize an ExtractionConfig to a JSON string.
 *
 * Converts a configuration object to its JSON representation.
 * The JSON can be used for persistence, transfer, or debugging.
 *
 * @param config - The configuration to serialize
 * @returns JSON string representation of the configuration
 *
 * @example
 * ```typescript
 * import { configToJson } from '@kreuzberg/core';
 *
 * const config: ExtractionConfig = { useCache: true, forceOcr: false };
 * const json = configToJson(config);
 * console.log(json); // '{"useCache":true,"forceOcr":false}'
 * ```
 */
export function configToJson(config: ExtractionConfig): string {
	return JSON.stringify(config);
}

/**
 * Get a configuration field by name with dot notation support.
 *
 * Retrieves a nested configuration field using dot notation
 * (e.g., "ocr.backend", "images.targetDpi").
 *
 * @param config - The configuration to access
 * @param fieldName - The field path to retrieve (supports dot notation)
 * @returns The field value, or undefined if not found
 *
 * @example
 * ```typescript
 * import { configGetField } from '@kreuzberg/core';
 *
 * const config: ExtractionConfig = {
 *   ocr: { backend: 'tesseract', language: 'en' },
 *   images: { targetDpi: 300 }
 * };
 *
 * const backend = configGetField(config, 'ocr.backend');
 * console.log(backend); // 'tesseract'
 *
 * const dpi = configGetField(config, 'images.targetDpi');
 * console.log(dpi); // 300
 *
 * const missing = configGetField(config, 'nonexistent.field');
 * console.log(missing); // undefined
 * ```
 */
export function configGetField(config: ExtractionConfig, fieldName: string): unknown {
	const parts = fieldName.split(".");
	let current: unknown = config;

	for (const part of parts) {
		if (current === null || current === undefined) {
			return undefined;
		}
		if (typeof current !== "object") {
			return undefined;
		}
		current = (current as Record<string, unknown>)[part];
	}

	return current;
}

/**
 * Merge two configurations, with override taking precedence.
 *
 * Performs a deep merge where fields from the override config
 * take precedence over the base config's fields. Returns a new
 * configuration object without modifying the inputs.
 *
 * For nested objects (like `ocr`, `images`, etc.), a shallow merge
 * is performed at each level, allowing partial overrides.
 *
 * @param base - The base configuration
 * @param override - Configuration to merge in (takes precedence)
 * @returns A new merged configuration
 *
 * @example
 * ```typescript
 * import { configMerge } from '@kreuzberg/core';
 *
 * const base: ExtractionConfig = {
 *   useCache: true,
 *   forceOcr: false,
 *   ocr: { backend: 'tesseract', language: 'en' }
 * };
 *
 * const override: Partial<ExtractionConfig> = {
 *   forceOcr: true,
 *   ocr: { backend: 'easyocr' }
 * };
 *
 * const merged = configMerge(base, override);
 * console.log(merged.useCache);       // true (from base)
 * console.log(merged.forceOcr);       // true (from override)
 * console.log(merged.ocr?.backend);   // 'easyocr' (from override)
 * console.log(merged.ocr?.language);  // 'en' (from base.ocr)
 * ```
 */
export function configMerge(base: ExtractionConfig, override: Partial<ExtractionConfig>): ExtractionConfig {
	const result: ExtractionConfig = { ...base };

	for (const key of Object.keys(override) as Array<keyof ExtractionConfig>) {
		const overrideValue = override[key];

		if (overrideValue === undefined) {
			continue;
		}

		const baseValue = base[key];

		// Deep merge for nested objects (but not arrays or null)
		if (
			overrideValue !== null &&
			typeof overrideValue === "object" &&
			!Array.isArray(overrideValue) &&
			baseValue !== null &&
			typeof baseValue === "object" &&
			!Array.isArray(baseValue)
		) {
			Reflect.set(result, key, {
				...baseValue,
				...overrideValue,
			});
		} else {
			Reflect.set(result, key, overrideValue);
		}
	}

	return result;
}
