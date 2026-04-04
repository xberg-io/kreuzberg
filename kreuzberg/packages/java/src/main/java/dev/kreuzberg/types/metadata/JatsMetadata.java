package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.List;
import java.util.Map;
import org.jspecify.annotations.Nullable;

/** JATS (Journal Article Tag Suite) metadata. */
public record JatsMetadata(@JsonProperty("copyright") @Nullable String copyright,
		@JsonProperty("license") @Nullable String license,
		@JsonProperty("history_dates") Map<String, String> historyDates,
		@JsonProperty("contributor_roles") List<ContributorRole> contributorRoles) {
}
