package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.List;
import org.jspecify.annotations.Nullable;

/** CSV/TSV file metadata. */
public record CsvMetadata(@JsonProperty("row_count") int rowCount, @JsonProperty("column_count") int columnCount,
		@JsonProperty("delimiter") @Nullable String delimiter, @JsonProperty("has_header") boolean hasHeader,
		@JsonProperty("column_types") @Nullable List<String> columnTypes) {
}
