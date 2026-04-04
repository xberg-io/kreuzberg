<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Structured document representation with hierarchical node tree.
 *
 * A flat array of nodes with index-based parent/child references forming a tree.
 * Root-level nodes have no parent. Nodes are stored in document/reading order.
 *
 * @property-read array<DocumentNode> $nodes All nodes in document reading order
 */
readonly class DocumentStructure
{
    /**
     * @param array<DocumentNode> $nodes
     */
    public function __construct(
        public array $nodes = [],
    ) {
    }

    /**
     * Create DocumentStructure from array returned by extension.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var array<array<string, mixed>> $nodesData */
        $nodesData = $data['nodes'] ?? [];

        $nodes = array_map(
            /** @param array<string, mixed> $node */
            static fn (array $node): DocumentNode => DocumentNode::fromArray($node),
            $nodesData,
        );

        return new self(
            nodes: $nodes,
        );
    }

    /**
     * Get total number of nodes in structure.
     */
    public function count(): int
    {
        return count($this->nodes);
    }

    /**
     * Get a node by index.
     */
    public function getNode(int $index): ?DocumentNode
    {
        return $this->nodes[$index] ?? null;
    }

    /**
     * Check if document structure is empty.
     */
    public function isEmpty(): bool
    {
        return count($this->nodes) === 0;
    }
}

/**
 * A single node in the document tree.
 *
 * Each node has a deterministic ID, typed content, optional parent/children references,
 * and metadata like page number and content layer classification.
 *
 * @property-read string $id Deterministic identifier (hash of content + position)
 * @property-read string $nodeType Node type discriminant (from content)
 * @property-read mixed $content Node content (map with type-specific fields)
 * @property-read string|null $contentLayer Content layer classification (body, header, footer, footnote)
 * @property-read int|null $parent Parent node index
 * @property-read array<int> $children Child node indices in reading order
 * @property-read int|null $pageNumber Page number where node starts (1-indexed)
 * @property-read int|null $pageNumberEnd Page number where node ends (for multi-page elements)
 * @property-read BoundingBox|null $bbox Bounding box coordinates
 * @property-read array<DocumentTextAnnotation> $annotations Inline text annotations
 */
readonly class DocumentNode
{
    /**
     * @param array<int> $children
     * @param array<DocumentTextAnnotation> $annotations
     */
    public function __construct(
        public string $id,
        public string $nodeType,
        public mixed $content,
        public ?string $contentLayer = null,
        public ?int $parent = null,
        public array $children = [],
        public ?int $pageNumber = null,
        public ?int $pageNumberEnd = null,
        public ?BoundingBox $bbox = null,
        public array $annotations = [],
    ) {
    }

    /**
     * Create DocumentNode from array returned by extension.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $id */
        $id = $data['id'] ?? '';

        /** @var mixed $content */
        $content = $data['content'] ?? [];

        $contentArray = is_array($content) ? $content : [];
        /** @var string $nodeType */
        $nodeType = is_string($contentArray['node_type'] ?? '') ? (string) ($contentArray['node_type'] ?? '') : '';

        /** @var string|null $contentLayer */
        $contentLayer = $data['content_layer'] ?? null;

        /** @var int|null $parent */
        $parent = isset($data['parent']) && is_numeric($data['parent'])
            ? (int) $data['parent']
            : null;

        /** @var array<int> $children */
        $children = [];
        if (isset($data['children']) && is_array($data['children'])) {
            foreach ($data['children'] as $child) {
                if (is_numeric($child)) {
                    $children[] = (int) $child;
                }
            }
        }

        /** @var int|null $pageNumber */
        $pageNumber = isset($data['page']) && is_numeric($data['page'])
            ? (int) $data['page']
            : null;

        /** @var int|null $pageNumberEnd */
        $pageNumberEnd = isset($data['page_end']) && is_numeric($data['page_end'])
            ? (int) $data['page_end']
            : null;

        $bbox = null;
        if (isset($data['bbox'])) {
            /** @var array<string, mixed> $bboxData */
            $bboxData = $data['bbox'];
            $bbox = BoundingBox::fromArray($bboxData);
        }

        $annotations = [];
        if (isset($data['annotations']) && is_array($data['annotations'])) {
            foreach ($data['annotations'] as $annotation) {
                if (is_array($annotation)) {
                    /** @var array<string, mixed> $annotation */
                    $annotations[] = DocumentTextAnnotation::fromArray($annotation);
                }
            }
        }

        return new self(
            id: $id,
            nodeType: $nodeType,
            content: $content,
            contentLayer: $contentLayer,
            parent: $parent,
            children: $children,
            pageNumber: $pageNumber,
            pageNumberEnd: $pageNumberEnd,
            bbox: $bbox,
            annotations: $annotations,
        );
    }

    /**
     * Check if this is a root node (no parent).
     */
    public function isRoot(): bool
    {
        return $this->parent === null;
    }

    /**
     * Check if this node has children.
     */
    public function hasChildren(): bool
    {
        return count($this->children) > 0;
    }

    /**
     * Get node type with snake_case to camelCase conversion for readability.
     */
    public function getNodeTypeReadable(): string
    {
        $map = [
            'title' => 'Title',
            'heading' => 'Heading',
            'paragraph' => 'Paragraph',
            'list' => 'List',
            'list_item' => 'List Item',
            'table' => 'Table',
            'image' => 'Image',
            'code' => 'Code',
            'quote' => 'Quote',
            'formula' => 'Formula',
            'footnote' => 'Footnote',
            'group' => 'Group',
            'page_break' => 'Page Break',
        ];

        return $map[$this->nodeType] ?? ucfirst(str_replace('_', ' ', $this->nodeType));
    }
}

