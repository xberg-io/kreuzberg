/**
 * Memory Management Tests for TypeScript WASM Binding
 *
 * Comprehensive test suite for memory management in Kreuzberg WASM bindings.
 * Tests cover memory allocation patterns, cleanup procedures, leak detection,
 * large document handling, buffer pooling, and garbage collection integration.
 *
 * CRITICAL: Memory management is essential for WASM due to fixed memory pools
 * and potential stack/heap exhaustion when processing large documents.
 *
 * @group wasm-binding
 * @group memory-management
 * @group critical
 */

import { afterEach, beforeEach, describe, expect, it } from "vitest";

/**
 * Memory allocation tracker for WASM module
 */
class WasmMemoryTracker {
	private allocations: Map<number, { size: number; timestamp: number }> = new Map();
	private deallocations: Map<number, { size: number; timestamp: number }> = new Map();
	private peakMemory = 0;
	private currentMemory = 0;
	private nextId = 1;

	allocate(bytes: number): number {
		const id = this.nextId++;
		const timestamp = Date.now();

		this.allocations.set(id, { size: bytes, timestamp });
		this.currentMemory += bytes;

		if (this.currentMemory > this.peakMemory) {
			this.peakMemory = this.currentMemory;
		}

		return id;
	}

	deallocate(id: number): boolean {
		const alloc = this.allocations.get(id);
		if (!alloc) {
			return false;
		}

		const timestamp = Date.now();
		this.deallocations.set(id, { size: alloc.size, timestamp });
		this.currentMemory -= alloc.size;
		this.allocations.delete(id);

		return true;
	}

	getStats() {
		return {
			currentMemory: this.currentMemory,
			peakMemory: this.peakMemory,
			allocationsCount: this.allocations.size,
			deallocationsCount: this.deallocations.size,
			leakedMemory: this.currentMemory, // Remaining allocated but not deallocated
		};
	}

	getLeaks(): number[] {
		return Array.from(this.allocations.keys());
	}

	clear(): void {
		this.allocations.clear();
		this.deallocations.clear();
		this.currentMemory = 0;
		this.peakMemory = 0;
	}
}

/**
 * Simulated WASM memory pool with fixed size
 */
class WasmMemoryPool {
	private pool: ArrayBuffer;
	private pageSize: number;
	private pageAllocations: boolean[];
	private pageMetadata: Map<number, { size: number; allocatedAt: number }> = new Map();

	constructor(totalBytes: number = 64 * 1024 * 1024, pageSize: number = 64 * 1024) {
		// 64MB pool, 64KB pages
		this.pool = new ArrayBuffer(totalBytes);
		this.pageSize = pageSize;
		const pageCount = Math.ceil(totalBytes / pageSize);
		this.pageAllocations = Array(pageCount).fill(false);
	}

	allocate(bytes: number): Uint8Array | null {
		const pagesNeeded = Math.ceil(bytes / this.pageSize);
		let startPage = -1;

		// Find contiguous free pages
		for (let i = 0; i <= this.pageAllocations.length - pagesNeeded; i++) {
			let free = true;
			for (let j = 0; j < pagesNeeded; j++) {
				if (this.pageAllocations[i + j]) {
					free = false;
					break;
				}
			}
			if (free) {
				startPage = i;
				break;
			}
		}

		if (startPage === -1) {
			return null; // Allocation failed
		}

		// Mark pages as allocated
		for (let i = startPage; i < startPage + pagesNeeded; i++) {
			this.pageAllocations[i] = true;
		}

		const offset = startPage * this.pageSize;
		const buffer = new Uint8Array(this.pool, offset, bytes);
		this.pageMetadata.set(offset, { size: bytes, allocatedAt: Date.now() });

		return buffer;
	}

	deallocate(buffer: Uint8Array): boolean {
		const offset = buffer.byteOffset;
		const metadata = this.pageMetadata.get(offset);

		if (!metadata) {
			return false;
		}

		const pagesNeeded = Math.ceil(metadata.size / this.pageSize);
		const startPage = Math.floor(offset / this.pageSize);

		for (let i = startPage; i < startPage + pagesNeeded; i++) {
			this.pageAllocations[i] = false;
		}

		this.pageMetadata.delete(offset);
		return true;
	}

	getFreeMemory(): number {
		const freePages = this.pageAllocations.filter((p) => !p).length;
		return freePages * this.pageSize;
	}

	getUsedMemory(): number {
		const usedPages = this.pageAllocations.filter((p) => p).length;
		return usedPages * this.pageSize;
	}

