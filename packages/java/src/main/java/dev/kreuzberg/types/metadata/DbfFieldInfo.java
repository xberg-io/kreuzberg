package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonProperty;

/** dBASE field information. */
public record DbfFieldInfo(@JsonProperty("name") String name, @JsonProperty("field_type") String fieldType) {
}
