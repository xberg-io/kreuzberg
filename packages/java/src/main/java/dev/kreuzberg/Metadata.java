package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonAnySetter;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;

/**
 * Metadata extracted from a document.
 *
 * <p>
 * Contains common fields applicable to all formats (title, subject, authors,
 * keywords, language, timestamps, creators) extracted from document properties,
 * plus format-specific metadata and additional custom fields from
 * postprocessors.
 *
 * @since 0.8.0
 */
public final class Metadata {
	private final Optional<String> title;
	private final Optional<String> subject;
	private final Optional<List<String>> authors;
	private final Optional<List<String>> keywords;
	private final Optional<String> language;
	private final Optional<String> createdAt;
	private final Optional<String> modifiedAt;
	private final Optional<String> createdBy;
	private final Optional<String> modifiedBy;
	private final Optional<PageStructure> pages;
	private final Optional<Map<String, Object>> imagePreprocessing;
	private final Optional<Map<String, Object>> jsonSchema;
	private final Optional<Map<String, Object>> error;
	private final Optional<String> category;
	private final Optional<List<String>> tags;
	private final Optional<String> documentVersion;
	private final Optional<String> abstractText;
	private final Optional<String> outputFormat;
	private final Map<String, Object> additional;

	@JsonCreator
	public Metadata(@JsonProperty("title") Optional<String> title, @JsonProperty("subject") Optional<String> subject,
			@JsonProperty("authors") Optional<List<String>> authors,
			@JsonProperty("keywords") Optional<List<String>> keywords,
			@JsonProperty("language") Optional<String> language, @JsonProperty("created_at") Optional<String> createdAt,
			@JsonProperty("modified_at") Optional<String> modifiedAt,
			@JsonProperty("created_by") Optional<String> createdBy,
			@JsonProperty("modified_by") Optional<String> modifiedBy,
			@JsonProperty("pages") Optional<PageStructure> pages,
			@JsonProperty("image_preprocessing") Optional<Map<String, Object>> imagePreprocessing,
			@JsonProperty("json_schema") Optional<Map<String, Object>> jsonSchema,
			@JsonProperty("error") Optional<Map<String, Object>> error,
			@JsonProperty("category") Optional<String> category, @JsonProperty("tags") Optional<List<String>> tags,
			@JsonProperty("document_version") Optional<String> documentVersion,
			@JsonProperty("abstract_text") Optional<String> abstractText,
			@JsonProperty("output_format") Optional<String> outputFormat) {
		this.title = title != null ? title : Optional.empty();
		this.subject = subject != null ? subject : Optional.empty();
		this.authors = authors != null && authors.isPresent()
				? Optional.of(Collections.unmodifiableList(new ArrayList<>(authors.get())))
				: Optional.empty();
		this.keywords = keywords != null && keywords.isPresent()
				? Optional.of(Collections.unmodifiableList(new ArrayList<>(keywords.get())))
				: Optional.empty();
		this.language = language != null ? language : Optional.empty();
		this.createdAt = createdAt != null ? createdAt : Optional.empty();
		this.modifiedAt = modifiedAt != null ? modifiedAt : Optional.empty();
		this.createdBy = createdBy != null ? createdBy : Optional.empty();
		this.modifiedBy = modifiedBy != null ? modifiedBy : Optional.empty();
		this.pages = pages != null ? pages : Optional.empty();
		this.imagePreprocessing = imagePreprocessing != null ? imagePreprocessing : Optional.empty();
		this.jsonSchema = jsonSchema != null ? jsonSchema : Optional.empty();
		this.error = error != null ? error : Optional.empty();
		this.category = category != null ? category : Optional.empty();
		this.tags = tags != null && tags.isPresent()
				? Optional.of(Collections.unmodifiableList(new ArrayList<>(tags.get())))
				: Optional.empty();
		this.documentVersion = documentVersion != null ? documentVersion : Optional.empty();
		this.abstractText = abstractText != null ? abstractText : Optional.empty();
		this.outputFormat = outputFormat != null ? outputFormat : Optional.empty();
		this.additional = new HashMap<>();
	}

	/**
	 * Creates a new empty Metadata.
	 *
	 * @return a new empty Metadata instance
	 */
	public static Metadata empty() {
		return new Metadata(Optional.empty(), Optional.empty(), Optional.empty(), Optional.empty(), Optional.empty(),
				Optional.empty(), Optional.empty(), Optional.empty(), Optional.empty(), Optional.empty(),
				Optional.empty(), Optional.empty(), Optional.empty(), Optional.empty(), Optional.empty(),
				Optional.empty(), Optional.empty(), Optional.empty());
	}

	/**
	 * Used by Jackson to handle additional flattened fields from format metadata
	 * and custom postprocessor fields.
	 *
	 * @param name
	 *            the field name
	 * @param value
	 *            the field value
	 */
	@JsonAnySetter
	public void setAdditionalProperty(String name, Object value) {
		additional.put(name, value);
	}

	/**
	 * Get the document title.
	 *
	 * @return optional title
	 */
	public Optional<String> getTitle() {
		return title;
	}

	/**
	 * Get the document subject or description.
	 *
	 * @return optional subject
	 */
	public Optional<String> getSubject() {
		return subject;
	}

	/**
	 * Get the document authors.
	 *
	 * @return optional unmodifiable list of authors
	 */
	public Optional<List<String>> getAuthors() {
		return authors;
	}

	/**
	 * Get the document keywords/tags.
	 *
	 * @return optional unmodifiable list of keywords
	 */
	public Optional<List<String>> getKeywords() {
		return keywords;
	}

	/**
	 * Get the primary language code (ISO 639).
	 *
	 * @return optional language code (e.g., "en", "de")
	 */
	public Optional<String> getLanguage() {
		return language;
	}

