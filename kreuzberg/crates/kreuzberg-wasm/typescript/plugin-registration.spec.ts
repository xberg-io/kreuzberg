import { beforeEach, describe, expect, it, vi } from "vitest";
import {
	clearPostProcessors,
	clearValidators,
	listPostProcessors,
	listValidators,
	registerPostProcessor,
	registerValidator,
	unregisterPostProcessor,
	unregisterValidator,
} from "./plugin-registry.js";
import type { ExtractionResult } from "./types.js";

/** Mock PostProcessor for testing */
interface MockPostProcessor {
	name: () => string;
	stage?: () => "early" | "middle" | "late";
	process: (result: ExtractionResult) => Promise<ExtractionResult> | ExtractionResult;
	shutdown?: () => Promise<void>;
}

/** Mock Validator for testing */
interface MockValidator {
	name: () => string;
	priority?: () => number;
	validate: (result: ExtractionResult) => Promise<{ valid: boolean; errors: string[] }>;
	shutdown?: () => Promise<void>;
}

describe("Plugin Registration System", () => {
	beforeEach(async () => {
		await clearPostProcessors();
		await clearValidators();
	});

	// Helper functions
	const createMockPostProcessor = (
		name: string,
		options: { stage?: "early" | "middle" | "late"; processAsync?: boolean } = {},
	): MockPostProcessor => {
		const stage = options.stage || "middle";
		const processAsync = options.processAsync !== false;

		return {
			name: () => name,
			stage: () => stage,
			process: processAsync
				? vi.fn(async (result) => ({
						...result,
						content: `[${name}] ${result.content}`,
					}))
				: vi.fn((result) => ({
						...result,
						content: `[${name}] ${result.content}`,
					})),
			shutdown: vi.fn(async () => undefined),
		};
	};

	const createMockValidator = (
		name: string,
		options: { priority?: number; validateAsync?: boolean } = {},
	): MockValidator => {
		const priority = options.priority ?? 50;
		const validateAsync = options.validateAsync !== false;

		return {
			name: () => name,
			priority: () => priority,
			validate: validateAsync
				? vi.fn(async () => ({ valid: true, errors: [] }))
				: vi.fn(() => ({ valid: true, errors: [] })),
			shutdown: vi.fn(async () => undefined),
		};
	};

	describe("Post-Processor Registration", () => {
		it("should register a valid post-processor", () => {
			const processor = createMockPostProcessor("test-processor");
			expect(() => registerPostProcessor(processor as any)).not.toThrow();

			const registered = listPostProcessors();
			expect(registered).toContain("test-processor");
		});

		it("should throw if post-processor is null", () => {
			expect(() => registerPostProcessor(null as any)).toThrow("Post-processor cannot be null or undefined");
		});

		it("should throw if post-processor is undefined", () => {
			expect(() => registerPostProcessor(undefined as any)).toThrow("Post-processor cannot be null or undefined");
		});

		it("should throw if post-processor missing name method", () => {
			const invalid = {
				process: async () => ({}),
			};
			expect(() => registerPostProcessor(invalid as any)).toThrow("must implement name() method");
		});

		it("should throw if post-processor missing process method", () => {
			const invalid = {
				name: () => "test",
			};
			expect(() => registerPostProcessor(invalid as any)).toThrow("must implement process() method");
		});

		it("should throw if post-processor name is empty string", () => {
			const processor = {
				name: () => "",
				process: async () => ({}),
			};
			expect(() => registerPostProcessor(processor as any)).toThrow("Post-processor name must be a non-empty string");
		});

		it("should throw if post-processor name is not a string", () => {
			const processor = {
				name: () => 123,
				process: async () => ({}),
			};
			expect(() => registerPostProcessor(processor as any)).toThrow("Post-processor name must be a non-empty string");
		});

		it("should allow overwriting existing post-processor", () => {
			const processor1 = createMockPostProcessor("test");
			const processor2 = createMockPostProcessor("test");

			registerPostProcessor(processor1 as any);
			expect(() => registerPostProcessor(processor2 as any)).not.toThrow();

			const registered = listPostProcessors();
			expect(registered).toContain("test");
		});

		it("should warn when overwriting post-processor", () => {
			const warnSpy = vi.spyOn(console, "warn");
			const processor1 = createMockPostProcessor("test");
			const processor2 = createMockPostProcessor("test");

			registerPostProcessor(processor1 as any);
			registerPostProcessor(processor2 as any);

			expect(warnSpy).toHaveBeenCalledWith(expect.stringContaining("already registered"));

			warnSpy.mockRestore();
		});

		it("should register multiple different post-processors", () => {
			const processor1 = createMockPostProcessor("processor1");
			const processor2 = createMockPostProcessor("processor2");

			registerPostProcessor(processor1 as any);
			registerPostProcessor(processor2 as any);

			const registered = listPostProcessors();
			expect(registered).toContain("processor1");
			expect(registered).toContain("processor2");
			expect(registered.length).toBe(2);
		});

		it("should support synchronous post-processors", () => {
			const processor = createMockPostProcessor("sync-processor", { processAsync: false });
			expect(() => registerPostProcessor(processor as any)).not.toThrow();

			const registered = listPostProcessors();
			expect(registered).toContain("sync-processor");
		});

		it("should support asynchronous post-processors", () => {
			const processor = createMockPostProcessor("async-processor", { processAsync: true });
			expect(() => registerPostProcessor(processor as any)).not.toThrow();

			const registered = listPostProcessors();
			expect(registered).toContain("async-processor");
		});
	});

	describe("Post-Processor Unregistration", () => {
		it("should unregister a registered post-processor", async () => {
			const processor = createMockPostProcessor("test");
			registerPostProcessor(processor as any);

			expect(listPostProcessors()).toContain("test");

			await unregisterPostProcessor("test");

			expect(listPostProcessors()).not.toContain("test");
		});

		it("should throw if post-processor not found", async () => {
			await expect(unregisterPostProcessor("nonexistent")).rejects.toThrow(
				'Post-processor "nonexistent" is not registered',
			);
		});

		it("should call shutdown method if available", async () => {
			const processor = createMockPostProcessor("test");
			const shutdownSpy = vi.spyOn(processor, "shutdown");

			registerPostProcessor(processor as any);
			await unregisterPostProcessor("test");

			expect(shutdownSpy).toHaveBeenCalled();
		});

		it("should not throw if shutdown fails", async () => {
			const processor = createMockPostProcessor("test");
			processor.shutdown = vi.fn(async () => {
				throw new Error("Shutdown failed");
			});

			registerPostProcessor(processor as any);

			expect(async () => {
				await unregisterPostProcessor("test");
			}).not.toThrow();
		});

		it("should remove post-processor even if shutdown fails", async () => {
			const processor = createMockPostProcessor("test");
			processor.shutdown = vi.fn(async () => {
				throw new Error("Shutdown error");
			});

			registerPostProcessor(processor as any);
			await unregisterPostProcessor("test");

			expect(listPostProcessors()).not.toContain("test");
		});

		it("should be case-sensitive", async () => {
			const processor = createMockPostProcessor("Test");
			registerPostProcessor(processor as any);

			await expect(unregisterPostProcessor("test")).rejects.toThrow("is not registered");

			expect(listPostProcessors()).toContain("Test");
		});
	});

	describe("Post-Processor Listing and Clearing", () => {
		it("should return empty array when no post-processors registered", () => {
			const processors = listPostProcessors();
			expect(processors).toEqual([]);
		});

		it("should return post-processor names as array", () => {
			const processor = createMockPostProcessor("test");
			registerPostProcessor(processor as any);

			const processors = listPostProcessors();
			expect(Array.isArray(processors)).toBe(true);
			expect(processors).toContain("test");
		});

		it("should return all registered post-processors", () => {
			const processor1 = createMockPostProcessor("processor1");
			const processor2 = createMockPostProcessor("processor2");
			const processor3 = createMockPostProcessor("processor3");

			registerPostProcessor(processor1 as any);
			registerPostProcessor(processor2 as any);
			registerPostProcessor(processor3 as any);

			const processors = listPostProcessors();
			expect(processors).toHaveLength(3);
			expect(processors).toContain("processor1");
			expect(processors).toContain("processor2");
			expect(processors).toContain("processor3");
		});

		it("should clear all post-processors", async () => {
			const processor1 = createMockPostProcessor("processor1");
			const processor2 = createMockPostProcessor("processor2");

			registerPostProcessor(processor1 as any);
			registerPostProcessor(processor2 as any);

			expect(listPostProcessors()).toHaveLength(2);

			await clearPostProcessors();

			expect(listPostProcessors()).toEqual([]);
		});

		it("should call shutdown on all post-processors during clear", async () => {
			const processor1 = createMockPostProcessor("processor1");
			const processor2 = createMockPostProcessor("processor2");

			const shutdown1 = vi.spyOn(processor1, "shutdown");
			const shutdown2 = vi.spyOn(processor2, "shutdown");

			registerPostProcessor(processor1 as any);
			registerPostProcessor(processor2 as any);

			await clearPostProcessors();

			expect(shutdown1).toHaveBeenCalled();
			expect(shutdown2).toHaveBeenCalled();
		});

		it("should not throw if shutdown fails during clear", async () => {
			const processor = createMockPostProcessor("test");
			processor.shutdown = vi.fn(async () => {
				throw new Error("Shutdown error");
			});

			registerPostProcessor(processor as any);

			expect(async () => {
				await clearPostProcessors();
			}).not.toThrow();
		});

		it("should clear all even if shutdowns fail", async () => {
			const processor1 = createMockPostProcessor("processor1");
			const processor2 = createMockPostProcessor("processor2");

			processor1.shutdown = vi.fn(async () => {
				throw new Error("Error 1");
			});
			processor2.shutdown = vi.fn(async () => {
				throw new Error("Error 2");
			});

			registerPostProcessor(processor1 as any);
			registerPostProcessor(processor2 as any);

			await clearPostProcessors();

			expect(listPostProcessors()).toEqual([]);
		});
	});

	describe("Validator Registration", () => {
		it("should register a valid validator", () => {
			const validator = createMockValidator("test-validator");
			expect(() => registerValidator(validator as any)).not.toThrow();

			const registered = listValidators();
			expect(registered).toContain("test-validator");
		});

		it("should throw if validator is null", () => {
			expect(() => registerValidator(null as any)).toThrow("Validator cannot be null or undefined");
		});

		it("should throw if validator is undefined", () => {
			expect(() => registerValidator(undefined as any)).toThrow("Validator cannot be null or undefined");
		});

		it("should throw if validator missing name method", () => {
			const invalid = {
				validate: async () => ({ valid: true, errors: [] }),
			};
			expect(() => registerValidator(invalid as any)).toThrow("must implement name() method");
		});

		it("should throw if validator missing validate method", () => {
			const invalid = {
				name: () => "test",
			};
			expect(() => registerValidator(invalid as any)).toThrow("must implement validate() method");
		});

		it("should throw if validator name is empty string", () => {
			const validator = {
				name: () => "",
				validate: async () => ({ valid: true, errors: [] }),
			};
			expect(() => registerValidator(validator as any)).toThrow("Validator name must be a non-empty string");
		});

		it("should throw if validator name is not a string", () => {
			const validator = {
				name: () => 123,
				validate: async () => ({ valid: true, errors: [] }),
			};
			expect(() => registerValidator(validator as any)).toThrow("Validator name must be a non-empty string");
		});

		it("should allow overwriting existing validator", () => {
			const validator1 = createMockValidator("test");
			const validator2 = createMockValidator("test");

			registerValidator(validator1 as any);
			expect(() => registerValidator(validator2 as any)).not.toThrow();

			const registered = listValidators();
			expect(registered).toContain("test");
		});

		it("should warn when overwriting validator", () => {
			const warnSpy = vi.spyOn(console, "warn");
			const validator1 = createMockValidator("test");
			const validator2 = createMockValidator("test");

			registerValidator(validator1 as any);
			registerValidator(validator2 as any);

			expect(warnSpy).toHaveBeenCalledWith(expect.stringContaining("already registered"));

			warnSpy.mockRestore();
		});

		it("should register multiple different validators", () => {
			const validator1 = createMockValidator("validator1");
			const validator2 = createMockValidator("validator2");

			registerValidator(validator1 as any);
			registerValidator(validator2 as any);

			const registered = listValidators();
			expect(registered).toContain("validator1");
			expect(registered).toContain("validator2");
			expect(registered.length).toBe(2);
		});

		it("should support synchronous validators", () => {
			const validator = createMockValidator("sync-validator", { validateAsync: false });
			expect(() => registerValidator(validator as any)).not.toThrow();

			const registered = listValidators();
			expect(registered).toContain("sync-validator");
		});

		it("should support asynchronous validators", () => {
			const validator = createMockValidator("async-validator", { validateAsync: true });
			expect(() => registerValidator(validator as any)).not.toThrow();

			const registered = listValidators();
			expect(registered).toContain("async-validator");
		});
	});

	describe("Validator Unregistration", () => {
		it("should unregister a registered validator", async () => {
			const validator = createMockValidator("test");
			registerValidator(validator as any);

			expect(listValidators()).toContain("test");

			await unregisterValidator("test");

			expect(listValidators()).not.toContain("test");
		});

		it("should throw if validator not found", async () => {
			await expect(unregisterValidator("nonexistent")).rejects.toThrow('Validator "nonexistent" is not registered');
		});

		it("should call shutdown method if available", async () => {
			const validator = createMockValidator("test");
			const shutdownSpy = vi.spyOn(validator, "shutdown");

			registerValidator(validator as any);
			await unregisterValidator("test");

			expect(shutdownSpy).toHaveBeenCalled();
		});

		it("should not throw if shutdown fails", async () => {
			const validator = createMockValidator("test");
			validator.shutdown = vi.fn(async () => {
				throw new Error("Shutdown failed");
			});

			registerValidator(validator as any);

			expect(async () => {
				await unregisterValidator("test");
			}).not.toThrow();
		});

		it("should remove validator even if shutdown fails", async () => {
			const validator = createMockValidator("test");
			validator.shutdown = vi.fn(async () => {
				throw new Error("Shutdown error");
			});

			registerValidator(validator as any);
			await unregisterValidator("test");

			expect(listValidators()).not.toContain("test");
		});

		it("should be case-sensitive", async () => {
			const validator = createMockValidator("Test");
			registerValidator(validator as any);

			await expect(unregisterValidator("test")).rejects.toThrow("is not registered");

			expect(listValidators()).toContain("Test");
		});
	});

	describe("Validator Listing and Clearing", () => {
		it("should return empty array when no validators registered", () => {
			const validators = listValidators();
			expect(validators).toEqual([]);
		});

		it("should return validator names as array", () => {
			const validator = createMockValidator("test");
			registerValidator(validator as any);

			const validators = listValidators();
			expect(Array.isArray(validators)).toBe(true);
			expect(validators).toContain("test");
		});

		it("should return all registered validators", () => {
			const validator1 = createMockValidator("validator1");
			const validator2 = createMockValidator("validator2");
			const validator3 = createMockValidator("validator3");

			registerValidator(validator1 as any);
			registerValidator(validator2 as any);
			registerValidator(validator3 as any);

			const validators = listValidators();
			expect(validators).toHaveLength(3);
			expect(validators).toContain("validator1");
			expect(validators).toContain("validator2");
			expect(validators).toContain("validator3");
		});

		it("should clear all validators", async () => {
			const validator1 = createMockValidator("validator1");
			const validator2 = createMockValidator("validator2");

			registerValidator(validator1 as any);
			registerValidator(validator2 as any);

			expect(listValidators()).toHaveLength(2);

			await clearValidators();

			expect(listValidators()).toEqual([]);
		});

		it("should call shutdown on all validators during clear", async () => {
			const validator1 = createMockValidator("validator1");
			const validator2 = createMockValidator("validator2");

			const shutdown1 = vi.spyOn(validator1, "shutdown");
			const shutdown2 = vi.spyOn(validator2, "shutdown");

			registerValidator(validator1 as any);
			registerValidator(validator2 as any);

			await clearValidators();

			expect(shutdown1).toHaveBeenCalled();
			expect(shutdown2).toHaveBeenCalled();
		});

		it("should not throw if shutdown fails during clear", async () => {
			const validator = createMockValidator("test");
			validator.shutdown = vi.fn(async () => {
				throw new Error("Shutdown error");
			});

			registerValidator(validator as any);

			expect(async () => {
				await clearValidators();
			}).not.toThrow();
		});

		it("should clear all even if shutdowns fail", async () => {
			const validator1 = createMockValidator("validator1");
			const validator2 = createMockValidator("validator2");

			validator1.shutdown = vi.fn(async () => {
				throw new Error("Error 1");
			});
			validator2.shutdown = vi.fn(async () => {
				throw new Error("Error 2");
			});

			registerValidator(validator1 as any);
			registerValidator(validator2 as any);

			await clearValidators();

			expect(listValidators()).toEqual([]);
		});
	});

	describe("Processing Stage Support", () => {
		it("should handle early processing stage", () => {
			const processor = createMockPostProcessor("early-processor", { stage: "early" });
			expect(() => registerPostProcessor(processor as any)).not.toThrow();

			const registered = listPostProcessors();
			expect(registered).toContain("early-processor");
		});

		it("should handle middle processing stage", () => {
			const processor = createMockPostProcessor("middle-processor", { stage: "middle" });
			expect(() => registerPostProcessor(processor as any)).not.toThrow();

			const registered = listPostProcessors();
			expect(registered).toContain("middle-processor");
		});

		it("should handle late processing stage", () => {
			const processor = createMockPostProcessor("late-processor", { stage: "late" });
			expect(() => registerPostProcessor(processor as any)).not.toThrow();

			const registered = listPostProcessors();
			expect(registered).toContain("late-processor");
		});

		it("should register processors with different stages", () => {
			const earlyProcessor = createMockPostProcessor("early", { stage: "early" });
			const middleProcessor = createMockPostProcessor("middle", { stage: "middle" });
			const lateProcessor = createMockPostProcessor("late", { stage: "late" });

			registerPostProcessor(earlyProcessor as any);
			registerPostProcessor(middleProcessor as any);
			registerPostProcessor(lateProcessor as any);

			const registered = listPostProcessors();
			expect(registered).toContain("early");
			expect(registered).toContain("middle");
			expect(registered).toContain("late");
			expect(registered.length).toBe(3);
		});
	});

	describe("Validator Priority Support", () => {
		it("should register validator with default priority", () => {
			const validator = createMockValidator("test");
			expect(() => registerValidator(validator as any)).not.toThrow();

			const registered = listValidators();
			expect(registered).toContain("test");
		});

		it("should register validator with custom priority", () => {
			const validator = createMockValidator("high-priority", { priority: 100 });
			expect(() => registerValidator(validator as any)).not.toThrow();

			const registered = listValidators();
			expect(registered).toContain("high-priority");
		});

		it("should register multiple validators with different priorities", () => {
			const validator1 = createMockValidator("validator1", { priority: 10 });
			const validator2 = createMockValidator("validator2", { priority: 50 });
			const validator3 = createMockValidator("validator3", { priority: 100 });

			registerValidator(validator1 as any);
			registerValidator(validator2 as any);
			registerValidator(validator3 as any);

			const registered = listValidators();
			expect(registered).toHaveLength(3);
			expect(registered).toContain("validator1");
			expect(registered).toContain("validator2");
			expect(registered).toContain("validator3");
		});
	});

	describe("Integration Scenarios", () => {
		it("should support registering and executing multiple post-processors", async () => {
			const processor1 = createMockPostProcessor("processor1");
			const processor2 = createMockPostProcessor("processor2");

			registerPostProcessor(processor1 as any);
			registerPostProcessor(processor2 as any);

			const processors = listPostProcessors();
			expect(processors).toHaveLength(2);
			expect(processors).toContain("processor1");
			expect(processors).toContain("processor2");
		});

		it("should support registering and executing multiple validators", async () => {
			const validator1 = createMockValidator("validator1");
			const validator2 = createMockValidator("validator2");

			registerValidator(validator1 as any);
			registerValidator(validator2 as any);

			const validators = listValidators();
			expect(validators).toHaveLength(2);
			expect(validators).toContain("validator1");
			expect(validators).toContain("validator2");
		});

		it("should support clearing and re-registering post-processors", async () => {
			const processor1 = createMockPostProcessor("processor1");
			const processor2 = createMockPostProcessor("processor2");

			registerPostProcessor(processor1 as any);
			registerPostProcessor(processor2 as any);

			await clearPostProcessors();
			expect(listPostProcessors()).toEqual([]);

			const processor3 = createMockPostProcessor("processor3");
			registerPostProcessor(processor3 as any);

			expect(listPostProcessors()).toEqual(["processor3"]);
		});

		it("should support clearing and re-registering validators", async () => {
			const validator1 = createMockValidator("validator1");
			const validator2 = createMockValidator("validator2");

			registerValidator(validator1 as any);
			registerValidator(validator2 as any);

			await clearValidators();
			expect(listValidators()).toEqual([]);

			const validator3 = createMockValidator("validator3");
			registerValidator(validator3 as any);

			expect(listValidators()).toEqual(["validator3"]);
		});

		it("should maintain separate registries for processors and validators", () => {
			const processor = createMockPostProcessor("processor");
			const validator = createMockValidator("validator");

			registerPostProcessor(processor as any);
			registerValidator(validator as any);

			const processors = listPostProcessors();
			const validators = listValidators();

			expect(processors).toContain("processor");
			expect(processors).not.toContain("validator");

			expect(validators).toContain("validator");
			expect(validators).not.toContain("processor");
		});

		it("should support re-registering after unregister", async () => {
			const processor = createMockPostProcessor("test");

			registerPostProcessor(processor as any);
			await unregisterPostProcessor("test");
			expect(listPostProcessors()).not.toContain("test");

			registerPostProcessor(processor as any);
			expect(listPostProcessors()).toContain("test");
		});

		it("should allow independent management of processors and validators", async () => {
			const processor = createMockPostProcessor("processor");
			const validator = createMockValidator("validator");

			registerPostProcessor(processor as any);
			registerValidator(validator as any);

			await unregisterPostProcessor("processor");

			expect(listPostProcessors()).not.toContain("processor");
			expect(listValidators()).toContain("validator");
		});
	});

	describe("Error Handling and Edge Cases", () => {
		it("should provide helpful error when unregistering non-existent post-processor", async () => {
			const processor = createMockPostProcessor("existing");
			registerPostProcessor(processor as any);

			try {
				await unregisterPostProcessor("nonexistent");
				expect.fail("Should have thrown");
			} catch (error) {
				if (error instanceof Error) {
					expect(error.message).toContain("nonexistent");
					expect(error.message).toContain("is not registered");
				}
			}
		});

		it("should provide helpful error when unregistering non-existent validator", async () => {
			const validator = createMockValidator("existing");
			registerValidator(validator as any);

			try {
				await unregisterValidator("nonexistent");
				expect.fail("Should have thrown");
			} catch (error) {
				if (error instanceof Error) {
					expect(error.message).toContain("nonexistent");
					expect(error.message).toContain("is not registered");
				}
			}
		});

		it("should work with empty registry operations", async () => {
			expect(() => {
				listPostProcessors();
				listValidators();
			}).not.toThrow();

			expect(async () => {
				await clearPostProcessors();
				await clearValidators();
			}).not.toThrow();
		});

		it("should handle rapid register/unregister cycles", async () => {
			for (let i = 0; i < 5; i++) {
				const processor = createMockPostProcessor(`processor-${i}`);
				registerPostProcessor(processor as any);
				await unregisterPostProcessor(`processor-${i}`);
			}

			expect(listPostProcessors()).toEqual([]);
		});

		it("should not throw when clearing empty registries", async () => {
			expect(async () => {
				await clearPostProcessors();
				await clearValidators();
			}).not.toThrow();

			expect(listPostProcessors()).toEqual([]);
			expect(listValidators()).toEqual([]);
		});
	});
});
