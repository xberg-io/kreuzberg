package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import java.util.Collections;
import java.util.List;
import java.util.Optional;

/**
 * Content for a single page/slide.
 *
 * <p>
 * When page extraction is enabled, documents are split into per-page content
 * with associated tables and images mapped to each page.
 *
 * @param pageNumber
 *            the page number (1-indexed)
 * @param content
 *            the text content for this page
 * @param tables
 *            tables found on this page
 * @param images
 *            images found on this page
 * @param hierarchy
 *            hierarchy information for the page, or null
 * @param isBlank
 *            whether this page is blank, or null
 */
public record PageContent(@JsonProperty("page_number") int pageNumber, @JsonProperty("content") String content,
		@JsonDeserialize(contentAs = Table.class) @JsonProperty("tables") List<Table> tables,
		@JsonDeserialize(contentAs = ExtractedImage.class) @JsonProperty("images") List<ExtractedImage> images,
		@JsonProperty("hierarchy") PageHierarchy hierarchy, @JsonProperty("is_blank") Boolean isBlank) {
	@JsonCreator
	public PageContent(@JsonProperty("page_number") int pageNumber, @JsonProperty("content") String content,
			@JsonProperty("tables") List<Table> tables, @JsonProperty("images") List<ExtractedImage> images,
			@JsonProperty("hierarchy") PageHierarchy hierarchy, @JsonProperty("is_blank") Boolean isBlank) {
		this.pageNumber = pageNumber;
		this.content = content != null ? content : "";
		this.tables = tables != null ? Collections.unmodifiableList(tables) : List.of();
		this.images = images != null ? Collections.unmodifiableList(images) : List.of();
		this.hierarchy = hierarchy;
		this.isBlank = isBlank;
	}

	/**
	 * Get the hierarchy information for this page.
	 *
	 * @return hierarchy, or empty if not available
	 */
	public Optional<PageHierarchy> getHierarchy() {
		return Optional.ofNullable(hierarchy);
	}

	/**
	 * Get whether this page is blank (contains no meaningful content).
	 *
	 * @return true if blank, false otherwise, empty if not applicable
	 */
	public Optional<Boolean> getIsBlank() {
		return Optional.ofNullable(isBlank);
	}
}
