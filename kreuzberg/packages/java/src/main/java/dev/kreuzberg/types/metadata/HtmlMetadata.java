package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.List;
import java.util.Map;
import org.jspecify.annotations.Nullable;

/** HTML metadata extracted from HTML documents. */
public record HtmlMetadata(@JsonProperty("title") @Nullable String title,
		@JsonProperty("description") @Nullable String description, @JsonProperty("keywords") List<String> keywords,
		@JsonProperty("author") @Nullable String author, @JsonProperty("canonical_url") @Nullable String canonicalUrl,
		@JsonProperty("base_href") @Nullable String baseHref, @JsonProperty("language") @Nullable String language,
		@JsonProperty("text_direction") @Nullable TextDirection textDirection,
		@JsonProperty("open_graph") Map<String, String> openGraph,
		@JsonProperty("twitter_card") Map<String, String> twitterCard,
		@JsonProperty("meta_tags") Map<String, String> metaTags, @JsonProperty("headers") List<HeaderMetadata> headers,
		@JsonProperty("links") List<LinkMetadata> links, @JsonProperty("images") List<ImageMetadata> images,
		@JsonProperty("structured_data") List<StructuredData> structuredData) {
}
