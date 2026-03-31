package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.List;
import org.jspecify.annotations.Nullable;

/** Citation file metadata (RIS, PubMed, EndNote). */
public record CitationMetadata(@JsonProperty("citation_count") int citationCount,
		@JsonProperty("format") @Nullable String format, @JsonProperty("authors") List<String> authors,
		@JsonProperty("year_range") @Nullable YearRange yearRange, @JsonProperty("dois") List<String> dois,
		@JsonProperty("keywords") List<String> keywords) {
}