	/**
	 * Get the creation timestamp (ISO 8601 format).
	 *
	 * @return optional creation timestamp
	 */
	public Optional<String> getCreatedAt() {
		return createdAt;
	}

	/**
	 * Get the last modification timestamp (ISO 8601 format).
	 *
	 * @return optional modification timestamp
	 */
	public Optional<String> getModifiedAt() {
		return modifiedAt;
	}

	/**
	 * Get the user who created the document.
	 *
	 * @return optional creator username
	 */
	public Optional<String> getCreatedBy() {
		return createdBy;
	}

	/**
	 * Get the user who last modified the document.
	 *
	 * @return optional modifier username
	 */
	public Optional<String> getModifiedBy() {
		return modifiedBy;
	}

	/**
	 * Get the page/slide/sheet structure with boundaries.
	 *
	 * @return optional page structure information
	 */
	public Optional<PageStructure> getPages() {
		return pages;
	}

	/**
	 * Get image preprocessing metadata (when OCR preprocessing was applied).
	 *
	 * @return optional image preprocessing metadata
	 */
	public Optional<Map<String, Object>> getImagePreprocessing() {
		return imagePreprocessing;
	}

	/**
	 * Get JSON schema (for structured data extraction).
	 *
	 * @return optional JSON schema
	 */
	public Optional<Map<String, Object>> getJsonSchema() {
		return jsonSchema;
	}

	/**
	 * Get error metadata (for batch operations).
	 *
	 * @return optional error metadata
	 */
	public Optional<Map<String, Object>> getError() {
		return error;
	}

	/**
	 * Get the document category.
	 *
	 * @return optional category string
	 * @since 4.5.0
	 */
	public Optional<String> getCategory() {
		return category;
	}

	/**
	 * Get the document tags.
	 *
	 * @return optional unmodifiable list of tags
	 * @since 4.5.0
	 */
	public Optional<List<String>> getTags() {
		return tags;
	}

	/**
	 * Get the document version.
	 *
	 * @return optional document version string
	 * @since 4.5.0
	 */
	public Optional<String> getDocumentVersion() {
		return documentVersion;
	}

	/**
	 * Get the document abstract text.
	 *
	 * @return optional abstract text
	 * @since 4.5.0
	 */
	public Optional<String> getAbstractText() {
		return abstractText;
	}

	/**
	 * Get the output format used during extraction.
	 *
	 * @return optional output format string
	 * @since 4.5.0
	 */
	public Optional<String> getOutputFormat() {
		return outputFormat;
	}

	/**
	 * Get additional custom fields from postprocessors and flattened format
	 * metadata.
	 *
	 * @return metadata map with additional fields (never null, may be empty)
	 */
	public Map<String, Object> getAdditional() {
		return Collections.unmodifiableMap(additional);
	}

	/**
	 * Check if any metadata is present.
	 *
	 * @return true if at least one field is present
	 */
	public boolean isEmpty() {
		return !title.isPresent() && !subject.isPresent() && !authors.isPresent() && !keywords.isPresent()
				&& !language.isPresent() && !createdAt.isPresent() && !modifiedAt.isPresent() && !createdBy.isPresent()
				&& !modifiedBy.isPresent() && !pages.isPresent() && !imagePreprocessing.isPresent()
				&& !jsonSchema.isPresent() && !error.isPresent() && !category.isPresent() && !tags.isPresent()
				&& !documentVersion.isPresent() && !abstractText.isPresent() && !outputFormat.isPresent()
				&& additional.isEmpty();
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof Metadata)) {
			return false;
		}
		Metadata other = (Metadata) obj;
		return Objects.equals(title, other.title) && Objects.equals(subject, other.subject)
				&& Objects.equals(authors, other.authors) && Objects.equals(keywords, other.keywords)
				&& Objects.equals(language, other.language) && Objects.equals(createdAt, other.createdAt)
				&& Objects.equals(modifiedAt, other.modifiedAt) && Objects.equals(createdBy, other.createdBy)
				&& Objects.equals(modifiedBy, other.modifiedBy) && Objects.equals(pages, other.pages)
				&& Objects.equals(imagePreprocessing, other.imagePreprocessing)
				&& Objects.equals(jsonSchema, other.jsonSchema) && Objects.equals(error, other.error)
				&& Objects.equals(category, other.category) && Objects.equals(tags, other.tags)
				&& Objects.equals(documentVersion, other.documentVersion)
				&& Objects.equals(abstractText, other.abstractText) && Objects.equals(outputFormat, other.outputFormat)
				&& Objects.equals(additional, other.additional);
	}

	@Override
	public int hashCode() {
		return Objects.hash(title, subject, authors, keywords, language, createdAt, modifiedAt, createdBy, modifiedBy,
				pages, imagePreprocessing, jsonSchema, error, category, tags, documentVersion, abstractText,
				outputFormat, additional);
	}

	@Override
	public String toString() {
		return "Metadata{" + "title=" + title + ", subject=" + subject + ", authors=" + authors + ", keywords="
				+ keywords + ", language=" + language + ", createdAt=" + createdAt + ", modifiedAt=" + modifiedAt
				+ ", createdBy=" + createdBy + ", modifiedBy=" + modifiedBy + ", pages=" + pages
				+ ", imagePreprocessing=" + imagePreprocessing + ", jsonSchema=" + jsonSchema + ", error=" + error
				+ ", category=" + category + ", tags=" + tags + ", documentVersion=" + documentVersion
				+ ", abstractText=" + abstractText + ", outputFormat=" + outputFormat + ", additional=" + additional
				+ '}';
	}
}
