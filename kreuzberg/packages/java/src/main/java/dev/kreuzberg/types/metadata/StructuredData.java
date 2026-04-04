package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonProperty;
import org.jspecify.annotations.Nullable;

/** Structured data (JSON-LD, microdata, RDFa) metadata. */
public record StructuredData(@JsonProperty("data_type") StructuredDataType dataType,
		@JsonProperty("raw_json") String rawJson, @JsonProperty("schema_type") @Nullable String schemaType) {
}
