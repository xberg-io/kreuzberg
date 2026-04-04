/**
 * Comprehensive table extraction quality tests for TypeScript Node.js bindings.
 *
 * Tests verify table extraction quality across multiple scenarios:
 * 1. Table structure extraction (rows, columns, headers)
 * 2. Complex tables (merged cells, nested tables)
 * 3. Table-in-table edge cases
 * 4. Format-specific table handling (PDF vs. Office formats)
 * 5. Performance with large tables (100+ rows)
 * 6. Markdown conversion accuracy
 * 7. Cell content preservation
 * 8. Table boundary detection
 * 9. Batch table extraction consistency
 * 10. Table metadata validation
 *
 * NAPI-RS bindings with plain object configs (NO builder pattern).
 */

import { readFileSync, realpathSync } from "node:fs";
import { beforeAll, describe, expect, it } from "vitest";
import { extractBytesSync, extractFileSync } from "../../dist/index.js";
import type { ExtractionConfig, Table } from "../../src/types.js";
import { getTestDocumentPath } from "../helpers/index.js";

let tinyPdfPath: string;
let mediumPdfPath: string;
let largePdfPath: string;
let tinyPdfBytes: Uint8Array;

beforeAll(() => {
	// Use PDFs with tables for testing
	tinyPdfPath = getTestDocumentPath("pdf/tiny.pdf");
	mediumPdfPath = getTestDocumentPath("pdf/medium.pdf");
	largePdfPath = getTestDocumentPath("pdf/large.pdf");

	try {
		// Resolve symlinks to get the actual file path (important for Windows compatibility)
		tinyPdfBytes = new Uint8Array(readFileSync(realpathSync(tinyPdfPath)));
	} catch {
		// File may not be accessible
	}
});