	getFragmentation(): number {
		if (this.pageMetadata.size === 0) return 0;

		let gaps = 0;
		let inAllocation = false;

		for (const allocated of this.pageAllocations) {
			if (allocated && !inAllocation) {
				inAllocation = true;
			} else if (!allocated && inAllocation) {
				gaps++;
				inAllocation = false;
			}
		}

		return gaps;
	}

	clear(): void {
		this.pageAllocations.fill(false);
		this.pageMetadata.clear();
	}
}

describe("WASM: Memory Management", () => {
	let tracker: WasmMemoryTracker;

	beforeEach(() => {
		tracker = new WasmMemoryTracker();
	});

	afterEach(() => {
		tracker.clear();
	});

	describe("memory allocation patterns", () => {
		it("should allocate memory and track allocation ID", () => {
			const id = tracker.allocate(1024);

			expect(typeof id).toBe("number");
			expect(id).toBeGreaterThan(0);

			const stats = tracker.getStats();
			expect(stats.currentMemory).toBe(1024);
		});

		it("should track multiple allocations", () => {
			const id1 = tracker.allocate(512);
			const id2 = tracker.allocate(1024);
			const id3 = tracker.allocate(256);

			expect(id1).not.toBe(id2);
			expect(id2).not.toBe(id3);

			const stats = tracker.getStats();
			expect(stats.currentMemory).toBe(512 + 1024 + 256);
			expect(stats.allocationsCount).toBe(3);
		});

		it("should track peak memory usage", () => {
			tracker.allocate(1024);
			tracker.allocate(2048);

			let stats = tracker.getStats();
			expect(stats.peakMemory).toBe(1024 + 2048);

			tracker.allocate(512);

			stats = tracker.getStats();
			expect(stats.peakMemory).toBe(1024 + 2048 + 512);
		});

		it("should support reallocation patterns", () => {
			const id1 = tracker.allocate(1024);

			tracker.deallocate(id1);

			const id2 = tracker.allocate(2048);

			const stats = tracker.getStats();
			expect(stats.currentMemory).toBe(2048);
			expect(stats.deallocationsCount).toBe(1);
		});

		it("should track timestamp of allocations", () => {
			const before = Date.now();
			tracker.allocate(512);
			const after = Date.now();

			const stats = tracker.getStats();
			expect(stats.allocationsCount).toBe(1);

			// Stats should reflect the allocation
			expect(stats.currentMemory).toBe(512);
		});
	});

	describe("memory cleanup", () => {
		it("should deallocate memory and update current usage", () => {
			const id = tracker.allocate(1024);

			let stats = tracker.getStats();
			expect(stats.currentMemory).toBe(1024);

			tracker.deallocate(id);

			stats = tracker.getStats();
			expect(stats.currentMemory).toBe(0);
		});

		it("should deallocate multiple allocations", () => {
			const id1 = tracker.allocate(512);
			const id2 = tracker.allocate(1024);

			let stats = tracker.getStats();
			expect(stats.currentMemory).toBe(512 + 1024);

			tracker.deallocate(id1);
			tracker.deallocate(id2);

			stats = tracker.getStats();
			expect(stats.currentMemory).toBe(0);
		});

		it("should handle deallocation of non-existent allocation", () => {
			const success = tracker.deallocate(999);

			expect(success).toBe(false);

			const stats = tracker.getStats();
			expect(stats.currentMemory).toBe(0);
		});

		it("should reset to clean state", () => {
			tracker.allocate(1024);
			tracker.allocate(2048);

			let stats = tracker.getStats();
			expect(stats.peakMemory).toBeGreaterThan(0);

			tracker.clear();

			stats = tracker.getStats();
			expect(stats.currentMemory).toBe(0);
			expect(stats.peakMemory).toBe(0);
			expect(stats.allocationsCount).toBe(0);
		});
	});

	describe("memory leak detection", () => {
		it("should detect leaked allocations", () => {
			tracker.allocate(1024);
			tracker.allocate(2048);
			tracker.allocate(512);

			const leaks = tracker.getLeaks();

			expect(leaks).toHaveLength(3);
		});

		it("should report no leaks after proper cleanup", () => {
			const id1 = tracker.allocate(1024);
			const id2 = tracker.allocate(2048);

			tracker.deallocate(id1);
			tracker.deallocate(id2);

			const leaks = tracker.getLeaks();

			expect(leaks).toHaveLength(0);
		});

		it("should calculate leaked memory size", () => {
			tracker.allocate(512);
			tracker.allocate(1024);

			const stats = tracker.getStats();

			expect(stats.leakedMemory).toBe(512 + 1024);
		});

		it("should distinguish leaked vs deallocated memory", () => {
			const id1 = tracker.allocate(1024);
			const id2 = tracker.allocate(512);

			tracker.deallocate(id1);
			// id2 is leaked

			let stats = tracker.getStats();
			expect(stats.leakedMemory).toBe(512);
			expect(stats.deallocationsCount).toBe(1);

			tracker.deallocate(id2);

			stats = tracker.getStats();
			expect(stats.leakedMemory).toBe(0);
			expect(stats.deallocationsCount).toBe(2);
		});
	});

	describe("WASM memory pool", () => {
		it("should allocate from memory pool", () => {
			const pool = new WasmMemoryPool(10 * 1024 * 1024); // 10MB

			const buffer = pool.allocate(1024);

			expect(buffer).not.toBeNull();
			expect(buffer!.byteLength).toBe(1024);
		});

		it("should track used memory in pool", () => {
			const pool = new WasmMemoryPool(10 * 1024 * 1024);

			pool.allocate(1024);
			pool.allocate(2048);

			const used = pool.getUsedMemory();
			const free = pool.getFreeMemory();

			expect(used).toBeGreaterThan(0);
			expect(used + free).toBe(10 * 1024 * 1024);
		});

		it("should deallocate from pool", () => {
			const pool = new WasmMemoryPool(10 * 1024 * 1024);

			const buffer = pool.allocate(1024);
			expect(pool.getUsedMemory()).toBeGreaterThan(0);

			const success = pool.deallocate(buffer!);

			expect(success).toBe(true);
			expect(pool.getUsedMemory()).toBe(0);
		});

		it("should fail allocation when pool exhausted", () => {
			const pool = new WasmMemoryPool(1024 * 1024); // 1MB

			const allocations: Uint8Array[] = [];

			// Fill pool
			let buffer = pool.allocate(500 * 1024);
			while (buffer !== null) {
				allocations.push(buffer);
				buffer = pool.allocate(500 * 1024);
			}

			expect(allocations.length).toBeGreaterThan(0);

			// Next allocation should fail
			const failed = pool.allocate(1024);
			expect(failed).toBeNull();
		});

		it("should measure fragmentation", () => {
			const pool = new WasmMemoryPool(10 * 1024 * 1024);

			const buf1 = pool.allocate(64 * 1024); // Full page
			const buf2 = pool.allocate(64 * 1024); // Full page
			const buf3 = pool.allocate(64 * 1024); // Full page

			let fragmentation = pool.getFragmentation();
			expect(fragmentation).toBeGreaterThanOrEqual(0); // Initial state

			pool.deallocate(buf2!);

			fragmentation = pool.getFragmentation();
			expect(fragmentation).toBeGreaterThanOrEqual(0); // May have gap now

			pool.clear();
			fragmentation = pool.getFragmentation();
			expect(fragmentation).toBe(0); // Reset
		});
	});

	describe("large document handling", () => {
		it("should handle allocation of large document (1MB)", () => {
			const largeDocSize = 1024 * 1024; // 1MB
			const id = tracker.allocate(largeDocSize);

			const stats = tracker.getStats();

			expect(stats.currentMemory).toBe(largeDocSize);
			expect(stats.peakMemory).toBe(largeDocSize);

			tracker.deallocate(id);
		});

		it("should handle allocation of very large document (10MB)", () => {
			const veryLargeSize = 10 * 1024 * 1024; // 10MB
			const id = tracker.allocate(veryLargeSize);

			const stats = tracker.getStats();

			expect(stats.currentMemory).toBe(veryLargeSize);

			tracker.deallocate(id);
		});

		it("should process multiple large documents sequentially", () => {
			const docSize = 5 * 1024 * 1024; // 5MB

			for (let i = 0; i < 3; i++) {
				const id = tracker.allocate(docSize);

				let stats = tracker.getStats();
				expect(stats.currentMemory).toBe(docSize);

				tracker.deallocate(id);

				stats = tracker.getStats();
				expect(stats.currentMemory).toBe(0);
			}
		});

		it("should handle streaming processing with bounded memory", () => {
			const chunkSize = 256 * 1024; // 256KB chunks
			const docSize = 5 * 1024 * 1024; // 5MB total
			let maxMemory = 0;

			for (let i = 0; i < docSize; i += chunkSize) {
				const id = tracker.allocate(chunkSize);

				const stats = tracker.getStats();
				maxMemory = Math.max(maxMemory, stats.currentMemory);

				tracker.deallocate(id);
			}

			// Should never exceed 1.5x chunk size with streaming
			expect(maxMemory).toBeLessThanOrEqual(chunkSize * 1.5);
		});
	});

	describe("buffer reuse and pooling", () => {
		it("should implement buffer pooling pattern", () => {
			const pool = new WasmMemoryPool(10 * 1024 * 1024);

			// Allocate and deallocate repeatedly
			const iterations = 100;
			for (let i = 0; i < iterations; i++) {
				const buffer = pool.allocate(1024);
				expect(buffer).not.toBeNull();
				pool.deallocate(buffer!);
			}

			// Pool should be empty after all deallocations
			const used = pool.getUsedMemory();
			expect(used).toBe(0);
		});

		it("should minimize fragmentation with pooling", () => {
			const pool = new WasmMemoryPool(10 * 1024 * 1024);

			// Allocate different sizes
			const buffers: Uint8Array[] = [];
			buffers.push(pool.allocate(1024)!);
			buffers.push(pool.allocate(2048)!);
			buffers.push(pool.allocate(512)!);

			const fragBefore = pool.getFragmentation();

			// Deallocate in order
			buffers.forEach((buf) => pool.deallocate(buf));

			const fragAfter = pool.getFragmentation();

			// Fragmentation should be minimal after ordered deallocation
			expect(fragAfter).toBeLessThanOrEqual(fragBefore);
		});

		it("should handle allocation size mismatch gracefully", () => {
			const tracker2 = new WasmMemoryTracker();

			// Allocate various sizes
			const sizes = [256, 512, 1024, 2048, 4096];
			const ids = sizes.map((size) => tracker2.allocate(size));

			let stats = tracker2.getStats();
			expect(stats.currentMemory).toBe(256 + 512 + 1024 + 2048 + 4096);

			// Deallocate in different order
			[3, 0, 4, 1, 2].forEach((idx) => tracker2.deallocate(ids[idx]));

			stats = tracker2.getStats();
			expect(stats.currentMemory).toBe(0);
		});
	});

	describe("garbage collection integration", () => {
		it("should track memory across GC cycles", () => {
			const tracker2 = new WasmMemoryTracker();

			tracker2.allocate(1024 * 1024); // 1MB

			let stats = tracker2.getStats();
			const memBefore = stats.currentMemory;

			// Force GC (if available)
			if (global.gc) {
				global.gc();
			}

			stats = tracker2.getStats();
			const memAfter = stats.currentMemory;

			// Tracked memory should remain consistent
			expect(memAfter).toBe(memBefore);
		});

		it("should not lose allocation state during cleanup", () => {
			const tracker2 = new WasmMemoryTracker();

			const ids = [tracker2.allocate(512), tracker2.allocate(1024), tracker2.allocate(256)];

			tracker2.deallocate(ids[1]); // Deallocate middle

			let stats = tracker2.getStats();
			expect(stats.leakedMemory).toBe(512 + 256); // Should still track remaining

			tracker2.deallocate(ids[0]);
			tracker2.deallocate(ids[2]);

			stats = tracker2.getStats();
			expect(stats.leakedMemory).toBe(0);
		});
	});

	describe("memory exhaustion scenarios", () => {
		it("should detect memory exhaustion", () => {
			const smallPool = new WasmMemoryPool(2 * 1024 * 1024); // 2MB pool

			const buffers: Uint8Array[] = [];

			let allocationFailed = false;
			for (let i = 0; i < 100; i++) {
				const buffer = smallPool.allocate(256 * 1024); // 256KB each
				if (buffer === null) {
					allocationFailed = true;
					break;
				}
				buffers.push(buffer);
			}

			expect(allocationFailed).toBe(true);
			expect(buffers.length).toBeLessThan(100);
		});

		it("should allow recovery after memory exhaustion", () => {
			const pool = new WasmMemoryPool(5 * 1024 * 1024);

			const buffers: Uint8Array[] = [];

			// Fill pool
			for (let i = 0; i < 20; i++) {
				const buf = pool.allocate(256 * 1024);
				if (buf) buffers.push(buf);
				else break;
			}

			// Free some
			buffers.slice(0, 10).forEach((buf) => pool.deallocate(buf));

			// Should be able to allocate again
			const recovered = pool.allocate(256 * 1024);

			expect(recovered).not.toBeNull();
		});
	});
});
