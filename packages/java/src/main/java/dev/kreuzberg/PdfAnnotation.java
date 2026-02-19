package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;

/**
 * Represents an annotation extracted from a PDF document.
 *
 * <p>
 * PDF annotations include comments, highlights, links, stamps, and other markup
 * added to a PDF document. Each annotation has a type, optional content text,
 * page number, and optional bounding box.
 *
 * @param annotationType
 *            the type of annotation (e.g., "text", "highlight", "link",
 *            "stamp", "underline", "strike_out", "other")
 * @param content
 *            the text content of the annotation, or null if not available
 * @param pageNumber
 *            the page number where the annotation appears (1-indexed)
 * @param boundingBox
 *            the bounding box coordinates of the annotation on the page, or
 *            null if not available
 */
public record PdfAnnotation(@JsonProperty("annotation_type") String annotationType,
		@JsonProperty("content") String content, @JsonProperty("page_number") int pageNumber,
		@JsonProperty("bounding_box") PdfAnnotationBoundingBox boundingBox) {

	/**
	 * Creates a new PdfAnnotation.
	 *
	 * @param annotationType
	 *            the type of annotation (must not be null)
	 * @param content
	 *            the text content, or null if not available
	 * @param pageNumber
	 *            the page number (must be >= 1)
	 * @param boundingBox
	 *            the bounding box, or null if not available
	 * @throws NullPointerException
	 *             if annotationType is null
	 * @throws IllegalArgumentException
	 *             if pageNumber is less than 1
	 */
	@JsonCreator
	public PdfAnnotation(@JsonProperty("annotation_type") String annotationType,
			@JsonProperty("content") String content, @JsonProperty("page_number") int pageNumber,
			@JsonProperty("bounding_box") PdfAnnotationBoundingBox boundingBox) {
		if (annotationType == null) {
			throw new NullPointerException("annotationType must not be null");
		}
		if (pageNumber < 1) {
			throw new IllegalArgumentException("pageNumber must be >= 1, got " + pageNumber);
		}
		this.annotationType = annotationType;
		this.content = content;
		this.pageNumber = pageNumber;
		this.boundingBox = boundingBox;
	}

	/**
	 * Bounding box for a PDF annotation (PDF coordinates).
	 *
	 * @param x0
	 *            the left x-coordinate
	 * @param y0
	 *            the bottom y-coordinate
	 * @param x1
	 *            the right x-coordinate
	 * @param y1
	 *            the top y-coordinate
	 */
	public record PdfAnnotationBoundingBox(@JsonProperty("x0") double x0, @JsonProperty("y0") double y0,
			@JsonProperty("x1") double x1, @JsonProperty("y1") double y1) {

		@JsonCreator
		public PdfAnnotationBoundingBox(@JsonProperty("x0") double x0, @JsonProperty("y0") double y0,
				@JsonProperty("x1") double x1, @JsonProperty("y1") double y1) {
			this.x0 = x0;
			this.y0 = y0;
			this.x1 = x1;
			this.y1 = y1;
		}
	}

	@Override
	public String toString() {
		return "PdfAnnotation{" + "annotationType='" + annotationType + '\'' + ", content='" + content + '\''
				+ ", pageNumber=" + pageNumber + ", boundingBox=" + boundingBox + '}';
	}
}
