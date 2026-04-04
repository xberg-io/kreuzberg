/**
 * Configuration validation utilities that delegate to native Rust validators via FFI.
 *
 * These functions call the native validation functions exposed through NAPI-RS,
 * ensuring a single source of truth for validation logic across all language bindings.
 *
 * NOTE: These functions are bound at runtime to the native NAPI-RS module.
 * The actual validation logic lives in crates/kreuzberg-ffi/src/validation.rs
 * and is exposed through crates/kreuzberg-node/src/lib.rs
 */

let nativeModule: Record<string, any>;

function getNativeModule(): Record<string, any> {
	if (!nativeModule) {
		try {
			nativeModule = require("kreuzberg-node");
		} catch (error) {
			throw new Error(
				`Unable to load native kreuzberg-node module: ${error instanceof Error ? error.message : String(error)}`,
			);
		}
	}
	return nativeModule;
}

/**
 * Interface for a single validation rule
 */
interface ValidationRule<T> {
	validatorName: string;
	errorMessage: (value: T) => string;
}

/**
 * Creates a validator function using the factory pattern.
 * Generates a function that validates a value against a native module validator.
 *
 * @param rule The validation rule containing native validator name and error message
 * @returns A validator function that throws on invalid input
 */
function createValidator<T>(rule: ValidationRule<T>): (value: T) => void {
	return (value: T): void => {
		const validator = getNativeModule()[rule.validatorName];
		if (!validator(value)) {
			throw new Error(rule.errorMessage(value));
		}
	};
}

/**
 * Creates a multi-parameter validator function for cases like chunking params.
 * Generates a function that validates multiple values against a native module validator.
 *
 * @param rule The validation rule
 * @param paramCount Number of parameters to validate
 * @returns A validator function that throws on invalid input
 */
function createMultiParamValidator<T extends any[]>(
	rule: ValidationRule<T>,
	_paramCount: number,
): (...args: T) => void {
	return (...args: T): void => {
		const validator = getNativeModule()[rule.validatorName];
		if (!validator(...args)) {
			throw new Error(rule.errorMessage(args));
		}
	};
}

/**
 * Centralized validation rules registry.
 * Each rule maps to a native validator function and provides error messaging.
 */
const VALIDATION_RULES = {
	binarizationMethod: {
		validatorName: "validateBinarizationMethod",
		errorMessage: (value: string) => `Invalid binarization method: ${value}`,
	} as ValidationRule<string>,

	ocrBackend: {
		validatorName: "validateOcrBackend",
		errorMessage: (value: string) => `Invalid OCR backend: ${value}`,
	} as ValidationRule<string>,

	languageCode: {
		validatorName: "validateLanguageCode",
		errorMessage: (value: string) => `Invalid language code: ${value}`,
	} as ValidationRule<string>,

	tokenReductionLevel: {
		validatorName: "validateTokenReductionLevel",
		errorMessage: (value: string) => `Invalid token reduction level: ${value}`,
	} as ValidationRule<string>,

	tesseractPsm: {
		validatorName: "validateTesseractPsm",
		errorMessage: (value: number) => `Invalid Tesseract PSM: ${value}. Valid range: 0-13`,
	} as ValidationRule<number>,

	tesseractOem: {
		validatorName: "validateTesseractOem",
		errorMessage: (value: number) => `Invalid Tesseract OEM: ${value}. Valid range: 0-3`,
	} as ValidationRule<number>,

	outputFormat: {
		validatorName: "validateOutputFormat",
		errorMessage: (value: string) => `Invalid output format: ${value}`,
	} as ValidationRule<string>,

	confidence: {
		validatorName: "validateConfidence",
		errorMessage: (value: number) => `Invalid confidence: ${value}. Valid range: 0.0-1.0`,
	} as ValidationRule<number>,

	dpi: {
		validatorName: "validateDpi",
		errorMessage: (value: number) => `Invalid DPI: ${value}. Valid range: 1-2400`,
	} as ValidationRule<number>,

	chunkingParams: {
		validatorName: "validateChunkingParams",
		errorMessage: (args: [number, number]) => `Invalid chunking params: maxChars=${args[0]}, maxOverlap=${args[1]}`,
	} as ValidationRule<[number, number]>,
};

