package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.List;
import org.jspecify.annotations.Nullable;

/** Year range for bibliographic metadata. */
public record YearRange(@JsonProperty("min") @Nullable Integer min, @JsonProperty("max") @Nullable Integer max,
		@JsonProperty("years") List<Integer> years) {
}
