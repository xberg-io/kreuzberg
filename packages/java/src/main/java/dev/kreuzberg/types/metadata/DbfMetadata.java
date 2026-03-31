package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.List;

/** dBASE (DBF) file metadata. */
public record DbfMetadata(@JsonProperty("record_count") int recordCount,
		@JsonProperty("field_count") int fieldCount, @JsonProperty("fields") List<DbfFieldInfo> fields) {
}
