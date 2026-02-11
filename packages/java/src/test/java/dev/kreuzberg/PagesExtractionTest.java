package dev.kreuzberg;

import static org.junit.jupiter.api.Assertions.*;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.PropertyNamingStrategies;
import dev.kreuzberg.config.PageConfig;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import org.junit.jupiter.api.Test;

/**
 * Comprehensive tests for document page extraction functionality. Tests cover
 * page extraction, page marker insertion, custom marker formats, and page
 * content validation.
 *
 * <p>
 * Note: These tests use mock extraction since actual PDF files are not
 * available in unit tests. Real integration tests should be run separately with
 * actual document files.
 */
class PagesExtractionTest {

	// Helper method to count occurrences of a pattern in text
	private int countMatches(String text, String pattern) {
		if (text == null || text.isEmpty()) {
			return 0;
		}
		Pattern p = Pattern.compile(pattern);
		Matcher m = p.matcher(text);
		int count = 0;
		while (m.find()) {
			count++;
		}
		return count;
	}

	// ============= Extract Pages Configuration Tests =============

	@Test
	void testPageConfig_DefaultValues() {
		PageConfig config = PageConfig.builder().build();

		assertFalse(config.isExtractPages(), "ExtractPages should default to false");
		assertFalse(config.isInsertPageMarkers(), "InsertPageMarkers should default to false");
		assertNotNull(config.getMarkerFormat(), "MarkerFormat should have default value");
	}

	@Test
	void testPageConfig_AcceptsCustomValues() {
		PageConfig config = PageConfig.builder().extractPages(true).insertPageMarkers(true)
				.markerFormat("CUSTOM_{page_num}").build();

		assertTrue(config.isExtractPages(), "ExtractPages should be true");
		assertTrue(config.isInsertPageMarkers(), "InsertPageMarkers should be true");
		assertEquals("CUSTOM_{page_num}", config.getMarkerFormat(), "MarkerFormat should match custom value");
	}

	@Test
	void testPageConfig_ToMap_ContainsAllFields() {
		PageConfig config = PageConfig.builder().extractPages(true).insertPageMarkers(true)
				.markerFormat("TEST_{page_num}").build();

		java.util.Map<String, Object> map = config.toMap();

		assertNotNull(map, "Map should not be null");
		assertTrue(map.containsKey("extract_pages"), "Map should contain extract_pages");
		assertTrue(map.containsKey("insert_page_markers"), "Map should contain insert_page_markers");
		assertTrue(map.containsKey("marker_format"), "Map should contain marker_format");
	}

	@Test
	void testPageConfig_WithMarkerFormat() {
		PageConfig config = PageConfig.builder().markerFormat("[PAGE {page_num}]").build();

		assertEquals("[PAGE {page_num}]", config.getMarkerFormat());
	}

	// ============= Page Info Metadata Tests =============

	@Test
	void testPageInfo_HasPageNumber() {
		PageInfo page = new PageInfo(1L, "Page 1", new double[]{612.0, 792.0}, null, null, false, null);

		assertEquals(1L, page.getNumber(), "Page number should be 1");
		assertTrue(page.getNumber() > 0, "Page number should be positive");
	}

	@Test
	void testPageInfo_WithTitle() {
		PageInfo page = new PageInfo(1L, "Introduction", new double[]{612.0, 792.0}, null, null, false, null);

		var title = page.getTitle();
		assertTrue(title.isPresent(), "Page should have title");
		assertEquals("Introduction", title.get(), "Title should match");
	}

	@Test
	void testPageInfo_WithDimensions() {
		PageInfo page = new PageInfo(1L, null, new double[]{612.0, 792.0}, null, null, false, null);

		var width = page.getWidth();
		var height = page.getHeight();

		assertTrue(width.isPresent(), "Page should have width");
		assertTrue(height.isPresent(), "Page should have height");
		assertEquals(612.0, width.get(), "Width should match");
		assertEquals(792.0, height.get(), "Height should match");
	}

	@Test
	void testPageInfo_Visibility() {
		PageInfo visiblePage = new PageInfo(1L, null, null, null, null, false, null);
		PageInfo hiddenPage = new PageInfo(2L, null, null, null, null, true, null);

		var visibility1 = visiblePage.isVisible();
		var visibility2 = hiddenPage.isVisible();

		assertTrue(visibility1.isPresent(), "Visible page should have visibility set");
		assertTrue(visibility1.get(), "Page should be visible");
		assertTrue(visibility2.isPresent(), "Hidden page should have visibility set");
		assertFalse(visibility2.get(), "Page should be hidden");
	}

