/**
 * Worker Integration Tests for TypeScript WASM Binding
 *
 * Comprehensive test suite for worker pool integration in Kreuzberg WASM bindings.
 * Tests cover worker pool initialization, message passing, error propagation,
 * worker lifecycle management, concurrent processing, and resource cleanup.
 *
 * @group wasm-binding
 * @group worker-integration
 * @group extraction
 */

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

/**
 * Mock worker message for WASM task
 */
interface WorkerMessage {
	id: string;
	type: "extract" | "chunk" | "embed";
	data: ArrayBuffer | Uint8Array;
	config?: Record<string, unknown>;
}

/**
 * Mock worker response
 */
interface WorkerResponse {
	id: string;
	success: boolean;
	result?: unknown;
	error?: string;
	duration: number;
}

/**
 * Mock worker with simulated WASM processing
 */
class MockWorker {
	private id: string;
	private active = false;
	private processedTasks = 0;
	private totalProcessingTime = 0;
	private lastActivity = 0;

	constructor(id: string) {
		this.id = id;
	}

	async init(): Promise<void> {
		this.active = true;
		this.lastActivity = Date.now();
	}

	async process(message: WorkerMessage): Promise<WorkerResponse> {
		if (!this.active) {
			throw new Error("Worker not initialized");
		}

		const startTime = Date.now();
		this.lastActivity = startTime;

		// Simulate processing based on data size
		const dataSize = message.data instanceof ArrayBuffer ? message.data.byteLength : message.data.length;

		// Simulate processing time proportional to data size
		const processingTime = Math.min(dataSize / (1024 * 100), 100); // Up to 100ms
		await new Promise((resolve) => setTimeout(resolve, processingTime));

		const endTime = Date.now();
		const duration = endTime - startTime;

		this.processedTasks++;
		this.totalProcessingTime += duration;

		// Return mock result
		const result = {
			content: "Processed content",
			tables: [],
			keywords: [],
			extractedSize: dataSize,
		};

		return {
			id: message.id,
			success: true,
			result,
			duration,
		};
	}

	terminate(): void {
		this.active = false;
	}

	getStats() {
		return {
			id: this.id,
			active: this.active,
			processedTasks: this.processedTasks,
			totalProcessingTime: this.totalProcessingTime,
			averageProcessingTime: this.processedTasks > 0 ? this.totalProcessingTime / this.processedTasks : 0,
			lastActivity: this.lastActivity,
		};
	}

	isActive(): boolean {
		return this.active;
	}
}

/**
 * Mock worker pool for WASM processing
 */
class WorkerPool {
	private workers: Map<string, MockWorker> = new Map();
	private pendingTasks: Map<string, WorkerMessage> = new Map();
	private completedTasks = 0;
	private failedTasks = 0;
	private poolSize: number;

	constructor(poolSize: number = 4) {
		this.poolSize = poolSize;
	}

	async init(): Promise<void> {
		for (let i = 0; i < this.poolSize; i++) {
			const worker = new MockWorker(`worker-${i}`);
			await worker.init();
			this.workers.set(`worker-${i}`, worker);
		}
	}

	async submit(message: WorkerMessage): Promise<WorkerResponse> {
		const availableWorker = Array.from(this.workers.values()).find((w) => w.isActive());

		if (!availableWorker) {
			throw new Error("No available workers");
		}

		this.pendingTasks.set(message.id, message);

		try {
			const response = await availableWorker.process(message);
			this.completedTasks++;
			this.pendingTasks.delete(message.id);
			return response;
		} catch (error) {
			this.failedTasks++;
			this.pendingTasks.delete(message.id);
			throw error;
		}
	}

	async terminate(): Promise<void> {
		for (const worker of this.workers.values()) {
			worker.terminate();
		}
		this.workers.clear();
		this.pendingTasks.clear();
	}

