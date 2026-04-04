package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import java.util.Collections;
import java.util.List;

/**
 * Page hierarchy structure containing heading levels and block information.
 *
 * @param blockCount
 *            number of hierarchy blocks on this page
 * @param blocks
 *            hierarchical blocks with heading levels
 */
public record PageHierarchy(@JsonProperty("block_count") int blockCount,
		@JsonDeserialize(contentAs = HierarchicalBlock.class) @JsonProperty("blocks") List<HierarchicalBlock> blocks) {
	@JsonCreator
	public PageHierarchy(@JsonProperty("block_count") int blockCount,
			@JsonProperty("blocks") List<HierarchicalBlock> blocks) {
		this.blockCount = blockCount;
		this.blocks = blocks != null ? Collections.unmodifiableList(blocks) : List.of();
	}
}