	@Test
	void testPageInfo_WithoutOptionalFields() {
		PageInfo page = new PageInfo(5L, null, null, null, null, null, null);

		assertEquals(5L, page.getNumber());
		assertTrue(page.getTitle().isEmpty(), "Title should be empty");
		assertTrue(page.getWidth().isEmpty(), "Width should be empty");
		assertTrue(page.getHeight().isEmpty(), "Height should be empty");
		assertTrue(page.isHidden().isEmpty(), "Hidden status should be empty");
	}

	@Test
	void testPageInfo_InvalidPageNumber_ThrowsException() {
		assertThrows(IllegalArgumentException.class, () -> {
			new PageInfo(0L, null, null, null, null, null, null);
		}, "Page number must be positive");

		assertThrows(IllegalArgumentException.class, () -> {
			new PageInfo(-1L, null, null, null, null, null, null);
		}, "Page number must be positive");
	}

	@Test
	void testPageInfo_Equality() {
		PageInfo page1 = new PageInfo(1L, "Title", new double[]{612.0, 792.0}, null, null, false, null);
		PageInfo page2 = new PageInfo(1L, "Title", new double[]{612.0, 792.0}, null, null, false, null);
		PageInfo page3 = new PageInfo(2L, "Title", new double[]{612.0, 792.0}, null, null, false, null);

		assertEquals(page1, page2, "Pages with same values should be equal");
		assertNotEquals(page1, page3, "Pages with different numbers should not be equal");
	}

	@Test
	void testPageInfo_HashCode() {
		PageInfo page1 = new PageInfo(1L, "Title", new double[]{612.0, 792.0}, null, null, false, null);
		PageInfo page2 = new PageInfo(1L, "Title", new double[]{612.0, 792.0}, null, null, false, null);

		assertEquals(page1.hashCode(), page2.hashCode(), "Equal pages should have equal hash codes");
	}

	@Test
	void testPageInfo_IsBlank() {
		PageInfo blankPage = new PageInfo(1L, null, null, null, null, false, true);
		PageInfo nonBlankPage = new PageInfo(2L, null, null, null, null, false, false);
		PageInfo unknownBlankPage = new PageInfo(3L, null, null, null, null, false, null);

		assertTrue(blankPage.isBlank().isPresent(), "isBlank should be present for blank page");
		assertTrue(blankPage.isBlank().get(), "Page should be blank");

		assertTrue(nonBlankPage.isBlank().isPresent(), "isBlank should be present for non-blank page");
		assertFalse(nonBlankPage.isBlank().get(), "Page should not be blank");

		assertTrue(unknownBlankPage.isBlank().isEmpty(), "isBlank should be empty when null");
	}

	@Test
	void testPageContent_GetIsBlank() {
		PageContent blankContent = new PageContent(1, "", List.of(), List.of(), null, true);
		PageContent nonBlankContent = new PageContent(2, "Some text", List.of(), List.of(), null, false);
		PageContent unknownContent = new PageContent(3, "Text", List.of(), List.of(), null, null);

		assertTrue(blankContent.getIsBlank().isPresent(), "getIsBlank should be present for blank page");
		assertTrue(blankContent.getIsBlank().get(), "Page content should be blank");

		assertTrue(nonBlankContent.getIsBlank().isPresent(), "getIsBlank should be present for non-blank page");
		assertFalse(nonBlankContent.getIsBlank().get(), "Page content should not be blank");

		assertTrue(unknownContent.getIsBlank().isEmpty(), "getIsBlank should be empty when null");
	}

	// ============= Page Structure Tests =============

	@Test
	void testPageStructure_WithPages() {
		List<PageInfo> pages = List.of(new PageInfo(1L, "Page 1", new double[]{612.0, 792.0}, null, null, false, null),
				new PageInfo(2L, "Page 2", new double[]{612.0, 792.0}, null, null, false, null));

		PageStructure structure = new PageStructure(2L, PageUnitType.PAGE, null, pages);

		assertEquals(2L, structure.getTotalCount(), "Total count should match");
		assertEquals(PageUnitType.PAGE, structure.getUnitType(), "Unit type should be PAGE");

		var pagesOpt = structure.getPages();
		assertTrue(pagesOpt.isPresent(), "Pages should be present");
		assertEquals(2, pagesOpt.get().size(), "Should have 2 pages");
	}

