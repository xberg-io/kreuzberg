/**
 * FontConfig configuration tests
 *
 * Tests for FontConfig feature that allows users to enable/disable custom
 * font provider and add custom font directories.
 */

import { describe, expect, it } from "vitest";
import type { FontConfig, PdfConfig } from "./config";

describe("FontConfig", () => {
	it("should have default values", () => {
		const config: FontConfig = {};

		expect(config.enabled).toBeUndefined();
		expect(config.customFontDirs).toBeUndefined();
	});

	it("should create with enabled=true", () => {
		const config: FontConfig = { enabled: true };

		expect(config.enabled).toBe(true);
		expect(config.customFontDirs).toBeUndefined();
	});

	it("should create with enabled=false", () => {
		const config: FontConfig = { enabled: false };

		expect(config.enabled).toBe(false);
		expect(config.customFontDirs).toBeUndefined();
	});

	it("should create with custom font directories", () => {
		const dirs = ["/usr/share/fonts/custom", "~/my-fonts"];
		const config: FontConfig = { customFontDirs: dirs };

		expect(config.customFontDirs).toEqual(dirs);
		expect(config.customFontDirs?.length).toBe(2);
	});

	it("should create with all parameters", () => {
		const dirs = ["/path/to/fonts", "/another/path"];
		const config: FontConfig = {
			enabled: true,
			customFontDirs: dirs,
		};

		expect(config.enabled).toBe(true);
		expect(config.customFontDirs).toEqual(dirs);
	});

	it("should create with enabled=false and custom dirs", () => {
		const dirs = ["/fonts"];
		const config: FontConfig = {
			enabled: false,
			customFontDirs: dirs,
		};

		expect(config.enabled).toBe(false);
		expect(config.customFontDirs).toEqual(dirs);
	});

	it("should handle empty custom directories array", () => {
		const config: FontConfig = { customFontDirs: [] };

		expect(config.customFontDirs).toEqual([]);
		expect(config.customFontDirs?.length).toBe(0);
	});

	it("should handle multiple custom directories", () => {
		const dirs = ["/path1", "/path2", "/path3", "~/fonts", "./relative-fonts"];
		const config: FontConfig = { customFontDirs: dirs };

		expect(config.customFontDirs?.length).toBe(5);
		expect(config.customFontDirs).toEqual(dirs);
	});

	it("should nest in PdfConfig", () => {
		const fontConfig: FontConfig = {
			enabled: true,
			customFontDirs: ["/fonts"],
		};
		const pdfConfig: PdfConfig = {
			extractImages: true,
			fontConfig,
		};

		expect(pdfConfig.fontConfig).toBeDefined();
		expect(pdfConfig.fontConfig?.enabled).toBe(true);
		expect(pdfConfig.fontConfig?.customFontDirs).toEqual(["/fonts"]);
	});

	it("should nest in PdfConfig with other options", () => {
		const fontConfig: FontConfig = {
			enabled: true,
			customFontDirs: ["/custom-fonts"],
		};
		const pdfConfig: PdfConfig = {
			extractImages: true,
			passwords: ["pass1"],
			extractMetadata: true,
			fontConfig,
		};

		expect(pdfConfig.extractImages).toBe(true);
		expect(pdfConfig.passwords).toEqual(["pass1"]);
		expect(pdfConfig.extractMetadata).toBe(true);
		expect(pdfConfig.fontConfig?.enabled).toBe(true);
	});

	it("should support type-safe property access", () => {
		const config: FontConfig = {
			enabled: false,
			customFontDirs: ["/usr/share/fonts"],
		};

		const enabled: boolean | undefined = config.enabled;
		const dirs: string[] | undefined = config.customFontDirs;

		expect(enabled).toBe(false);
		expect(dirs).toEqual(["/usr/share/fonts"]);
	});
});