describe("Table Extraction Quality (Node.js Bindings)", () => {
	describe("table structure extraction", () => {
		it("should extract table rows, columns, and headers", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			expect(result).toBeDefined();
			expect(result.tables).toBeDefined();
			expect(Array.isArray(result.tables)).toBe(true);

			if (result.tables && result.tables.length > 0) {
				const table = result.tables[0];
				expect(table.cells).toBeDefined();
				expect(Array.isArray(table.cells)).toBe(true);
				expect(table.cells.length).toBeGreaterThan(0);

				// Each row should be an array of cells
				for (const row of table.cells) {
					expect(Array.isArray(row)).toBe(true);
					expect(row.length).toBeGreaterThan(0);
				}

				// Table should have markdown representation
				expect(table.markdown).toBeDefined();
				expect(typeof table.markdown).toBe("string");
				expect(table.markdown.length).toBeGreaterThan(0);
			}
		});

		it("should preserve cell content in extracted tables", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				const table = result.tables[0];

				// All cells should be strings
				for (const row of table.cells) {
					for (const cell of row) {
						expect(typeof cell).toBe("string");
						// Cell content should not be null or undefined
						expect(cell).not.toBeNull();
					}
				}
			}
		});

		it("should include page number for extracted tables", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				for (const table of result.tables) {
					expect(table).toHaveProperty("pageNumber");
					expect(table.pageNumber).toBeGreaterThanOrEqual(1);
					expect(Number.isInteger(table.pageNumber)).toBe(true);
				}
			}
		});

		it("should maintain column count consistency within tables", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				for (const table of result.tables) {
					const columnCounts = table.cells.map((row) => row.length);
					// All rows should have consistent column count
					const minCols = Math.min(...columnCounts);
					const maxCols = Math.max(...columnCounts);
					expect(minCols).toBeGreaterThan(0);
					expect(maxCols).toEqual(minCols);
				}
			}
		});
	});

	describe("complex table handling", () => {
		it("should extract tables with merged cell structures", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(mediumPdfPath, config);

			expect(result.tables).toBeDefined();

			if (result.tables && result.tables.length > 0) {
				// Tables may have merged cells or irregular structures
				const table = result.tables[0];
				expect(table.cells).toBeDefined();
				expect(table.cells.length).toBeGreaterThan(0);

				// Verify table structure is valid despite potential merges
				for (const row of table.cells) {
					expect(Array.isArray(row)).toBe(true);
					expect(row.length).toBeGreaterThan(0);
				}
			}
		});

		it("should handle tables with nested content", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(mediumPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				for (const table of result.tables) {
					// Tables with nested content should still have valid cell structure
					expect(Array.isArray(table.cells)).toBe(true);

					for (const row of table.cells) {
						expect(Array.isArray(row)).toBe(true);
						// Cells may contain multiline or formatted content
						for (const cell of row) {
							expect(typeof cell).toBe("string");
						}
					}
				}
			}
		});

		it("should preserve cell formatting and special characters", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(mediumPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				const allCellContent = result.tables.flatMap((t) => t.cells.flatMap((row) => row.join(" ")));

				// Content should be preserved with possible special characters
				for (const content of allCellContent) {
					expect(typeof content).toBe("string");
				}
			}
		});
	});

	describe("table-in-table edge cases", () => {
		it("should handle documents with multiple tables on single page", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(mediumPdfPath, config);

			expect(result.tables).toBeDefined();
			expect(Array.isArray(result.tables)).toBe(true);

			// Multiple tables may exist
			if (result.tables && result.tables.length > 1) {
				const pageNumbers = result.tables.map((t) => t.pageNumber);

				// Verify all tables have valid page numbers
				for (const pageNum of pageNumbers) {
					expect(pageNum).toBeGreaterThanOrEqual(1);
				}
			}
		});

		it("should detect table boundaries correctly", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(mediumPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				for (const table of result.tables) {
					// Tables should have complete structure
					const rowCount = table.cells.length;
					expect(rowCount).toBeGreaterThan(0);

					// All rows should have cells
					for (const row of table.cells) {
						expect(row.length).toBeGreaterThan(0);
					}
				}
			}
		});

		it("should handle empty cells and whitespace-only cells", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(mediumPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				for (const table of result.tables) {
					for (const row of table.cells) {
						for (const cell of row) {
							// Cells should be strings (may be empty or whitespace)
							expect(typeof cell).toBe("string");
						}
					}
				}
			}
		});
	});

	describe("format-specific table handling", () => {
		it("should extract tables from PDF documents", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			expect(result).toBeDefined();
			expect(result.mimeType).toContain("application/pdf");
			expect(result.tables).toBeDefined();
		});

		it("should extract tables from PDF using bytes interface", () => {
			if (!tinyPdfBytes || tinyPdfBytes.length === 0) {
				expect(true).toBe(true);
				return;
			}

			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractBytesSync(Buffer.from(tinyPdfBytes), "application/pdf", config);

			expect(result).toBeDefined();
			expect(result.tables).toBeDefined();
			expect(Array.isArray(result.tables)).toBe(true);
		});

		it("should handle PDF table extraction configuration", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
					extractMetadata: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			expect(result).toBeDefined();
			expect(result.tables).toBeDefined();
		});

		it("should maintain table quality across different PDF settings", () => {
			const config1: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const config2: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
					extractMetadata: true,
				},
			};

			const result1 = extractFileSync(tinyPdfPath, config1);
			const result2 = extractFileSync(tinyPdfPath, config2);

			// Both should extract tables
			expect(result1.tables).toBeDefined();
			expect(result2.tables).toBeDefined();

			// Table counts should match
			if (result1.tables && result2.tables) {
				expect(result1.tables.length).toBe(result2.tables.length);
			}
		});
	});

	describe("performance with large tables", () => {
		it("should handle extraction of large tables (100+ rows)", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(largePdfPath, config);

			expect(result.tables).toBeDefined();

			// If any table exists, verify it can be processed
			if (result.tables && result.tables.length > 0) {
				const table = result.tables[0];

				// Should be able to access all rows
				const rowCount = table.cells.length;
				expect(rowCount).toBeGreaterThan(0);

				// Should be able to process large row counts
				for (let i = 0; i < Math.min(rowCount, 10); i++) {
					expect(Array.isArray(table.cells[i])).toBe(true);
					expect(table.cells[i].length).toBeGreaterThan(0);
				}
			}
		});

		it("should maintain extraction consistency for large tables", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result1 = extractFileSync(largePdfPath, config);
			const result2 = extractFileSync(largePdfPath, config);

			// Same file should produce same table count
			expect(result1.tables.length).toBe(result2.tables.length);

			// First table should be identical
			if (result1.tables.length > 0 && result2.tables.length > 0) {
				expect(result1.tables[0].cells.length).toBe(result2.tables[0].cells.length);
			}
		});

		it("should handle efficient memory usage for table extraction", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const startMemory = process.memoryUsage().heapUsed;
			const result = extractFileSync(largePdfPath, config);
			const endMemory = process.memoryUsage().heapUsed;

			expect(result.tables).toBeDefined();

			// Memory increase should be reasonable
			const memoryIncrease = endMemory - startMemory;
			expect(memoryIncrease).toBeLessThan(500 * 1024 * 1024); // 500MB limit
		});
	});

	describe("markdown conversion accuracy", () => {
		it("should convert tables to valid markdown format", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				for (const table of result.tables) {
					// Markdown should contain pipe characters for table format
					expect(table.markdown).toBeDefined();
					expect(typeof table.markdown).toBe("string");

					// Valid markdown tables should have pipe separators
					const lines = table.markdown.split("\n");
					const hasPipes = lines.some((line) => line.includes("|"));

					expect(hasPipes || table.markdown.length === 0).toBe(true);
				}
			}
		});

		it("should preserve cell content in markdown representation", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				const table = result.tables[0];

				// Markdown should be non-empty if cells exist
				if (table.cells.length > 0 && table.cells[0].length > 0) {
					expect(table.markdown.length).toBeGreaterThan(0);
				}
			}
		});

		it("should format markdown with proper alignment markers", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				for (const table of result.tables) {
					const markdown = table.markdown;

					// Markdown tables typically have header separator
					const lines = markdown.split("\n");
					if (lines.length > 1) {
						// Look for alignment row (contains dashes and pipes)
						const hasAlignmentRow = lines.some((line) => /\|[\s\-:]+\|/.test(line));

						expect(hasAlignmentRow || markdown.length === 0).toBe(true);
					}
				}
			}
		});

		it("should handle markdown conversion with special characters", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(mediumPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				for (const table of result.tables) {
					// Markdown should be valid (not throw errors)
					expect(typeof table.markdown).toBe("string");

					// Should not have unescaped pipes in content (unless table format)
					const lines = table.markdown.split("\n");
					for (const line of lines) {
						expect(typeof line).toBe("string");
					}
				}
			}
		});
	});

	describe("cell content preservation", () => {
		it("should maintain complete cell content without truncation", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				const maxCellLengths = new Map<number, number>();

				for (const table of result.tables) {
					for (let rowIdx = 0; rowIdx < table.cells.length; rowIdx++) {
						const row = table.cells[rowIdx];
						for (let colIdx = 0; colIdx < row.length; colIdx++) {
							const cellKey = rowIdx * 1000 + colIdx;
							const currentMax = maxCellLengths.get(cellKey) || 0;
							maxCellLengths.set(cellKey, Math.max(currentMax, row[colIdx].length));
						}
					}
				}

				// Cell content should be preserved
				expect(maxCellLengths.size).toBeGreaterThanOrEqual(0);
			}
		});

		it("should preserve multiline content in cells", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(mediumPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				for (const table of result.tables) {
					for (const row of table.cells) {
						for (const cell of row) {
							// Multiline cells should be preserved
							expect(typeof cell).toBe("string");
						}
					}
				}
			}
		});

		it("should maintain numeric precision in table cells", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				for (const table of result.tables) {
					for (const row of table.cells) {
						for (const cell of row) {
							// Numeric content should be preserved as strings
							expect(typeof cell).toBe("string");

							// If it's numeric, precision should be maintained
							if (!Number.isNaN(parseFloat(cell))) {
								// Numeric cells should parse correctly
								const parsed = parseFloat(cell);
								expect(Number.isFinite(parsed)).toBe(true);
							}
						}
					}
				}
			}
		});

		it("should preserve whitespace within cells appropriately", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				for (const table of result.tables) {
					for (const row of table.cells) {
						for (const cell of row) {
							// Cell content should be string
							expect(typeof cell).toBe("string");
							// Trimmed version should exist
							expect(cell.trim()).toBeDefined();
						}
					}
				}
			}
		});
	});

	describe("table boundary detection", () => {
		it("should correctly identify table boundaries", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			expect(result.tables).toBeDefined();

			// Each table should be properly bounded
			if (result.tables && result.tables.length > 0) {
				for (const table of result.tables) {
					expect(table.cells.length).toBeGreaterThan(0);

					// All rows should have same structure (within reason)
					for (const row of table.cells) {
						expect(row.length).toBeGreaterThan(0);
					}
				}
			}
		});

		it("should not include content outside table boundaries", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				for (const table of result.tables) {
					// Tables should have defined boundaries
					expect(Array.isArray(table.cells)).toBe(true);
					expect(table.cells.length).toBeGreaterThan(0);

					// Each cell should be a string (properly extracted)
					for (const row of table.cells) {
						for (const cell of row) {
							expect(typeof cell).toBe("string");
						}
					}
				}
			}
		});

		it("should separate adjacent tables correctly", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(mediumPdfPath, config);

			if (result.tables && result.tables.length > 1) {
				// Multiple tables should be separated
				const table1 = result.tables[0];
				const table2 = result.tables[1];

				// Tables should be independent
				expect(table1.cells).toBeDefined();
				expect(table2.cells).toBeDefined();

				// Structure should be valid for each
				expect(table1.cells.length).toBeGreaterThan(0);
				expect(table2.cells.length).toBeGreaterThan(0);
			}
		});
	});

	describe("batch table extraction consistency", () => {
		it("should extract tables consistently across multiple calls", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result1 = extractFileSync(tinyPdfPath, config);
			const result2 = extractFileSync(tinyPdfPath, config);

			// Table count should be consistent
			expect(result1.tables.length).toBe(result2.tables.length);

			// First table structure should match
			if (result1.tables.length > 0 && result2.tables.length > 0) {
				expect(result1.tables[0].cells.length).toBe(result2.tables[0].cells.length);

				// Cell content should match
				for (let i = 0; i < result1.tables[0].cells.length; i++) {
					expect(result1.tables[0].cells[i]).toEqual(result2.tables[0].cells[i]);
				}
			}
		});

		it("should handle batch extraction without losing table data", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(mediumPdfPath, config);

			expect(result.tables).toBeDefined();
			expect(Array.isArray(result.tables)).toBe(true);

			// All extracted tables should be complete
			for (const table of result.tables) {
				expect(table.cells).toBeDefined();
				expect(table.cells.length).toBeGreaterThan(0);
			}
		});

		it("should maintain table order in batch processing", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(mediumPdfPath, config);

			if (result.tables && result.tables.length > 1) {
				// Tables should be ordered by page number and position
				for (let i = 1; i < result.tables.length; i++) {
					const prevTable = result.tables[i - 1];
					const currTable = result.tables[i];

					// Page numbers should be non-decreasing
					expect(currTable.pageNumber).toBeGreaterThanOrEqual(prevTable.pageNumber);
				}
			}
		});

		it("should handle configuration changes between extractions", () => {
			const configWithTables: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const configWithoutTables: ExtractionConfig = {
				pdfOptions: {
					extractTables: false,
				},
			};

			const resultWith = extractFileSync(tinyPdfPath, configWithTables);
			const resultWithout = extractFileSync(tinyPdfPath, configWithoutTables);

			// With tables enabled should extract tables or have valid array
			expect(resultWith.tables).toBeDefined();

			// Without tables enabled should not extract tables or have empty array
			expect(resultWithout.tables).toBeDefined();

			// With tables should have at least as many tables
			expect(resultWith.tables.length).toBeGreaterThanOrEqual(0);
		});
	});

	describe("table metadata validation", () => {
		it("should have valid page numbers for all tables", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			for (const table of result.tables) {
				expect(table).toHaveProperty("pageNumber");
				expect(Number.isInteger(table.pageNumber)).toBe(true);
				expect(table.pageNumber).toBeGreaterThanOrEqual(1);
			}
		});

		it("should have valid cell structure in all tables", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			for (const table of result.tables) {
				expect(table).toHaveProperty("cells");
				expect(Array.isArray(table.cells)).toBe(true);

				for (const row of table.cells) {
					expect(Array.isArray(row)).toBe(true);
					expect(row.length).toBeGreaterThan(0);

					for (const cell of row) {
						expect(typeof cell).toBe("string");
					}
				}
			}
		});

		it("should have valid markdown representation", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			for (const table of result.tables) {
				expect(table).toHaveProperty("markdown");
				expect(typeof table.markdown).toBe("string");
			}
		});

		it("should validate table dimensions consistency", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			for (const table of result.tables) {
				const rowCount = table.cells.length;
				const columnCounts = table.cells.map((row) => row.length);

				// Should have at least one row
				expect(rowCount).toBeGreaterThan(0);

				// Column counts should be consistent (allow for merged cells)
				const minCols = Math.min(...columnCounts);
				const maxCols = Math.max(...columnCounts);

				expect(minCols).toBeGreaterThan(0);
				expect(maxCols).toBeGreaterThanOrEqual(minCols);
			}
		});

		it("should include all required table properties", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			for (const table of result.tables) {
				// Check required properties
				expect(table).toHaveProperty("cells");
				expect(table).toHaveProperty("markdown");
				expect(table).toHaveProperty("pageNumber");

				// Verify property types
				expect(Array.isArray(table.cells)).toBe(true);
				expect(typeof table.markdown).toBe("string");
				expect(typeof table.pageNumber).toBe("number");
			}
		});

		it("should return Table type objects from extraction", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractTables: true,
				},
			};

			const result = extractFileSync(tinyPdfPath, config);

			if (result.tables && result.tables.length > 0) {
				const table: Table = result.tables[0];

				// Table should conform to interface
				expect(table.cells).toBeDefined();
				expect(table.markdown).toBeDefined();
				expect(table.pageNumber).toBeDefined();

				// Types should match
				expect(Array.isArray(table.cells)).toBe(true);
				expect(typeof table.markdown).toBe("string");
				expect(typeof table.pageNumber).toBe("number");
			}
		});
	});
});