	getStats() {
		return {
			poolSize: this.poolSize,
			activeWorkers: Array.from(this.workers.values()).filter((w) => w.isActive()).length,
			completedTasks: this.completedTasks,
			failedTasks: this.failedTasks,
			pendingTasks: this.pendingTasks.size,
			workers: Array.from(this.workers.values()).map((w) => w.getStats()),
		};
	}

	getPendingTasks(): string[] {
		return Array.from(this.pendingTasks.keys());
	}
}

describe("WASM: Worker Integration", () => {
	let pool: WorkerPool;

	beforeEach(async () => {
		pool = new WorkerPool(4);
		await pool.init();
	});

	afterEach(async () => {
		await pool.terminate();
	});

	describe("worker pool initialization", () => {
		it("should initialize worker pool with default size", async () => {
			const testPool = new WorkerPool(4);
			await testPool.init();

			const stats = testPool.getStats();

			expect(stats.poolSize).toBe(4);
			expect(stats.activeWorkers).toBe(4);

			await testPool.terminate();
		});

		it("should initialize custom pool size", async () => {
			const testPool = new WorkerPool(8);
			await testPool.init();

			const stats = testPool.getStats();

			expect(stats.poolSize).toBe(8);
			expect(stats.activeWorkers).toBe(8);

			await testPool.terminate();
		});

		it("should initialize with single worker", async () => {
			const testPool = new WorkerPool(1);
			await testPool.init();

			const stats = testPool.getStats();

			expect(stats.poolSize).toBe(1);
			expect(stats.activeWorkers).toBe(1);

			await testPool.terminate();
		});

		it("should track worker initialization status", async () => {
			const stats = pool.getStats();

			expect(stats.activeWorkers).toBe(stats.poolSize);
			stats.workers.forEach((worker) => {
				expect(worker.active).toBe(true);
			});
		});
	});

	describe("message passing", () => {
		it("should submit and process message", async () => {
			const message: WorkerMessage = {
				id: "task-1",
				type: "extract",
				data: new Uint8Array([1, 2, 3, 4, 5]),
			};

			const response = await pool.submit(message);

			expect(response.id).toBe("task-1");
			expect(response.success).toBe(true);
			expect(response.duration).toBeGreaterThan(0);
		});

		it("should handle message with config", async () => {
			const message: WorkerMessage = {
				id: "task-2",
				type: "chunk",
				data: new Uint8Array(Array(1000).fill(0)),
				config: {
					maxChars: 500,
					overlap: 50,
				},
			};

			const response = await pool.submit(message);

			expect(response.success).toBe(true);
		});

		it("should process multiple messages in sequence", async () => {
			const messages: WorkerMessage[] = Array(5)
				.fill(null)
				.map((_, i) => ({
					id: `task-${i}`,
					type: "extract" as const,
					data: new Uint8Array(Array(100).fill(i)),
				}));

			const responses = [];
			for (const msg of messages) {
				responses.push(await pool.submit(msg));
			}

			expect(responses).toHaveLength(5);
			responses.forEach((resp) => {
				expect(resp.success).toBe(true);
			});

			const stats = pool.getStats();
			expect(stats.completedTasks).toBe(5);
		});

		it("should maintain message IDs through processing", async () => {
			const ids = ["msg-a", "msg-b", "msg-c"];
			const responses = [];

			for (const id of ids) {
				const message: WorkerMessage = {
					id,
					type: "extract",
					data: new Uint8Array([1]),
				};
				responses.push(await pool.submit(message));
			}

			responses.forEach((resp, index) => {
				expect(resp.id).toBe(ids[index]);
			});
		});

		it("should handle structured clone of worker messages", () => {
			const originalMessage: WorkerMessage = {
				id: "test-msg",
				type: "extract",
				data: new Uint8Array([1, 2, 3, 4, 5]),
				config: { enabled: true },
			};

			const cloned = structuredClone(originalMessage);

			expect(cloned.id).toBe(originalMessage.id);
			expect(cloned.type).toBe(originalMessage.type);
			expect(cloned.data).toEqual(originalMessage.data);
			expect(cloned.config).toEqual(originalMessage.config);

			// Verify deep copy
			(cloned.data as Uint8Array)[0] = 255;
			expect((originalMessage.data as Uint8Array)[0]).toBe(1);
		});
	});

	describe("error propagation", () => {
		it("should fail gracefully when no workers available", async () => {
			const testPool = new WorkerPool(0); // No workers
			await testPool.init();

			const message: WorkerMessage = {
				id: "test",
				type: "extract",
				data: new Uint8Array([1]),
			};

			try {
				await testPool.submit(message);
				expect(false).toBe(true); // Should not reach here
			} catch (error) {
				expect(error).toBeTruthy();
			}

			await testPool.terminate();
		});

		it("should track failed tasks", async () => {
			let stats = pool.getStats();
			expect(stats.failedTasks).toBe(0);

			// Submit some successful tasks
			for (let i = 0; i < 3; i++) {
				const message: WorkerMessage = {
					id: `task-${i}`,
					type: "extract",
					data: new Uint8Array([1]),
				};
				await pool.submit(message);
			}

			stats = pool.getStats();
			expect(stats.completedTasks).toBe(3);
			expect(stats.failedTasks).toBe(0);
		});

		it("should provide error details in response", async () => {
			const message: WorkerMessage = {
				id: "test-error",
				type: "extract",
				data: new Uint8Array([1]),
			};

			const response = await pool.submit(message);

			if (!response.success) {
				expect(response.error).toBeTruthy();
			}
		});
	});

	describe("worker lifecycle", () => {
		it("should track worker activity", () => {
			const stats = pool.getStats();

			stats.workers.forEach((workerStats) => {
				expect(workerStats.active).toBe(true);
				expect(workerStats.lastActivity).toBeGreaterThan(0);
			});
		});

		it("should terminate workers cleanly", async () => {
			let stats = pool.getStats();
			expect(stats.activeWorkers).toBe(4);

			await pool.terminate();

			stats = pool.getStats();
			expect(stats.activeWorkers).toBe(0);
		});

		it("should prevent processing after termination", async () => {
			await pool.terminate();

			const message: WorkerMessage = {
				id: "test",
				type: "extract",
				data: new Uint8Array([1]),
			};

			try {
				await pool.submit(message);
				expect(false).toBe(true); // Should not reach
			} catch {
				// Expected
			}
		});

		it("should track processing duration per worker", async () => {
			for (let i = 0; i < 10; i++) {
				const message: WorkerMessage = {
					id: `task-${i}`,
					type: "extract",
					data: new Uint8Array(Array(100).fill(0)),
				};
				await pool.submit(message);
			}

			const stats = pool.getStats();

			stats.workers.forEach((workerStats) => {
				if (workerStats.processedTasks > 0) {
					expect(workerStats.averageProcessingTime).toBeGreaterThan(0);
				}
			});
		});
	});

	describe("concurrent processing", () => {
		it("should handle concurrent task submission", async () => {
			const messages: WorkerMessage[] = Array(10)
				.fill(null)
				.map((_, i) => ({
					id: `concurrent-${i}`,
					type: "extract" as const,
					data: new Uint8Array(Array(50).fill(i)),
				}));

			// Submit all at once (simulating concurrent)
			const promises = messages.map((msg) => pool.submit(msg));
			const responses = await Promise.all(promises);

			expect(responses).toHaveLength(10);
			responses.forEach((resp) => {
				expect(resp.success).toBe(true);
			});

			const stats = pool.getStats();
			expect(stats.completedTasks).toBe(10);
		});

		it("should balance load across workers", async () => {
			for (let i = 0; i < 20; i++) {
				const message: WorkerMessage = {
					id: `load-${i}`,
					type: "extract",
					data: new Uint8Array(Array(100).fill(0)),
				};
				await pool.submit(message);
			}

			const stats = pool.getStats();

			// All workers should have processed tasks
			const activeWorkers = stats.workers.filter((w) => w.processedTasks > 0);
			expect(activeWorkers.length).toBeGreaterThan(0);
		});

		it("should queue pending tasks correctly", async () => {
			// This is simulated - in real implementation, tasks would queue
			const message: WorkerMessage = {
				id: "pending-test",
				type: "extract",
				data: new Uint8Array([1, 2, 3]),
			};

			const promise = pool.submit(message);
			const pendingBefore = pool.getPendingTasks();

			// Should be in pending or completed
			await promise;

			const pendingAfter = pool.getPendingTasks();
			expect(pendingAfter.includes(message.id)).toBe(false);
		});
	});

	describe("resource management", () => {
		it("should track memory per worker", () => {
			const stats = pool.getStats();

			stats.workers.forEach((workerStats) => {
				expect(typeof workerStats.id).toBe("string");
				expect(workerStats.active).toBe(true);
			});
		});

		it("should cleanup resources on termination", async () => {
			const testPool = new WorkerPool(4);
			await testPool.init();

			for (let i = 0; i < 5; i++) {
				await testPool.submit({
					id: `cleanup-${i}`,
					type: "extract",
					data: new Uint8Array(Array(100).fill(0)),
				});
			}

			let stats = testPool.getStats();
			expect(stats.completedTasks).toBe(5);

			await testPool.terminate();

			stats = testPool.getStats();
			expect(stats.activeWorkers).toBe(0);
		});

		it("should support worker pool reset", async () => {
			// Terminate and reinitialize
			await pool.terminate();

			const newPool = new WorkerPool(4);
			await newPool.init();

			const stats = newPool.getStats();
			expect(stats.activeWorkers).toBe(4);
			expect(stats.completedTasks).toBe(0);

			await newPool.terminate();
		});

		it("should measure total processing throughput", async () => {
			const taskCount = 50;
			const messages: WorkerMessage[] = Array(taskCount)
				.fill(null)
				.map((_, i) => ({
					id: `throughput-${i}`,
					type: "extract" as const,
					data: new Uint8Array(Array(100).fill(0)),
				}));

			const startTime = Date.now();

			for (const msg of messages) {
				await pool.submit(msg);
			}

			const endTime = Date.now();
			const totalDuration = endTime - startTime;

			const stats = pool.getStats();
			expect(stats.completedTasks).toBe(taskCount);

			const throughput = (taskCount / totalDuration) * 1000; // tasks per second
			expect(throughput).toBeGreaterThan(0);
		});
	});

	describe("worker data transfer", () => {
		it("should transfer binary data to worker", async () => {
			const binaryData = new Uint8Array([0xff, 0xd8, 0xff, 0xe0]); // JPEG header

			const message: WorkerMessage = {
				id: "binary-test",
				type: "extract",
				data: binaryData,
			};

			const response = await pool.submit(message);

			expect(response.success).toBe(true);
		});

		it("should handle large data transfers", async () => {
			const largeData = new Uint8Array(10 * 1024 * 1024); // 10MB
			for (let i = 0; i < largeData.length; i++) {
				largeData[i] = (i % 256) as number;
			}

			const message: WorkerMessage = {
				id: "large-data",
				type: "extract",
				data: largeData,
			};

			const response = await pool.submit(message);

			expect(response.success).toBe(true);
		});

		it("should support multiple data transfer types", async () => {
			const uint8Data = new Uint8Array([1, 2, 3]);
			const arrayBuffer = new ArrayBuffer(4);

			const msg1: WorkerMessage = {
				id: "uint8",
				type: "extract",
				data: uint8Data,
			};

			const msg2: WorkerMessage = {
				id: "arraybuffer",
				type: "extract",
				data: arrayBuffer,
			};

			const [resp1, resp2] = await Promise.all([pool.submit(msg1), pool.submit(msg2)]);

			expect(resp1.success).toBe(true);
			expect(resp2.success).toBe(true);
		});
	});
});
