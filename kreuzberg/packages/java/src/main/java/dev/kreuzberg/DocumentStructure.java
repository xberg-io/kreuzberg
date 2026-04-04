package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Hierarchical, tree-based representation of document content.
 *
 * <p>
 * Contains a flat array of document nodes with index-based parent/child
 * references forming a tree structure. Root-level nodes have no parent.
 *
 * <p>
 * Available when extraction is configured with
 * {@code include_document_structure=true}.
 *
 * @since 4.3.0
 */
@JsonIgnoreProperties(ignoreUnknown = true)
public final class DocumentStructure {
	private final List<DocumentNode> nodes;

	/**
	 * Create a new DocumentStructure.
	 *
	 * @param nodes
	 *            the flat list of document nodes (must not be null)
	 */
	@JsonCreator
	public DocumentStructure(@JsonProperty("nodes") List<DocumentNode> nodes) {
		this.nodes = Collections.unmodifiableList(nodes != null ? nodes : Collections.emptyList());
	}

	/**
	 * Get all nodes in the document structure.
	 *
	 * <p>
	 * Nodes are stored in reading/document order and form a tree through
	 * parent/child index references.
	 *
	 * @return unmodifiable list of all nodes (never null, but may be empty)
	 */
	@JsonProperty("nodes")
	public List<DocumentNode> getNodes() {
		return nodes;
	}

	/**
	 * Get the total number of nodes in the document structure.
	 *
	 * @return the node count
	 */
	public int getNodeCount() {
		return nodes.size();
	}

	/**
	 * Check if the document structure is empty.
	 *
	 * @return true if there are no nodes, false otherwise
	 */
	public boolean isEmpty() {
		return nodes.isEmpty();
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof DocumentStructure)) {
			return false;
		}
		DocumentStructure other = (DocumentStructure) obj;
		return Objects.equals(nodes, other.nodes);
	}

	@Override
	public int hashCode() {
		return Objects.hash(nodes);
	}

	@Override
	public String toString() {
		return "DocumentStructure{" + "nodeCount=" + nodes.size() + '}';
	}
}