	@Test
	void testPageStructure_WithoutPages() {
		PageStructure structure = new PageStructure(3L, PageUnitType.SLIDE, null, null);

		assertEquals(3L, structure.getTotalCount());
		assertEquals(PageUnitType.SLIDE, structure.getUnitType());
		assertTrue(structure.getPages().isEmpty(), "Pages should be empty when not provided");
	}

	@Test
	void testPageStructure_WithBoundaries() {
		List<PageBoundary> boundaries = List.of(new PageBoundary(0L, 1000L, 1L), new PageBoundary(1000L, 2000L, 2L));

		PageStructure structure = new PageStructure(2L, PageUnitType.PAGE, boundaries, null);

		var boundariesOpt = structure.getBoundaries();
		assertTrue(boundariesOpt.isPresent(), "Boundaries should be present");
		assertEquals(2, boundariesOpt.get().size(), "Should have 2 boundaries");
	}

	// ============= Page Configuration Integration Tests =============

	@Test
	void testPageConfig_BuilderChain() {
		PageConfig config = PageConfig.builder().extractPages(true).insertPageMarkers(true)
				.markerFormat("[PAGE {page_num}]").build();

		assertTrue(config.isExtractPages());
		assertTrue(config.isInsertPageMarkers());
		assertEquals("[PAGE {page_num}]", config.getMarkerFormat());
	}

	@Test
	void testPageConfig_PartialConfiguration() {
		PageConfig config1 = PageConfig.builder().extractPages(true).build();

		assertTrue(config1.isExtractPages());
		assertFalse(config1.isInsertPageMarkers(), "InsertPageMarkers should default to false");
	}

	@Test
	void testPageUnitType_Values() {
		assertEquals(PageUnitType.PAGE, PageUnitType.PAGE);
		assertEquals(PageUnitType.SLIDE, PageUnitType.SLIDE);
		assertEquals(PageUnitType.SHEET, PageUnitType.SHEET);

		assertNotEquals(PageUnitType.PAGE, PageUnitType.SLIDE);
		assertNotEquals(PageUnitType.SLIDE, PageUnitType.SHEET);
	}

	// ============= Page Boundary Tests =============

	@Test
	void testPageBoundary_Creation() {
		PageBoundary boundary = new PageBoundary(0L, 500L, 1L);

		assertEquals(1L, boundary.pageNumber());
		assertEquals(0L, boundary.byteStart());
		assertEquals(500L, boundary.byteEnd());
	}

	@Test
	void testPageBoundary_OrderedBoundaries() {
		List<PageBoundary> boundaries = List.of(new PageBoundary(0L, 500L, 1L), new PageBoundary(500L, 1000L, 2L),
				new PageBoundary(1000L, 1500L, 3L));

		for (int i = 0; i < boundaries.size() - 1; i++) {
			PageBoundary current = boundaries.get(i);
			PageBoundary next = boundaries.get(i + 1);
			assertTrue(current.byteEnd() <= next.byteStart(), "Page boundaries should be contiguous or overlapping");
		}
	}

	// ============= JSON Deserialization Tests (Regression for #355) =============

	@Test
	void testPageStructure_JsonDeserialization_WithBoundaries() throws Exception {
		// Regression test for issue #355: ClassCastException when deserializing
		// PageStructure with nested List<PageBoundary>
		ObjectMapper mapper = new ObjectMapper().setPropertyNamingStrategy(PropertyNamingStrategies.SNAKE_CASE)
				.configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);

		String json = """
				{
					"total_count": 3,
					"unit_type": "page",
					"boundaries": [
						{"byte_start": 0, "byte_end": 500, "page_number": 1},
						{"byte_start": 500, "byte_end": 1000, "page_number": 2},
						{"byte_start": 1000, "byte_end": 1500, "page_number": 3}
					],
					"pages": null
				}
				""";

		PageStructure structure = mapper.readValue(json, new TypeReference<PageStructure>() {
		});

		assertEquals(3L, structure.getTotalCount());
		assertEquals(PageUnitType.PAGE, structure.getUnitType());

		var boundariesOpt = structure.getBoundaries();
		assertTrue(boundariesOpt.isPresent(), "Boundaries should be present");