/**
 * Validates a binarization method string.
 *
 * Valid methods: "otsu", "adaptive", "sauvola"
 *
 * @param method The binarization method to validate
 * @throws if the method is invalid
 *
 * @example
 * ```typescript
 * import { validateBinarizationMethod } from '@kreuzberg/core';
 *
 * try {
 *   validateBinarizationMethod('otsu');
 *   console.log('Valid method');
 * } catch (error) {
 *   console.error('Invalid method:', error.message);
 * }
 * ```
 */
export const validateBinarizationMethod = createValidator(VALIDATION_RULES.binarizationMethod);

/**
 * Validates an OCR backend string.
 *
 * Valid backends: "tesseract", "easyocr", "paddleocr"
 *
 * @param backend The OCR backend to validate
 * @throws if the backend is invalid
 *
 * @example
 * ```typescript
 * import { validateOcrBackend } from '@kreuzberg/core';
 *
 * try {
 *   validateOcrBackend('tesseract');
 * } catch (error) {
 *   console.error('Invalid backend:', error.message);
 * }
 * ```
 */
export const validateOcrBackend = createValidator(VALIDATION_RULES.ocrBackend);

/**
 * Validates a language code (ISO 639-1 or 639-3 format).
 *
 * Accepts both 2-letter codes (e.g., "en", "de") and 3-letter codes (e.g., "eng", "deu").
 *
 * @param code The language code to validate
 * @throws if the code is invalid
 *
 * @example
 * ```typescript
 * import { validateLanguageCode } from '@kreuzberg/core';
 *
 * try {
 *   validateLanguageCode('en');
 * } catch (error) {
 *   console.error('Invalid language code:', error.message);
 * }
 * ```
 */
export const validateLanguageCode = createValidator(VALIDATION_RULES.languageCode);

/**
 * Validates a token reduction level string.
 *
 * Valid levels: "off", "light", "moderate", "aggressive", "maximum"
 *
 * @param level The token reduction level to validate
 * @throws if the level is invalid
 *
 * @example
 * ```typescript
 * import { validateTokenReductionLevel } from '@kreuzberg/core';
 *
 * try {
 *   validateTokenReductionLevel('moderate');
 * } catch (error) {
 *   console.error('Invalid token reduction level:', error.message);
 * }
 * ```
 */
export const validateTokenReductionLevel = createValidator(VALIDATION_RULES.tokenReductionLevel);

/**
 * Validates a Tesseract Page Segmentation Mode (PSM) value.
 *
 * Valid range: 0-13
 *
 * @param psm The PSM value to validate
 * @throws if the PSM is invalid
 *
 * @example
 * ```typescript
 * import { validateTesseractPsm } from '@kreuzberg/core';
 *
 * try {
 *   validateTesseractPsm(3);
 * } catch (error) {
 *   console.error('Invalid PSM:', error.message);
 * }
 * ```
 */
export const validateTesseractPsm = createValidator(VALIDATION_RULES.tesseractPsm);

/**
 * Validates a Tesseract OCR Engine Mode (OEM) value.
 *
 * Valid range: 0-3
 *
 * @param oem The OEM value to validate
 * @throws if the OEM is invalid
 *
 * @example
 * ```typescript
 * import { validateTesseractOem } from '@kreuzberg/core';
 *
 * try {
 *   validateTesseractOem(1);
 * } catch (error) {
 *   console.error('Invalid OEM:', error.message);
 * }
 * ```
 */
export const validateTesseractOem = createValidator(VALIDATION_RULES.tesseractOem);

/**
 * Validates a tesseract output format string.
 *
 * Valid formats: "text", "markdown"
 *
 * @param format The output format to validate
 * @throws if the format is invalid
 *
 * @example
 * ```typescript
 * import { validateOutputFormat } from '@kreuzberg/core';
 *
 * try {
 *   validateOutputFormat('markdown');
 * } catch (error) {
 *   console.error('Invalid output format:', error.message);
 * }
 * ```
 */
