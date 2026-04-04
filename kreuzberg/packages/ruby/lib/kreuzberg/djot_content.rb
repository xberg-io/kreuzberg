# frozen_string_literal: true

begin
  require 'json'
rescue LoadError
  require 'json/pure'
end

module Kreuzberg
  class Result
    # Djot structured content representation
    #
    # Represents document content in Djot format with structured metadata about
    # blocks, images, links, footnotes, and other document elements.
    #
    class DjotContent
      attr_reader :plain_text, :blocks, :metadata_json, :tables, :images, :links, :footnotes, :attributes

      # Represents a formatted block in Djot content
      class FormattedBlock
        attr_reader :block_type, :children, :attributes, :content, :level

        # rubocop:disable Metrics/CyclomaticComplexity, Metrics/PerceivedComplexity
        def initialize(hash_or_type = nil, children: nil, attributes: nil, content: nil, level: nil, block_type: nil)
          if hash_or_type.is_a?(Hash)
            # Initialize from hash
            @block_type = hash_or_type[:block_type] || hash_or_type['block_type'] || ''
            @children = hash_or_type[:children] || hash_or_type['children']
            @attributes = hash_or_type[:attributes] || hash_or_type['attributes'] || {}
            @content = hash_or_type[:content] || hash_or_type['content']
            @level = hash_or_type[:level] || hash_or_type['level']
          else
            # Initialize from keyword arguments (for backward compatibility)
            @block_type = block_type || hash_or_type || ''
            @children = children || []
            @attributes = attributes || {}
            @content = content
            @level = level
          end
        end
        # rubocop:enable Metrics/CyclomaticComplexity, Metrics/PerceivedComplexity

        def to_h
          {
            block_type: @block_type,
            children: @children,
            attributes: @attributes,
            content: @content,
            level: @level
          }.compact
        end
      end

      # Represents an image in Djot content
      class DjotImage
        attr_reader :url, :alt, :title, :width, :height
        alias src url

        # rubocop:disable Metrics/CyclomaticComplexity
        def initialize(hash_or_url = nil, alt: nil, title: nil, width: nil, height: nil, url: nil, src: nil)
          if hash_or_url.is_a?(Hash)
            # Initialize from hash (supports both 'url' and 'src' keys)
            @url = hash_or_url[:url] || hash_or_url['url'] || hash_or_url[:src] || hash_or_url['src']
            @alt = hash_or_url[:alt] || hash_or_url['alt']
            @title = hash_or_url[:title] || hash_or_url['title']
            @width = hash_or_url[:width] || hash_or_url['width']
            @height = hash_or_url[:height] || hash_or_url['height']
          else
            # Initialize from keyword arguments
            @url = url || src || hash_or_url
            @alt = alt
            @title = title
            @width = width
            @height = height
          end
        end
        # rubocop:enable Metrics/CyclomaticComplexity

        def to_h
          {
            url: @url,
            alt: @alt,
            title: @title,
            width: @width,
            height: @height
          }.compact
        end
      end

      # Represents a link in Djot content
      class DjotLink
        attr_reader :url, :text, :title, :link_type
        alias href url

        # rubocop:disable Metrics/CyclomaticComplexity
        def initialize(hash_or_url = nil, text: nil, title: nil, url: nil, href: nil, link_type: nil)
          if hash_or_url.is_a?(Hash)
            # Initialize from hash (supports both 'url' and 'href' keys)
            @url = hash_or_url[:url] || hash_or_url['url'] || hash_or_url[:href] || hash_or_url['href']
            @text = hash_or_url[:text] || hash_or_url['text']
            @title = hash_or_url[:title] || hash_or_url['title']
            @link_type = hash_or_url[:link_type] || hash_or_url['link_type']
          else
            # Initialize from keyword arguments
            @url = url || href || hash_or_url
            @text = text
            @title = title
            @link_type = link_type
          end
        end
        # rubocop:enable Metrics/CyclomaticComplexity

        def to_h
          {
            url: @url,
            text: @text,
            title: @title,
            link_type: @link_type
          }.compact
        end
      end

      # Represents a footnote in Djot content
      class Footnote
        attr_reader :label, :content

        def initialize(label:, content:)
          @label = label
          @content = content
        end

        def to_h
          {
            label: @label,
            content: @content
          }
        end
      end

      # rubocop:disable Metrics/AbcSize, Metrics/CyclomaticComplexity, Metrics/PerceivedComplexity
      def initialize(hash)
        @plain_text = hash['plain_text'] || hash[:plain_text] || ''
        @blocks = parse_blocks(hash['blocks'] || hash[:blocks] || [])
        @metadata_json = hash['metadata_json'] || hash[:metadata_json] || '{}'
        @tables = hash['tables'] || hash[:tables] || []
        @images = parse_images(hash['images'] || hash[:images] || [])
        @links = parse_links(hash['links'] || hash[:links] || [])
        @footnotes = parse_footnotes(hash['footnotes'] || hash[:footnotes] || [])
        @attributes = hash['attributes'] || hash[:attributes] || {}
      end
      # rubocop:enable Metrics/AbcSize, Metrics/CyclomaticComplexity, Metrics/PerceivedComplexity

      def metadata
        @metadata ||= parse_metadata(@metadata_json)
      end

      def to_h
        {
          plain_text: @plain_text,
          blocks: @blocks.map(&:to_h),
          metadata_json: @metadata_json,
          tables: @tables,
          images: @images.map(&:to_h),
          links: @links.map(&:to_h),
          footnotes: @footnotes.map(&:to_h),
          attributes: @attributes
        }
      end

      private

      def parse_metadata(metadata_json)
        JSON.parse(metadata_json)
      rescue JSON::ParserError
        {}
      end

      def parse_blocks(blocks_data)
        blocks_data.map do |block|
          FormattedBlock.new(
            block_type: block['block_type'] || block[:block_type] || '',
            children: block['children'] || block[:children],
            attributes: block['attributes'] || block[:attributes]
          )
        end
      end

      # rubocop:disable Metrics/CyclomaticComplexity
      def parse_images(images_data)
        images_data.map do |image|
          DjotImage.new(
            url: image['url'] || image[:url] || image['src'] || image[:src],
            alt: image['alt'] || image[:alt],
            title: image['title'] || image[:title],
            width: image['width'] || image[:width],
            height: image['height'] || image[:height]
          )
        end
      end
      # rubocop:enable Metrics/CyclomaticComplexity

      # rubocop:disable Metrics/CyclomaticComplexity
      def parse_links(links_data)
        links_data.map do |link|
          DjotLink.new(
            url: link['url'] || link[:url] || link['href'] || link[:href],
            text: link['text'] || link[:text],
            title: link['title'] || link[:title],
            link_type: link['link_type'] || link[:link_type]
          )
        end
      end
      # rubocop:enable Metrics/CyclomaticComplexity

      def parse_footnotes(footnotes_data)
        footnotes_data.map do |note|
          Footnote.new(
            label: note['label'] || note[:label],
            content: note['content'] || note[:content]
          )
        end
      end
    end
  end
end