		List<PageBoundary> boundaries = boundariesOpt.get();
		assertEquals(3, boundaries.size());

		// This would throw ClassCastException before the fix if elements were
		// LinkedHashMap
		PageBoundary first = boundaries.get(0);
		assertEquals(0L, first.byteStart());
		assertEquals(500L, first.byteEnd());
		assertEquals(1L, first.pageNumber());

		PageBoundary last = boundaries.get(2);
		assertEquals(1000L, last.byteStart());
		assertEquals(1500L, last.byteEnd());
		assertEquals(3L, last.pageNumber());
	}

	@Test
	void testPageStructure_JsonDeserialization_WithPages() throws Exception {
		// Regression test for issue #355: ClassCastException when deserializing
		// PageStructure with nested List<PageInfo>
		ObjectMapper mapper = new ObjectMapper().setPropertyNamingStrategy(PropertyNamingStrategies.SNAKE_CASE)
				.configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);

		String json = """
				{
					"total_count": 2,
					"unit_type": "slide",
					"boundaries": null,
					"pages": [
						{"number": 1, "title": "Introduction", "dimensions": [1920.0, 1080.0], "hidden": false},
						{"number": 2, "title": "Content", "dimensions": [1920.0, 1080.0], "hidden": true}
					]
				}
				""";

		PageStructure structure = mapper.readValue(json, new TypeReference<PageStructure>() {
		});

		assertEquals(2L, structure.getTotalCount());
		assertEquals(PageUnitType.SLIDE, structure.getUnitType());

		var pagesOpt = structure.getPages();
		assertTrue(pagesOpt.isPresent(), "Pages should be present");

		List<PageInfo> pages = pagesOpt.get();
		assertEquals(2, pages.size());

		// This would throw ClassCastException before the fix if elements were
		// LinkedHashMap
		PageInfo first = pages.get(0);
		assertEquals(1L, first.getNumber());
		assertEquals("Introduction", first.getTitle().orElse(null));
		assertTrue(first.isVisible().orElse(false));

		PageInfo second = pages.get(1);
		assertEquals(2L, second.getNumber());
		assertEquals("Content", second.getTitle().orElse(null));
		assertFalse(second.isVisible().orElse(true));
	}

	@Test
	void testPageStructure_JsonDeserialization_WithBothBoundariesAndPages() throws Exception {
		// Regression test for issue #355: Full PageStructure with both nested lists
		ObjectMapper mapper = new ObjectMapper().setPropertyNamingStrategy(PropertyNamingStrategies.SNAKE_CASE)
				.configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);

		String json = """
				{
					"total_count": 2,
					"unit_type": "page",
					"boundaries": [
						{"byte_start": 0, "byte_end": 1000, "page_number": 1},
						{"byte_start": 1000, "byte_end": 2000, "page_number": 2}
					],
					"pages": [
						{"number": 1, "title": "Page 1", "dimensions": [612.0, 792.0], "image_count": 2, "table_count": 1, "hidden": false},
						{"number": 2, "title": "Page 2", "dimensions": [612.0, 792.0], "image_count": 0, "table_count": 0, "hidden": false}
					]
				}
				""";

		PageStructure structure = mapper.readValue(json, new TypeReference<PageStructure>() {
		});

		assertEquals(2L, structure.getTotalCount());
		assertEquals(PageUnitType.PAGE, structure.getUnitType());

		// Verify boundaries
		var boundariesOpt = structure.getBoundaries();
		assertTrue(boundariesOpt.isPresent());
		List<PageBoundary> boundaries = boundariesOpt.get();
		assertEquals(2, boundaries.size());
		assertEquals(1L, boundaries.get(0).pageNumber());
		assertEquals(2L, boundaries.get(1).pageNumber());

		// Verify pages
		var pagesOpt = structure.getPages();
		assertTrue(pagesOpt.isPresent());
		List<PageInfo> pages = pagesOpt.get();
		assertEquals(2, pages.size());

		PageInfo page1 = pages.get(0);
		assertEquals(1L, page1.getNumber());
		assertEquals(2, page1.getImageCount().orElse(0));
		assertEquals(1, page1.getTableCount().orElse(0));

		PageInfo page2 = pages.get(1);
		assertEquals(2L, page2.getNumber());
		assertEquals(0, page2.getImageCount().orElse(-1));
		assertEquals(0, page2.getTableCount().orElse(-1));
	}
}