export const validateOutputFormat = createValidator(VALIDATION_RULES.outputFormat);

/**
 * Validates a confidence threshold value.
 *
 * Valid range: 0.0 to 1.0 (inclusive)
 *
 * @param confidence The confidence threshold to validate
 * @throws if the confidence is invalid
 *
 * @example
 * ```typescript
 * import { validateConfidence } from '@kreuzberg/core';
 *
 * try {
 *   validateConfidence(0.75);
 * } catch (error) {
 *   console.error('Invalid confidence:', error.message);
 * }
 * ```
 */
export const validateConfidence = createValidator(VALIDATION_RULES.confidence);

/**
 * Validates a DPI (dots per inch) value.
 *
 * Valid range: 1-2400
 *
 * @param dpi The DPI value to validate
 * @throws if the DPI is invalid
 *
 * @example
 * ```typescript
 * import { validateDpi } from '@kreuzberg/core';
 *
 * try {
 *   validateDpi(300);
 * } catch (error) {
 *   console.error('Invalid DPI:', error.message);
 * }
 * ```
 */
export const validateDpi = createValidator(VALIDATION_RULES.dpi);

/**
 * Validates chunking parameters.
 *
 * Checks that `maxChars > 0` and `maxOverlap < maxChars`.
 *
 * @param maxChars Maximum characters per chunk
 * @param maxOverlap Maximum overlap between chunks
 * @throws if the parameters are invalid
 *
 * @example
 * ```typescript
 * import { validateChunkingParams } from '@kreuzberg/core';
 *
 * try {
 *   validateChunkingParams(1000, 200);
 * } catch (error) {
 *   console.error('Invalid chunking params:', error.message);
 * }
 * ```
 */
export const validateChunkingParams = createMultiParamValidator(VALIDATION_RULES.chunkingParams, 2);

/**
 * Get all valid binarization methods.
 *
 * @returns Array of valid binarization methods
 *
 * @example
 * ```typescript
 * import { getValidBinarizationMethods } from '@kreuzberg/core';
 *
 * const methods = await getValidBinarizationMethods();
 * console.log(methods); // ['otsu', 'adaptive', 'sauvola']
 * ```
 */
export async function getValidBinarizationMethods(): Promise<string[]> {
	const getter = getNativeModule()["getValidBinarizationMethods"];
	return getter();
}

/**
 * Get all valid language codes.
 *
 * @returns Array of valid language codes (both 2-letter and 3-letter codes)
 *
 * @example
 * ```typescript
 * import { getValidLanguageCodes } from '@kreuzberg/core';
 *
 * const codes = await getValidLanguageCodes();
 * console.log(codes); // ['en', 'de', 'fr', ..., 'eng', 'deu', 'fra', ...]
 * ```
 */
export async function getValidLanguageCodes(): Promise<string[]> {
	const getter = getNativeModule()["getValidLanguageCodes"];
	return getter();
}

/**
 * Get all valid OCR backends.
 *
 * @returns Array of valid OCR backends
 *
 * @example
 * ```typescript
 * import { getValidOcrBackends } from '@kreuzberg/core';
 *
 * const backends = await getValidOcrBackends();
 * console.log(backends); // ['tesseract', 'easyocr', 'paddleocr']
 * ```
 */
export async function getValidOcrBackends(): Promise<string[]> {
	const getter = getNativeModule()["getValidOcrBackends"];
	return getter();
}

/**
 * Get all valid token reduction levels.
 *
 * @returns Array of valid token reduction levels
 *
 * @example
 * ```typescript
 * import { getValidTokenReductionLevels } from '@kreuzberg/core';
 *
 * const levels = await getValidTokenReductionLevels();
 * console.log(levels); // ['off', 'light', 'moderate', 'aggressive', 'maximum']
 * ```
 */
export async function getValidTokenReductionLevels(): Promise<string[]> {
	const getter = getNativeModule()["getValidTokenReductionLevels"];
	return getter();
}
