package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.PropertyNamingStrategies;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.Optional;

final class ResultParser {
	private static final ObjectMapper MAPPER = new ObjectMapper()
			.setPropertyNamingStrategy(PropertyNamingStrategies.SNAKE_CASE)
			.configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);

	private static final TypeReference<List<Table>> TABLE_LIST = new TypeReference<>() {
	};
	private static final TypeReference<List<String>> STRING_LIST = new TypeReference<>() {
	};
	private static final TypeReference<List<Chunk>> CHUNK_LIST = new TypeReference<>() {
	};
	private static final TypeReference<List<ExtractedImage>> IMAGE_LIST = new TypeReference<>() {
	};
	private static final TypeReference<List<Element>> ELEMENT_LIST = new TypeReference<>() {
	};
	private static final TypeReference<List<PageContent>> PAGE_CONTENT_LIST = new TypeReference<>() {
	};
	private static final TypeReference<Map<String, Object>> METADATA_MAP = new TypeReference<>() {
	};
	private static final TypeReference<EmbeddingPreset> EMBEDDING_PRESET = new TypeReference<>() {
	};
	private static final TypeReference<PageStructure> PAGE_STRUCTURE = new TypeReference<>() {
	};
	private static final TypeReference<DjotContent> DJOT_CONTENT = new TypeReference<>() {
	};

	private ResultParser() {
	}

	static ExtractionResult parse(String content, String mimeType, String tablesJson, String detectedLanguagesJson,
			String metadataJson, String chunksJson, String imagesJson, String pagesJson, String pageStructureJson,
			String elementsJson, String djotContentJson) throws KreuzbergException {
		return parse(content, mimeType, tablesJson, detectedLanguagesJson, metadataJson, chunksJson, imagesJson,
				pagesJson, pageStructureJson, elementsJson, djotContentJson, null, null, null);
	}

	static ExtractionResult parse(String content, String mimeType, String tablesJson, String detectedLanguagesJson,
			String metadataJson, String chunksJson, String imagesJson, String pagesJson, String pageStructureJson,
			String elementsJson, String djotContentJson, String language, String date, String subject)
			throws KreuzbergException {
		try {
			Map<String, Object> metadata = decode(metadataJson, METADATA_MAP, Collections.emptyMap());
			List<Table> tables = decode(tablesJson, TABLE_LIST, List.of());
			List<String> detectedLanguages = decode(detectedLanguagesJson, STRING_LIST, List.of());
			List<Chunk> chunks = decode(chunksJson, CHUNK_LIST, List.of());
			List<ExtractedImage> images = decode(imagesJson, IMAGE_LIST, List.of());
			List<PageContent> pages = decode(pagesJson, PAGE_CONTENT_LIST, List.of());
			PageStructure pageStructure = decode(pageStructureJson, PAGE_STRUCTURE, null);
			List<Element> elements = decode(elementsJson, ELEMENT_LIST, List.of());
			DjotContent djotContent = decode(djotContentJson, DJOT_CONTENT, null);

			// Build Metadata with FFI-provided language, date, and subject if available
			Metadata metadataObj = buildMetadata(metadata, language, date, subject);

			return new ExtractionResult(content != null ? content : "", mimeType != null ? mimeType : "", metadataObj,
					tables, detectedLanguages, chunks, images, pages, pageStructure, elements, djotContent);
		} catch (Exception e) {
			throw new KreuzbergException("Failed to parse extraction result", e);
		}
	}

	private static final java.util.Set<String> KNOWN_METADATA_KEYS = java.util.Set.of("title", "subject", "language",
			"created", "modified", "created_at", "modified_at", "created_by", "modified_by", "authors", "keywords",
			"pages", "image_preprocessing", "json_schema", "error");

	private static Metadata buildMetadata(Map<String, Object> metadataMap, String language, String date,
			String subject) {
		// Extract metadata fields, preferring FFI values over map values
		String title = getStringFromMap(metadataMap, "title");
		String actualSubject = subject != null ? subject : getStringFromMap(metadataMap, "subject");
		String actualLanguage = language != null ? language : getStringFromMap(metadataMap, "language");
		String createdAt = getStringFromMap(metadataMap, "created");
		String modifiedAt = date != null ? date : getStringFromMap(metadataMap, "modified");
		String createdBy = getStringFromMap(metadataMap, "created_by");
		String modifiedBy = getStringFromMap(metadataMap, "modified_by");

		@SuppressWarnings("unchecked")
		List<String> authors = (List<String>) metadataMap.get("authors");
		@SuppressWarnings("unchecked")
		List<String> keywords = (List<String>) metadataMap.get("keywords");
		// Convert the raw map to PageStructure using Jackson (cannot direct cast from
		// LinkedHashMap)
		PageStructure pages = convertValue(metadataMap.get("pages"), PageStructure.class);
		@SuppressWarnings("unchecked")
		Map<String, Object> imagePreprocessing = (Map<String, Object>) metadataMap.get("image_preprocessing");
		@SuppressWarnings("unchecked")
		Map<String, Object> jsonSchema = (Map<String, Object>) metadataMap.get("json_schema");
		@SuppressWarnings("unchecked")
		Map<String, Object> error = (Map<String, Object>) metadataMap.get("error");

		Metadata metadata = new Metadata(title != null ? Optional.of(title) : Optional.empty(),
				actualSubject != null ? Optional.of(actualSubject) : Optional.empty(),
				authors != null ? Optional.of(authors) : Optional.empty(),
				keywords != null ? Optional.of(keywords) : Optional.empty(),
				actualLanguage != null ? Optional.of(actualLanguage) : Optional.empty(),
				createdAt != null ? Optional.of(createdAt) : Optional.empty(),
				modifiedAt != null ? Optional.of(modifiedAt) : Optional.empty(),
				createdBy != null ? Optional.of(createdBy) : Optional.empty(),
				modifiedBy != null ? Optional.of(modifiedBy) : Optional.empty(),
				pages != null ? Optional.of(pages) : Optional.empty(),
				imagePreprocessing != null ? Optional.of(imagePreprocessing) : Optional.empty(),
				jsonSchema != null ? Optional.of(jsonSchema) : Optional.empty(),
				error != null ? Optional.of(error) : Optional.empty());

		// Add format-specific and other additional fields not handled above
		for (Map.Entry<String, Object> entry : metadataMap.entrySet()) {
			if (!KNOWN_METADATA_KEYS.contains(entry.getKey())) {
				metadata.setAdditionalProperty(entry.getKey(), entry.getValue());
			}
		}

		return metadata;
	}

	private static String getStringFromMap(Map<String, Object> map, String key) {
		Object value = map.get(key);
		return value instanceof String ? (String) value : null;
	}

	private static <T> T convertValue(Object value, Class<T> targetType) {
		if (value == null) {
			return null;
		}
		try {
			return MAPPER.convertValue(value, targetType);
		} catch (IllegalArgumentException e) {
			// Jackson conversion failed (e.g., validation error in constructor)
			// Return null instead of propagating the exception
			return null;
		}
	}

	private static <T> T decode(String json, TypeReference<T> type, T fallback) throws Exception {
		if (json == null || json.isBlank()) {
			return fallback;
		}
		return MAPPER.readValue(json, type);
	}

	static String toJson(Map<String, Object> map) throws Exception {
		return MAPPER.writeValueAsString(map);
	}

	static EmbeddingPreset parseEmbeddingPreset(String json) throws Exception {
		if (json == null || json.isBlank()) {
			return null;
		}
		return MAPPER.readValue(json, EMBEDDING_PRESET);
	}

	static ExtractionResult fromJson(String json) throws KreuzbergException {
		if (json == null || json.isBlank()) {
			throw new KreuzbergException("Result JSON cannot be null or empty");
		}
		try {
			WireExtractionResult wire = MAPPER.readValue(json, WireExtractionResult.class);
			return new ExtractionResult(wire.content != null ? wire.content : "",
					wire.mimeType != null ? wire.mimeType : "",
					wire.metadata != null ? wire.metadata : Metadata.empty(),
					wire.tables != null ? wire.tables : List.of(),
					wire.detectedLanguages != null ? wire.detectedLanguages : List.of(),
					wire.chunks != null ? wire.chunks : List.of(), wire.images != null ? wire.images : List.of(),
					wire.pages != null ? wire.pages : List.of(), wire.pageStructure,
					wire.elements != null ? wire.elements : List.of(), wire.djotContent);
		} catch (Exception e) {
			throw new KreuzbergException("Failed to parse result JSON", e);
		}
	}

	static String toJson(ExtractionResult result) throws Exception {
		WireExtractionResult wire = new WireExtractionResult(result.getContent(), result.getMimeType(),
				result.getMetadata(), result.getTables(), result.getDetectedLanguages(), result.getChunks(),
				result.getImages(), result.getPages(), result.getPageStructure().orElse(null), result.getElements(),
				result.getDjotContent().orElse(null));
		return MAPPER.writeValueAsString(wire);
	}

	static String toJsonValue(Object value) throws Exception {
		return MAPPER.writeValueAsString(value);
	}

	static List<String> parseStringList(String json) throws Exception {
		return MAPPER.readValue(json, STRING_LIST);
	}

	private static final class WireExtractionResult {
		private final String content;
		private final String mimeType;
		private final Metadata metadata;
		private final List<Table> tables;
		private final List<String> detectedLanguages;
		private final List<Chunk> chunks;
		private final List<ExtractedImage> images;
		private final List<PageContent> pages;
		private final PageStructure pageStructure;
		private final List<Element> elements;
		private final DjotContent djotContent;

		WireExtractionResult(@JsonProperty("content") String content, @JsonProperty("mime_type") String mimeType,
				@JsonProperty("metadata") Metadata metadata, @JsonProperty("tables") List<Table> tables,
				@JsonProperty("detected_languages") List<String> detectedLanguages,
				@JsonProperty("chunks") List<Chunk> chunks, @JsonProperty("images") List<ExtractedImage> images,
				@JsonProperty("pages") List<PageContent> pages,
				@JsonProperty("page_structure") PageStructure pageStructure,
				@JsonProperty("elements") List<Element> elements,
				@JsonProperty("djot_content") DjotContent djotContent) {
			this.content = content;
			this.mimeType = mimeType;
			this.metadata = metadata;
			this.tables = tables;
			this.detectedLanguages = detectedLanguages;
			this.chunks = chunks;
			this.images = images;
			this.pages = pages;
			this.pageStructure = pageStructure;
			this.elements = elements;
			this.djotContent = djotContent;
		}
	}
}
