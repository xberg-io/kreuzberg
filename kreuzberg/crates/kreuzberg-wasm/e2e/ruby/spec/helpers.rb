# frozen_string_literal: true

# rubocop:disable Metrics/AbcSize, Metrics/CyclomaticComplexity, Metrics/MethodLength, Metrics/PerceivedComplexity, Metrics/ParameterLists, Style/Documentation, Style/IfUnlessModifier, Layout/LineLength, Layout/EmptyLineAfterGuardClause

require 'json'
require 'pathname'
require 'rspec/expectations'
require 'kreuzberg'
require 'rspec/core'

module E2ERuby
  module_function

  WORKSPACE_ROOT = Pathname.new(__dir__).join('..', '..', '..').expand_path
  TEST_DOCUMENTS = WORKSPACE_ROOT.join('test_documents')

  def resolve_document(relative)
    TEST_DOCUMENTS.join(relative)
  end

  def build_config(raw)
    return nil unless raw.is_a?(Hash) && !raw.empty?

    symbolize_keys(raw)
  end

  def symbolize_keys(value)
    case value
    when Hash
      value.each_with_object({}) do |(key, val), acc|
        symbol_key = key.respond_to?(:to_sym) ? key.to_sym : key
        acc[symbol_key] = symbolize_keys(val)
      end
    when Array
      value.map { |item| symbolize_keys(item) }
    else
      value
    end
  end

  def skip_reason_for(error, fixture_id, requirements, notes = nil)
    message = error.message.to_s
    downcased = message.downcase
    requirement_hit = requirements.any? { |req| downcased.include?(req.downcase) }
    missing_dependency = error.is_a?(Kreuzberg::Errors::MissingDependencyError) || downcased.include?('missing dependency')
    unsupported_format = downcased.include?('unsupported format')

    return nil unless missing_dependency || unsupported_format || requirement_hit

    reason =
      if missing_dependency
        dependency = error.respond_to?(:dependency) ? error.dependency : nil
        if dependency && !dependency.to_s.empty?
          "missing dependency #{dependency}"
        else
          'missing dependency'
        end
      elsif unsupported_format
        'unsupported format'
      elsif requirements.any?
        "requires #{requirements.join(', ')}"
      else
        'environmental requirement'
      end

    details = "Skipping #{fixture_id}: #{reason}. #{error.class}: #{message}"
    details += " Notes: #{notes}" if notes
    warn(details)
    details
  end

  def run_fixture(fixture_id, relative_path, config_hash, requirements:, notes:, skip_if_missing: true, &block)
    run_fixture_with_method(fixture_id, relative_path, config_hash, :sync, :file,
                            requirements: requirements, notes: notes, skip_if_missing: skip_if_missing, &block)
  end

  def run_fixture_with_method(fixture_id, relative_path, config_hash, method, input_type, requirements:, notes:, skip_if_missing: true)
    document_path = resolve_document(relative_path)

    if skip_if_missing && !document_path.exist?
      warn "Skipping #{fixture_id}: missing document at #{document_path}"
      raise RSpec::Core::Pending::SkipDeclaredInExample, 'missing document'
    end

    config = build_config(config_hash)
    result = nil
    begin
      result = perform_extraction(document_path, config, method, input_type)
    rescue StandardError => e
      if (reason = skip_reason_for(e, fixture_id, requirements, notes))
        raise RSpec::Core::Pending::SkipDeclaredInExample, reason
      end
      raise
    end

    yield result
  end

  def perform_extraction(document_path, config, method, input_type)
    mime_type = detect_mime_type(document_path)
    case [method, input_type]
    when [:sync, :file]
      Kreuzberg.extract_file_sync(path: document_path.to_s, config: config)
    when [:sync, :bytes]
      bytes = File.binread(document_path.to_s)
      Kreuzberg.extract_bytes_sync(data: bytes, mime_type: mime_type, config: config)
    when [:async, :file]
      Kreuzberg.extract_file(path: document_path.to_s, config: config)
    when [:async, :bytes]
      bytes = File.binread(document_path.to_s)
      Kreuzberg.extract_bytes(data: bytes, mime_type: mime_type, config: config)
    when [:batch_sync, :file]
      results = Kreuzberg.batch_extract_files_sync(paths: [document_path.to_s], config: config)
      results.first
    when [:batch_sync, :bytes]
      bytes = File.binread(document_path.to_s)
      results = Kreuzberg.batch_extract_bytes_sync(data_array: [bytes], mime_types: [mime_type], config: config)
      results.first
    when [:batch_async, :file]
      results = Kreuzberg.batch_extract_files(paths: [document_path.to_s], config: config)
      results.first
    when [:batch_async, :bytes]
      bytes = File.binread(document_path.to_s)
      results = Kreuzberg.batch_extract_bytes(data_array: [bytes], mime_types: [mime_type], config: config)
      results.first
    else
      raise ArgumentError, "Unknown extraction method/input_type combo: #{method}/#{input_type}"
    end
  end

  def detect_mime_type(document_path)
    Kreuzberg.detect_mime_type_from_path(document_path.to_s)
  end

  module Assertions
    extend RSpec::Matchers

    def self.assert_expected_mime(result, expected)
      return if expected.empty?

      expect(expected.any? { |token| result.mime_type.include?(token) }).to be(true)
    end

    def self.assert_min_content_length(result, minimum)
      expect(result.content.length).to be >= minimum
    end

    def self.assert_max_content_length(result, maximum)
      expect(result.content.length).to be <= maximum
    end

    def self.assert_content_contains_any(result, snippets)
      return if snippets.empty?

      lowered = result.content.downcase
      expect(snippets.any? { |snippet| lowered.include?(snippet.downcase) }).to be(true)
    end

    def self.assert_content_contains_all(result, snippets)
      return if snippets.empty?

      lowered = result.content.downcase
      expect(snippets.all? { |snippet| lowered.include?(snippet.downcase) }).to be(true)
    end

    def self.assert_table_count(result, minimum, maximum)
      tables = Array(result.tables)
      expect(tables.length).to be >= minimum if minimum
      expect(tables.length).to be <= maximum if maximum
    end

    def self.assert_detected_languages(result, expected, min_confidence)
      return if expected.empty?

      languages = result.detected_languages
      expect(languages).not_to be_nil
      expect(expected.all? { |lang| languages.include?(lang) }).to be(true)

      return unless min_confidence

      metadata = result.metadata || {}
      confidence = metadata['confidence'] || metadata[:confidence]
      expect(confidence).to be >= min_confidence if confidence
    end

    def self.assert_metadata_expectation(result, path, expectation)
      metadata = result.metadata || {}
      value = fetch_metadata_value(metadata, path)
      raise "Metadata path '#{path}' missing in #{metadata.inspect}" if value.nil?

      # Handle simple values as implicit equality checks
      unless expectation.is_a?(Hash)
        expect(values_equal?(value, expectation)).to be(true)
        return
      end

      if expectation.key?(:eq)
        expect(values_equal?(value, expectation[:eq])).to be(true)
      end

      if expectation.key?(:gte)
        expect(convert_numeric(value)).to be >= convert_numeric(expectation[:gte])
      end

      if expectation.key?(:lte)
        expect(convert_numeric(value)).to be <= convert_numeric(expectation[:lte])
      end

      return unless expectation.key?(:contains)

      contains = expectation[:contains]
      if value.is_a?(String) && contains.is_a?(String)
        expect(value.include?(contains)).to be(true)
      elsif value.is_a?(Array) && contains.is_a?(String)
        expect(value.include?(contains)).to be(true)
      elsif value.is_a?(Array) && contains.is_a?(Array)
        expect(contains.all? { |item| value.include?(item) }).to be(true)
      else
        raise "Unsupported contains expectation for path '#{path}'"
      end
    end

    def self.assert_chunks(result, min_count: nil, max_count: nil, each_has_content: nil, each_has_embedding: nil)
      chunks = Array(result.chunks)
      expect(chunks.length).to be >= min_count if min_count
      expect(chunks.length).to be <= max_count if max_count
      if each_has_content
        chunks.each { |chunk| expect(chunk.content).not_to be_nil }
      end
      if each_has_embedding
        chunks.each { |chunk| expect(chunk.embedding).not_to be_nil }
      end
    end

    def self.assert_images(result, min_count: nil, max_count: nil, formats_include: nil)
      images = Array(result.images)
      expect(images.length).to be >= min_count if min_count
      expect(images.length).to be <= max_count if max_count
      if formats_include
        found_formats = images.map(&:format).compact.uniq
        formats_include.each do |fmt|
          expect(found_formats).to include(fmt)
        end
      end
    end

    def self.assert_pages(result, min_count: nil, exact_count: nil)
      pages = Array(result.pages)
      expect(pages.length).to be >= min_count if min_count
      expect(pages.length).to eq(exact_count) if exact_count
    end

    def self.assert_elements(result, min_count: nil, types_include: nil)
      elements = Array(result.elements)
      expect(elements.length).to be >= min_count if min_count
      if types_include
        found_types = elements.map(&:type).compact.uniq
        types_include.each do |t|
          expect(found_types).to include(t)
        end
      end
    end

    class << self
      private

      def fetch_metadata_value(metadata, path)
        value = lookup_metadata_path(metadata, path)
        return value unless value.nil?

        format = metadata['format'] || metadata[:format]
        return nil unless format.is_a?(Hash)

        lookup_metadata_path(format, path)
      end

      def lookup_metadata_path(metadata, path)
        current = metadata
        path.split('.').each do |segment|
          return nil unless current.is_a?(Hash)

          current = current[segment] || current[segment.to_sym]
        end
        current
      end

      def values_equal?(lhs, rhs)
        return lhs == rhs if lhs.is_a?(String) && rhs.is_a?(String)
        return convert_numeric(lhs) == convert_numeric(rhs) if numeric_like?(lhs) && numeric_like?(rhs)
        return lhs == rhs if lhs == rhs

        lhs == rhs
      end

      def numeric_like?(value)
        value.is_a?(Numeric) || value.respond_to?(:to_f)
      end

      def convert_numeric(value)
        return value if value.is_a?(Numeric)

        value.to_f
      end
    end
  end
end
# rubocop:enable Metrics/AbcSize, Metrics/CyclomaticComplexity, Metrics/MethodLength, Metrics/PerceivedComplexity, Metrics/ParameterLists, Style/Documentation, Style/IfUnlessModifier, Layout/LineLength, Layout/EmptyLineAfterGuardClause
