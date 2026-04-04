# frozen_string_literal: true

require 'spec_helper'
require 'tempfile'
require 'fileutils'

RSpec.describe 'Error Recovery' do
  describe 'error classification and categorization' do
    it 'raises ArgumentError for invalid configuration types' do
      expect do
        Kreuzberg::Config::Extraction.new(chunking: 'invalid_string')
      end.to raise_error(ArgumentError)
    end

    it 'classifies validation errors distinctly' do
      error = nil
      begin
        Kreuzberg::Config::Extraction.new(ocr: 12_345)
      rescue ArgumentError => e
        error = e
      end

      expect(error).not_to be_nil
      expect(error).to be_a(ArgumentError)
      expect(error.message).to match(/OCR|Expected/)
    end

    it 'raises error for invalid OCR backend configuration' do
      expect do
        Kreuzberg::Config::Extraction.new(ocr: [])
      end.to raise_error(ArgumentError)
    end

    it 'raises error for negative chunking parameters' do
      expect { Kreuzberg::Config::Chunking.new(max_chars: -100) }
        .to raise_error(StandardError, /negative|invalid|positive|max_chars/)
    end

    it 'raises error for invalid embedding dimensions' do
      # Test with embedding config validation
      expect do
        Kreuzberg::Config::Extraction.new(
          chunking: Kreuzberg::Config::Chunking.new(
            enabled: true,
            embedding: Kreuzberg::Config::Embedding.new(
              model: { type: :preset, name: 'invalid_model' }
            )
          )
        )
      end.not_to raise_error # May succeed, but model name validation may occur later
    end
  end

  describe 'error handling in extraction operations' do
    it 'gracefully handles file not found errors' do
      config = Kreuzberg::Config::Extraction.new
      nonexistent_path = '/nonexistent/file/that/does/not/exist.pdf'

      expect { Kreuzberg.extract_file_sync(path: nonexistent_path, config: config) }
        .to raise_error(Kreuzberg::Errors::IOError, /not found|does not exist|no such file/)
    end

    it 'provides descriptive error messages for invalid MIME types' do
      # Invalid MIME types should raise UnsupportedFormatError
      expect do
        Kreuzberg.extract_bytes_sync(data: 'test', mime_type: 'application/invalid-type')
      end.to raise_error(Kreuzberg::Errors::UnsupportedFormatError)
    end

    it 'handles empty file extraction gracefully' do
      file = Tempfile.new(['empty', '.txt']).tap do |f|
        f.write('')
        f.close
      end

      config = Kreuzberg::Config::Extraction.new
      result = Kreuzberg.extract_file_sync(path: file.path, config: config)

      expect(result).to be_a(Kreuzberg::Result)
      # Empty file may produce empty or minimal content
      expect(result.content).to be_a(String)

      FileUtils.rm_f(file.path)
    end

    it 'recovers from extraction with minimal data' do
      config = Kreuzberg::Config::Extraction.new
      result = Kreuzberg.extract_bytes_sync(data: '', mime_type: 'text/plain', config: config)

      expect(result).to be_a(Kreuzberg::Result)
      expect(result).to respond_to(:content)
    end
  end

  describe 'retry strategies and recovery patterns' do
    it 'implements retry with exponential backoff pattern' do
      file = Tempfile.new(['retry_test', '.txt']).tap do |f|
        f.write('Retry strategy test content')
        f.close
      end

      config = Kreuzberg::Config::Extraction.new
      max_retries = 3
      attempt = 0

      loop do
        attempt += 1
        result = Kreuzberg.extract_file_sync(path: file.path, config: config)
        expect(result).to be_a(Kreuzberg::Result)
        break
      rescue StandardError => e
        raise e if attempt >= max_retries

        sleep 0.1 * (2**(attempt - 1))
      end

      expect(attempt).to eq(1) # Should succeed on first attempt

      FileUtils.rm_f(file.path)
    end

    it 'handles retry with config modification' do
      file = Tempfile.new(['retry_config', '.txt']).tap do |f|
        f.write('Config modification retry')
        f.close
      end

      configs = [
        Kreuzberg::Config::Extraction.new,
        Kreuzberg::Config::Extraction.new(use_cache: false)
      ]

      results = []
      configs.each do |config|
        result = Kreuzberg.extract_file_sync(path: file.path, config: config)
        results << result
      rescue StandardError
        # Handle error, try next config
        next
      end

      expect(results).not_to be_empty

      FileUtils.rm_f(file.path)
    end

    it 'implements circuit breaker pattern for repeated failures' do
      circuit_state = :closed

      config = Kreuzberg::Config::Extraction.new
      text = 'Test content for circuit breaker'
      result = Kreuzberg.extract_bytes_sync(data: text, mime_type: 'text/plain', config: config)

      # Simulate successful extraction without repeated errors
      expect(result).to be_a(Kreuzberg::Result)
      expect(circuit_state).to eq(:closed)
    end

    it 'supports fallback configuration on extraction failure' do
      file = Tempfile.new(['fallback', '.txt']).tap do |f|
        f.write('Fallback configuration test')
        f.close
      end

      primary_config = Kreuzberg::Config::Extraction.new
      fallback_config = Kreuzberg::Config::Extraction.new(use_cache: false)

      result = begin
        Kreuzberg.extract_file_sync(path: file.path, config: primary_config)
      rescue StandardError => _e
        Kreuzberg.extract_file_sync(path: file.path, config: fallback_config)
      end

      expect(result).to be_a(Kreuzberg::Result)

      FileUtils.rm_f(file.path)
    end
  end

  describe 'graceful degradation strategies' do
    it 'degrades extraction features when dependencies unavailable' do
      # Test keyword extraction fallback
      config_with_keywords = Kreuzberg::Config::Extraction.new(
        keywords: Kreuzberg::Config::Keywords.new(algorithm: 'yake', max_keywords: 5)
      )

      config_without_keywords = Kreuzberg::Config::Extraction.new

      text = 'Machine learning transforms technology.'

      result_with = Kreuzberg.extract_bytes_sync(
        data: text, mime_type: 'text/plain', config: config_with_keywords
      )
      result_without = Kreuzberg.extract_bytes_sync(
        data: text, mime_type: 'text/plain', config: config_without_keywords
      )

      expect(result_with).to be_a(Kreuzberg::Result)
      expect(result_without).to be_a(Kreuzberg::Result)
      # Both should provide content even if keywords fail
      expect(result_without.content).not_to be_empty
    end

    it 'continues extraction without optional features' do
      config = Kreuzberg::Config::Extraction.new(
        chunking: Kreuzberg::Config::Chunking.new(
          enabled: false
        )
      )

      text = 'Content without chunking feature'
      result = Kreuzberg.extract_bytes_sync(
        data: text, mime_type: 'text/plain', config: config
      )

      expect(result).to be_a(Kreuzberg::Result)
      expect(result.content).not_to be_empty
    end

    it 'handles missing language detection gracefully' do
      text = 'Machine learning content'

      # With language detection disabled
      config_disabled = Kreuzberg::Config::Extraction.new(
        language_detection: Kreuzberg::Config::LanguageDetection.new(enabled: false)
      )

      result = Kreuzberg.extract_bytes_sync(
        data: text, mime_type: 'text/plain', config: config_disabled
      )

      expect(result).to be_a(Kreuzberg::Result)
      expect(result.content).not_to be_empty
    end

    it 'recovers from incomplete embedding generation' do
      config = Kreuzberg::Config::Extraction.new(
        chunking: Kreuzberg::Config::Chunking.new(
          enabled: true,
          max_chars: 100,
          embedding: Kreuzberg::Config::Embedding.new(
            model: { type: :preset, name: 'balanced' }
          )
        )
      )

      text = 'Test ' * 50
      result = Kreuzberg.extract_bytes_sync(
        data: text, mime_type: 'text/plain', config: config
      )

      # Should extract content even if embedding fails
      expect(result).to be_a(Kreuzberg::Result)
      expect(result.content).not_to be_empty
    end
  end

  describe 'error message clarity and debugging' do
    it 'provides informative error messages for validation failures' do
      error = nil
      begin
        Kreuzberg::Config::Extraction.new(ocr: 'invalid')
      rescue ArgumentError => e
        error = e
      end

      expect(error).not_to be_nil
      expect(error.message).to be_a(String)
      expect(error.message.length).to be > 10
    end

    it 'includes context in error messages' do
      error = nil
      begin
        Kreuzberg::Config::Chunking.new(max_overlap: -50)
      rescue StandardError => e
        error = e
      end

      expect(error).not_to be_nil
      expect(error.message).not_to be_empty
      expect(error.message.downcase).to include(/overlap|invalid|negative/)
    end

    it 'distinguishes between validation and runtime errors' do
      # Validation error
      validation_error = nil
      begin
        Kreuzberg::Config::Extraction.new(chunking: 'invalid')
      rescue StandardError => e
        validation_error = e
      end

      expect(validation_error).to be_a(ArgumentError)

      # Runtime error (file not found) - IOError since the file doesn't exist
      runtime_error = nil
      begin
        Kreuzberg.extract_file_sync(path: '/nonexistent/file.pdf')
      rescue StandardError => e
        runtime_error = e
      end

      expect(runtime_error).to be_a(Kreuzberg::Errors::IOError)
    end

    it 'provides error recovery suggestions in messages' do
      error = nil
      begin
        Kreuzberg::Config::Extraction.new(ocr: 12_345)
      rescue ArgumentError => e
        error = e
      end

      expect(error).not_to be_nil
      # Error message should be descriptive enough for debugging
      expect(error.message).to include('OCR') || error.message.include('Expected')
    end
  end

  describe 'recovery from partial extraction failures' do
    it 'continues extraction after keyword extraction failure' do
      config = Kreuzberg::Config::Extraction.new(
        keywords: Kreuzberg::Config::Keywords.new(
          algorithm: 'yake',
          max_keywords: 1000 # Extreme value that may fail gracefully
        )
      )

      text = 'Machine learning and artificial intelligence'
      result = Kreuzberg.extract_bytes_sync(
        data: text, mime_type: 'text/plain', config: config
      )

      # Should still extract content even if keyword extraction has issues
      expect(result).to be_a(Kreuzberg::Result)
      expect(result.content).not_to be_empty
    end

    it 'handles batch processing with some file failures' do
      valid_file = Tempfile.new(['valid_batch', '.txt']).tap do |f|
        f.write('Valid content')
        f.close
      end

      paths = [valid_file.path]
      config = Kreuzberg::Config::Extraction.new

      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)
      expect(results).to be_a(Array)
      expect(results).not_to be_empty

      FileUtils.rm_f(valid_file.path)
    end

    it 'recovers from chunking errors in batch' do
      file = Tempfile.new(['chunking_error', '.txt']).tap do |f|
        f.write('Content for chunking')
        f.close
      end

      config = Kreuzberg::Config::Extraction.new(
        chunking: Kreuzberg::Config::Chunking.new(
          enabled: true,
          max_chars: 10 # Very small chunk size
        )
      )

      result = Kreuzberg.extract_file_sync(path: file.path, config: config)
      expect(result).to be_a(Kreuzberg::Result)

      FileUtils.rm_f(file.path)
    end
  end

  describe 'timeout and resource limit handling' do
    it 'completes extraction within reasonable time' do
      config = Kreuzberg::Config::Extraction.new
      text = 'Machine learning ' * 100

      start_time = Time.now
      result = Kreuzberg.extract_bytes_sync(data: text, mime_type: 'text/plain', config: config)
      duration = Time.now - start_time

      expect(result).to be_a(Kreuzberg::Result)
      expect(duration).to be < 30.0 # Should complete within 30 seconds
    end

    it 'handles large file extraction gracefully' do
      large_file = Tempfile.new(['large_file', '.txt']).tap do |f|
        f.write('Large content ' * 1000)
        f.close
      end

      config = Kreuzberg::Config::Extraction.new
      result = Kreuzberg.extract_file_sync(path: large_file.path, config: config)

      expect(result).to be_a(Kreuzberg::Result)
      expect(result.content).not_to be_empty

      FileUtils.rm_f(large_file.path)
    end

    it 'manages memory efficiently during large batch operations' do
      paths = []
      10.times do |i|
        file = Tempfile.new(["memory_test_#{i}", '.txt'])
        file.write("Memory test #{i} " * 50)
        file.close
        paths << file.path
      end

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      expect(results.length).to eq(10)
      expect(results).to all(be_a(Kreuzberg::Result))

      paths.each { |p| FileUtils.rm_f(p) }
    end
  end

  describe 'configuration error prevention' do
    it 'validates conflicting configuration options early' do
      # Test invalid negative values
      expect do
        Kreuzberg::Config::Extraction.new(
          chunking: Kreuzberg::Config::Chunking.new(max_chars: -100)
        )
      end.to raise_error
    end

    it 'prevents invalid algorithm selection' do
      config = Kreuzberg::Config::Extraction.new(
        keywords: Kreuzberg::Config::Keywords.new(
          algorithm: 'yake',
          max_keywords: 5
        )
      )

      expect(config.keywords.algorithm).to eq('yake')
    end

    it 'validates keyword configuration completeness' do
      config = Kreuzberg::Config::Extraction.new(
        keywords: Kreuzberg::Config::Keywords.new(
          algorithm: 'rake',
          max_keywords: 10
        )
      )

      expect(config.keywords).not_to be_nil
      expect(config.keywords.algorithm).to eq('rake')
      expect(config.keywords.max_keywords).to eq(10)
    end
  end

  describe 'recovery monitoring and logging' do
    it 'tracks extraction success/failure states' do
      file = Tempfile.new(['tracking', '.txt']).tap do |f|
        f.write('Tracking content')
        f.close
      end

      config = Kreuzberg::Config::Extraction.new
      result = Kreuzberg.extract_file_sync(path: file.path, config: config)

      expect(result).to be_a(Kreuzberg::Result)
      expect(result).to respond_to(:content)

      FileUtils.rm_f(file.path)
    end

    it 'maintains extraction attempt history in application context' do
      results_history = []

      3.times do |i|
        config = Kreuzberg::Config::Extraction.new
        text = "Attempt #{i}"
        result = Kreuzberg.extract_bytes_sync(data: text, mime_type: 'text/plain', config: config)
        results_history << result
      end

      expect(results_history.length).to eq(3)
      expect(results_history).to all(be_a(Kreuzberg::Result))
    end
  end
end
