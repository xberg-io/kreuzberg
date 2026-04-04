# frozen_string_literal: true

module Kreuzberg
  class Result
    # Structured document representation.
    #
    # Provides a hierarchical, tree-based representation of document content
    # using a flat array of nodes with index-based parent/child references.
    #
    # @example
    #   if result.document
    #     result.document.nodes.each do |node|
    #       puts "#{node.id}: #{node.content[0..50]}"
    #     end
    #   end
    #
    class DocumentStructure
      attr_reader :nodes

      def initialize(hash)
        @nodes = parse_nodes(hash['nodes'] || hash[:nodes] || [])
      end

      # Convert to hash
      #
      # @return [Hash] Hash representation
      #
      def to_h
        { nodes: @nodes.map(&:to_h) }
      end

      private

      def parse_nodes(nodes_data)
        return [] if nodes_data.nil? || nodes_data.empty?

        nodes_data.map { |node_hash| DocumentNode.new(node_hash) }
      end
    end

    # Single node in the document structure tree.
    #
    # Represents a logical unit of content with deterministic ID, content,
    # tree structure information, and metadata.
    #
    class DocumentNode
      attr_reader :id, :content, :parent, :children, :content_layer, :page, :page_end, :bbox, :annotations

      def initialize(hash)
        assign_core_fields(hash)
        assign_tree_fields(hash)
        assign_metadata_fields(hash)
      end

      private

      def assign_core_fields(hash)
        @id = hash['id'] || hash[:id] || ''
        @content = hash['content'] || hash[:content] || {}
        @content_layer = hash['content_layer'] || hash[:content_layer] || 'body'
      end

      def assign_tree_fields(hash)
        @parent = hash['parent'] || hash[:parent]
        @children = parse_children(hash['children'] || hash[:children] || [])
      end

      def assign_metadata_fields(hash)
        @page = hash['page'] || hash[:page]
        @page_end = hash['page_end'] || hash[:page_end]
        @bbox = parse_bbox(hash['bbox'] || hash[:bbox])
        @annotations = parse_annotations(hash['annotations'] || hash[:annotations] || [])
      end

      # Convert to hash
      #
      # @return [Hash] Hash representation
      #
      def to_h
        {
          id: @id,
          content: @content,
          parent: @parent,
          children: @children,
          content_layer: @content_layer,
          page: @page,
          page_end: @page_end,
          bbox: @bbox&.to_h,
          annotations: @annotations.map(&:to_h)
        }.compact
      end

      def parse_children(children_data)
        return [] if children_data.nil? || children_data.empty?

        if children_data.is_a?(Array)
          children_data.map { |c| extract_child_index(c) }
        else
          []
        end
      end

      def extract_child_index(child)
        if child.is_a?(Integer)
          child
        else
          child['index'] || child[:index]
        end
      end

      def parse_bbox(bbox_data)
        return nil if bbox_data.nil?

        DocumentBoundingBox.new(bbox_data)
      end

      def parse_annotations(annotations_data)
        return [] if annotations_data.nil? || annotations_data.empty?

        annotations_data.map { |ann| DocumentAnnotation.new(ann) }
      end
    end

    # Bounding box for document node positioning.
    #
    # Represents rectangular coordinates for a node within the document.
    #
    class DocumentBoundingBox
      attr_reader :x0, :y0, :x1, :y1

      def initialize(hash)
        @x0 = extract_float(hash, 'x0')
        @y0 = extract_float(hash, 'y0')
        @x1 = extract_float(hash, 'x1')
        @y1 = extract_float(hash, 'y1')
      end

      # Convert to hash
      #
      # @return [Hash] Hash representation
      #
      def to_h
        {
          x0: @x0,
          y0: @y0,
          x1: @x1,
          y1: @y1
        }.compact
      end

      private

      def extract_float(hash, key)
        (hash[key] || hash[key.to_sym])&.to_f
      end
    end

    # Annotation for a document node.
    #
    # Represents inline text annotations (formatting, links) with byte-range
    # references into the node's text content.
    #
    class DocumentAnnotation
      attr_reader :start, :end_offset, :annotation_type, :url, :title

      def initialize(hash)
        @start = (hash['start'] || hash[:start] || 0).to_i
        @end_offset = (hash['end'] || hash[:end] || 0).to_i
        parse_kind(hash['kind'] || hash[:kind] || {})
      end

      # Convert to hash
      #
      # @return [Hash] Hash representation
      #
      def to_h
        kind_hash = { annotation_type: @annotation_type }
        url = @url
        kind_hash[:url] = url if url
        title = @title
        kind_hash[:title] = title if title

        {
          start: @start,
          end: @end_offset,
          kind: kind_hash
        }
      end

      private

      def parse_kind(kind_hash)
        return if kind_hash.nil? || kind_hash.empty?

        @annotation_type =
          kind_hash['annotation_type'] ||
          kind_hash[:annotation_type] ||
          'bold'
        @url = kind_hash['url'] || kind_hash[:url]
        @title = kind_hash['title'] || kind_hash[:title]
      end
    end
  end
end
