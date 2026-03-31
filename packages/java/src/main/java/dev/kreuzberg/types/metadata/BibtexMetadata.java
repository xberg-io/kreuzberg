package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.List;
import java.util.Map;
import org.jspecify.annotations.Nullable;

/** BibTeX bibliography metadata. */
public record BibtexMetadata(@JsonProperty("entry_count") int entryCount,
		@JsonProperty("citation_keys") List<String> citationKeys, @JsonProperty("authors") List<String> authors,
		@JsonProperty("year_range") @Nullable YearRange yearRange,
		@JsonProperty("entry_types") @Nullable Map<String, Integer> entryTypes) {
}