/**
 * Inline text annotation with byte-range formatting and links.
 *
 * Annotations reference byte offsets into a node's text content,
 * enabling precise identification of formatted regions.
 *
 * @property-read int $start Start byte offset (inclusive)
 * @property-read int $end End byte offset (exclusive)
 * @property-read string $kind Annotation type (bold, italic, link, etc.)
 * @property-read string|null $url URL for link annotations
 */
readonly class DocumentTextAnnotation
{
    public function __construct(
        public int $start,
        public int $end,
        public string $kind,
        public ?string $url = null,
    ) {
    }

    /**
     * Create DocumentTextAnnotation from array returned by extension.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var int $start */
        $start = isset($data['start']) && is_numeric($data['start'])
            ? (int) $data['start']
            : 0;

        /** @var int $end */
        $end = isset($data['end']) && is_numeric($data['end'])
            ? (int) $data['end']
            : 0;

        /** @var string $kind */
        $kind = $data['kind'] ?? $data['annotation_type'] ?? '';

        /** @var string|null $url */
        $url = isset($data['url']) && is_string($data['url'])
            ? $data['url']
            : null;

        return new self(
            start: $start,
            end: $end,
            kind: $kind,
            url: $url,
        );
    }

    /**
     * Get human-readable annotation kind name.
     */
    public function getKindReadable(): string
    {
        $map = [
            'bold' => 'Bold',
            'italic' => 'Italic',
            'underline' => 'Underline',
            'strikethrough' => 'Strikethrough',
            'code' => 'Code',
            'subscript' => 'Subscript',
            'superscript' => 'Superscript',
            'link' => 'Link',
        ];

        return $map[$this->kind] ?? ucfirst($this->kind);
    }
}

/**
 * Structured table grid with cell-level metadata.
 *
 * Stores row/column dimensions and cell information in row-major order.
 *
 * @property-read int $rows Number of rows
 * @property-read int $columns Number of columns
 * @property-read array<DocumentTableCell> $cells All cells in row-major order
 */
readonly class DocumentTableGrid
{
    /**
     * @param array<DocumentTableCell> $cells
     */
    public function __construct(
        public int $rows,
        public int $columns,
        public array $cells = [],
    ) {
    }

    /**
     * Create DocumentTableGrid from array returned by extension.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var int $rows */
        $rows = isset($data['rows']) && is_numeric($data['rows'])
            ? (int) $data['rows']
            : 0;

        /** @var int $columns */
        $columns = isset($data['cols']) && is_numeric($data['cols'])
            ? (int) $data['cols']
            : 0;

        $cells = [];
        if (isset($data['cells']) && is_array($data['cells'])) {
            foreach ($data['cells'] as $cell) {
                if (is_array($cell)) {
                    /** @var array<string, mixed> $cell */
                    $cells[] = DocumentTableCell::fromArray($cell);
                }
            }
        }

        return new self(
            rows: $rows,
            columns: $columns,
            cells: $cells,
        );
    }

    /**
     * Get total number of cells.
     */
    public function getCellCount(): int
    {
        return count($this->cells);
    }

    /**
     * Get cell at specific row and column (0-indexed).
     */
    public function getCellAt(int $row, int $col): ?DocumentTableCell
    {
        foreach ($this->cells as $cell) {
            if ($cell->row === $row && $cell->col === $col) {
                return $cell;
            }
        }

        return null;
    }
}

/**
 * Individual grid cell with position and span metadata.
 *
 * @property-read string $content Cell text content
 * @property-read int $row Zero-indexed row position
 * @property-read int $col Zero-indexed column position
 * @property-read int $rowSpan Number of rows this cell spans
 * @property-read int $colSpan Number of columns this cell spans
 * @property-read bool $isHeader Whether this is a header cell
 * @property-read BoundingBox|null $bbox Cell bounding box if available
 */
readonly class DocumentTableCell
{
    public function __construct(
        public string $content,
        public int $row,
        public int $col,
        public int $rowSpan = 1,
        public int $colSpan = 1,
        public bool $isHeader = false,
        public ?BoundingBox $bbox = null,
    ) {
    }

    /**
     * Create DocumentTableCell from array returned by extension.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $content */
        $content = $data['content'] ?? '';

        /** @var int $row */
        $row = isset($data['row']) && is_numeric($data['row'])
            ? (int) $data['row']
            : 0;

        /** @var int $col */
        $col = isset($data['col']) && is_numeric($data['col'])
            ? (int) $data['col']
            : 0;

        /** @var int $rowSpan */
        $rowSpan = isset($data['row_span']) && is_numeric($data['row_span'])
            ? (int) $data['row_span']
            : 1;

        /** @var int $colSpan */
        $colSpan = isset($data['col_span']) && is_numeric($data['col_span'])
            ? (int) $data['col_span']
            : 1;

        /** @var bool $isHeader */
        $isHeader = isset($data['is_header']) && $data['is_header'] === true;

        $bbox = null;
        if (isset($data['bbox'])) {
            /** @var array<string, mixed> $bboxData */
            $bboxData = $data['bbox'];
            $bbox = BoundingBox::fromArray($bboxData);
        }

        return new self(
            content: $content,
            row: $row,
            col: $col,
            rowSpan: $rowSpan,
            colSpan: $colSpan,
            isHeader: $isHeader,
            bbox: $bbox,
        );
    }
}
