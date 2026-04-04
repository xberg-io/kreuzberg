// Hand-written binding-specific edge case tests for PDF rendering.
// Happy-path render tests are auto-generated from fixtures in e2e/.
// These tests cover error handling, validation, and lifecycle patterns
// that vary per language and can't be generated uniformly.

package dev.kreuzberg;

import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import org.junit.jupiter.api.Test;

class RenderTest {

	private Path getTestPdf() {
		Path repoRoot = Path.of("").toAbsolutePath();
		// Walk up to find test_documents
		Path current = repoRoot;
		while (current != null) {
			Path candidate = current.resolve("test_documents/pdf/tiny.pdf");
			if (Files.exists(candidate)) {
				return candidate;
			}
			current = current.getParent();
		}
		return repoRoot.resolve("test_documents/pdf/tiny.pdf");
	}

	@Test
	void testRenderingMethodsExist() throws Exception {
		// Verify methods exist via reflection
		assertNotNull(Kreuzberg.class.getMethod("renderPdfPage", Path.class, int.class, int.class));
	}

	@Test
	void testRenderPdfPageNonexistentFile() {
		Path nonexistent = Path.of("/nonexistent/path/to/document.pdf");
		assertThrows(IOException.class, () -> {
			Kreuzberg.renderPdfPage(nonexistent, 0, 150);
		}, "Should throw IOException for nonexistent file");
	}

	@Test
	void testRenderPdfPageIndexOutOfBounds() {
		Path testPdf = getTestPdf();
		assumeTrue(Files.exists(testPdf),
				"Test PDF not found: " + testPdf.toAbsolutePath());

		assertThrows(Exception.class, () -> {
			Kreuzberg.renderPdfPage(testPdf, 9999, 150);
		}, "Should throw for out-of-bounds page index");
	}

	@Test
	void testRenderPdfPageNegativeIndex() {
		Path testPdf = getTestPdf();
		assumeTrue(Files.exists(testPdf),
				"Test PDF not found: " + testPdf.toAbsolutePath());

		assertThrows(IllegalArgumentException.class, () -> {
			Kreuzberg.renderPdfPage(testPdf, -1, 150);
		}, "Should throw IllegalArgumentException for negative page index");
	}

	@Test
	void testPdfPageIteratorClose() throws Exception {
		Path testPdf = getTestPdf();
		assumeTrue(Files.exists(testPdf),
				"Test PDF not found: " + testPdf.toAbsolutePath());

		var iter = Kreuzberg.PdfPageIterator.open(testPdf, 150);
		iter.close();
		// Double close should be safe
		iter.close();
		// After close, pageCount returns 0
		assertEquals(0, iter.pageCount(), "pageCount after close should be 0");
		// After close, hasNext returns false
		assertFalse(iter.hasNext(), "hasNext after close should be false");
	}

	@Test
	void testPdfPageIteratorNonexistentFile() {
		Path nonexistent = Path.of("/nonexistent/path/to/document.pdf");
		assertThrows(IOException.class, () -> {
			Kreuzberg.PdfPageIterator.open(nonexistent, 150);
		}, "Should throw IOException for nonexistent file");
	}

	@Test
	void testRenderPdfPageEmptyPath() {
		Path emptyPath = Path.of("");
		assertThrows(Exception.class, () -> {
			Kreuzberg.renderPdfPage(emptyPath, 0, 150);
		}, "Should throw for empty path");
	}

	@Test
	void testPdfPageIteratorEarlyTermination() throws Exception {
		Path testPdf = getTestPdf();
		assumeTrue(Files.exists(testPdf),
				"Test PDF not found: " + testPdf.toAbsolutePath());

		var iter = Kreuzberg.PdfPageIterator.open(testPdf, 150);
		assertTrue(iter.hasNext(), "iterator should have at least one page");
		var page = iter.next();
		assertEquals(0, page.pageIndex(), "first page index should be 0");
		byte[] data = page.data();
		assertTrue(data.length > 8, "PNG data should be longer than 8 bytes");
		// PNG magic bytes: 89 50 4E 47 0D 0A 1A 0A
		assertEquals((byte) 0x89, data[0]);
		assertEquals((byte) 0x50, data[1]);
		assertEquals((byte) 0x4E, data[2]);
		assertEquals((byte) 0x47, data[3]);
		// Close without exhausting the iterator
		iter.close();
	}
}
