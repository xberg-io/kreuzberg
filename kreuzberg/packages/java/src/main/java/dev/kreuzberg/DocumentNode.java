package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collections;
import java.util.List;
import java.util.Objects;
import java.util.Optional;

/**
 * A single node in the document structure tree.
 *
 * <p>
 * Each node has a deterministic ID, typed content, optional parent/children
 * references for tree structure, and metadata like page number and bounding
 * box.
 *
 * @since 4.3.0
 */
@JsonIgnoreProperties(ignoreUnknown = true)
public final class DocumentNode {
	private final String id;
	private final NodeContent content;
	private final Integer parent;
	private final List<Integer> children;
	private final String contentLayer;
	private final Integer pageNumber;
	private final Integer pageEnd;
	private final BoundingBox bbox;
	private final List<TextAnnotation> annotations;

	/**
	 * Create a new DocumentNode.
	 *
	 * @param id
	 *            deterministic node identifier (must not be null)
	 * @param content
	 *            the typed node content (must not be null)
	 * @param parent
	 *            parent node index, or null for root-level nodes
	 * @param children
	 *            list of child node indices (may be empty or null)
	 * @param contentLayer
	 *            content layer classification (body, header, footer, footnote), or
	 *            null for default (body)
	 * @param pageNumber
	 *            page number where this node appears (1-indexed), or null
	 * @param pageEnd
	 *            page number where this node ends (for multi-page content), or null
	 * @param bbox
	 *            bounding box in document coordinates, or null
	 * @param annotations
	 *            inline text annotations, or null
	 */
	@JsonCreator
	public DocumentNode(@JsonProperty("id") String id, @JsonProperty("content") NodeContent content,
			@JsonProperty("parent") Integer parent, @JsonProperty("children") List<Integer> children,
			@JsonProperty("content_layer") String contentLayer, @JsonProperty("page") Integer pageNumber,
			@JsonProperty("page_end") Integer pageEnd, @JsonProperty("bbox") BoundingBox bbox,
			@JsonProperty("annotations") List<TextAnnotation> annotations) {
		this.id = Objects.requireNonNull(id, "id must not be null");
		this.content = Objects.requireNonNull(content, "content must not be null");
		this.parent = parent;
		this.children = Collections.unmodifiableList(children != null ? children : Collections.emptyList());
		this.contentLayer = contentLayer != null ? contentLayer : "body";
		this.pageNumber = pageNumber;
		this.pageEnd = pageEnd;
		this.bbox = bbox;
		this.annotations = Collections.unmodifiableList(annotations != null ? annotations : Collections.emptyList());
	}

	/**
	 * Get the deterministic node identifier.
	 *
	 * @return node ID string
	 */
	@JsonProperty("id")
	public String getId() {
		return id;
	}

	/**
	 * Get the typed node content.
	 *
	 * @return node content (never null)
	 */
	@JsonProperty("content")
	public NodeContent getContent() {
		return content;
	}

	/**
	 * Get the parent node index.
	 *
	 * @return parent index, or empty if this is a root-level node
	 */
	@JsonProperty("parent")
	public Optional<Integer> getParent() {
		return Optional.ofNullable(parent);
	}

	/**
	 * Get the child node indices.
	 *
	 * <p>
	 * Indices are in reading order.
	 *
	 * @return unmodifiable list of child indices (never null, but may be empty)
	 */
	@JsonProperty("children")
	public List<Integer> getChildren() {
		return children;
	}

	/**
	 * Get the content layer classification.
	 *
	 * <p>
	 * Possible values: "body" (default), "header", "footer", "footnote".
	 *
	 * @return content layer classification
	 */
	@JsonProperty("content_layer")
	public String getContentLayer() {
		return contentLayer;
	}

	/**
	 * Get the page number where this node appears.
	 *
	 * <p>
	 * Page numbers are 1-indexed.
	 *
	 * @return page number, or empty if not available
	 */
	@JsonProperty("page")
	public Optional<Integer> getPageNumber() {
		return Optional.ofNullable(pageNumber);
	}

	/**
	 * Get the ending page number for multi-page nodes.
	 *
	 * <p>
	 * For content that spans multiple pages (e.g., tables), this indicates the last
	 * page.
	 *
	 * @return ending page number, or empty if not available or single-page
	 */
	@JsonProperty("page_end")
	public Optional<Integer> getPageEnd() {
		return Optional.ofNullable(pageEnd);
	}

	/**
	 * Get the bounding box for this node.
	 *
	 * @return bounding box in document coordinates, or empty if not available
	 */
	@JsonProperty("bbox")
	public Optional<BoundingBox> getBBox() {
		return Optional.ofNullable(bbox);
	}

	/**
	 * Get inline text annotations on this node's content.
	 *
	 * <p>
	 * Annotations reference byte offsets for formatting, links, and other inline
	 * markup. Only meaningful for text-carrying nodes; empty for containers.
	 *
	 * @return unmodifiable list of annotations (never null, but may be empty)
	 */
	@JsonProperty("annotations")
	public List<TextAnnotation> getAnnotations() {
		return annotations;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof DocumentNode)) {
			return false;
		}
		DocumentNode other = (DocumentNode) obj;
		return Objects.equals(id, other.id) && Objects.equals(content, other.content)
				&& Objects.equals(parent, other.parent) && Objects.equals(children, other.children)
				&& Objects.equals(contentLayer, other.contentLayer) && Objects.equals(pageNumber, other.pageNumber)
				&& Objects.equals(pageEnd, other.pageEnd) && Objects.equals(bbox, other.bbox)
				&& Objects.equals(annotations, other.annotations);
	}

	@Override
	public int hashCode() {
		return Objects.hash(id, content, parent, children, contentLayer, pageNumber, pageEnd, bbox, annotations);
	}

	@Override
	public String toString() {
		return "DocumentNode{" + "id='" + id + '\'' + ", contentType=" + content.getNodeType() + ", parent=" + parent
				+ ", children=" + children.size() + ", layer=" + contentLayer + ", page=" + pageNumber + '}';
	}
}
