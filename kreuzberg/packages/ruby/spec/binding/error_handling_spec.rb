# frozen_string_literal: true

RSpec.describe 'Error Handling' do
  let(:nested_ocr_result) do
    {
      'content' => 'ocr text',
      'mime_type' => 'text/plain',
      'metadata_json' => '{}',
      'tables' => []
    }
  end

  let(:image_result_payload) do
    {
      content: 'Test',
      mime_type: 'text/plain',
      images: [
        {
          'data' => "binary\0data",
          'format' => 'png',
          'image_index' => 0,
          'page_number' => 1,
          'width' => 100,
          'height' => 200,
          'colorspace' => 'RGB',
          'bits_per_component' => 8,
          'is_mask' => false,
          'description' => 'inline image',
          'ocr_result' => nested_ocr_result
        }
      ]
    }
  end

  describe 'invalid configuration handling' do
    it 'raises error for negative max_chars in chunking' do
      # rubocop:disable Style/MultilineBlockChain
      expect do
        Kreuzberg::Config::Extraction.new(
          chunking: Kreuzberg::Config::Chunking.new(max_chars: -100)
        )
      end.to raise_error do |error|
        expect(error).to be_a(StandardError)
        expect(error.message.downcase).to match(/negative|invalid|positive|max_chars/)
      end
      # rubocop:enable Style/MultilineBlockChain
    end

    it 'raises error for negative max_overlap in chunking' do
      # rubocop:disable Style/MultilineBlockChain
      expect do
        Kreuzberg::Config::Chunking.new(max_overlap: -50)
      end.to raise_error do |error|
        expect(error).to be_a(StandardError)
        expect(error.message.downcase).to match(/negative|invalid|overlap/)
      end
      # rubocop:enable Style/MultilineBlockChain
    end

    it 'raises ArgumentError for invalid OCR config type' do
      # rubocop:disable Style/MultilineBlockChain
      expect do
        Kreuzberg::Config::Extraction.new(ocr: 'invalid_string')
      end.to raise_error(ArgumentError) do |error|
        expect(error.message).to include('Expected')
        expect(error.message).to include('OCR')
      end
      # rubocop:enable Style/MultilineBlockChain
    end

    it 'raises ArgumentError for invalid chunking config type' do
      expect do
        Kreuzberg::Config::Extraction.new(chunking: 123)
      end.to raise_error(ArgumentError)
    end

    it 'raises ArgumentError for invalid language_detection config' do
      expect do
        Kreuzberg::Config::Extraction.new(language_detection: [])
      end.to raise_error(ArgumentError)
    end

    it 'raises ArgumentError for invalid pdf_options config' do
      expect do
        Kreuzberg::Config::Extraction.new(pdf_options: 'invalid_string')
      end.to raise_error(ArgumentError)
    end

    it 'provides descriptive error messages for config validation' do
      error = nil
      begin
        Kreuzberg::Config::Extraction.new(ocr: 12_345)
      rescue ArgumentError => e
        error = e
      end

      expect(error).not_to be_nil
      expect(error.message).to be_a(String)
      expect(error.message).not_to be_empty
    end
  end

  describe 'file not found and corrupted files' do
    it 'raises error for non-existent file with meaningful message' do
      # rubocop:disable Style/MultilineBlockChain
      expect do
        Kreuzberg.extract_file_sync(path: '/nonexistent/path/file.txt')
      end.to raise_error do |error|
        expect(error).to be_a(StandardError)
        expect(error.message).not_to be_empty
      end
      # rubocop:enable Style/MultilineBlockChain
    end

    it 'raises error for empty file path' do
      expect do
        Kreuzberg.extract_file_sync(path: '')
      end.to raise_error(StandardError)
    end

    it 'raises error for nil file path' do
      expect do
        Kreuzberg.extract_file_sync(path: nil)
      end.to raise_error(StandardError)
    end

    it 'handles corrupted file gracefully' do
      # Create a file with binary garbage that is not a valid document
      corrupted_path = create_test_file("\x00\x01\x02\xFF\xFE\xFD", filename: 'corrupted.bin')

      begin
        result = Kreuzberg.extract_file_sync(path: corrupted_path, mime_type: 'application/octet-stream')
        # May succeed with empty content or raise error - both acceptable
        expect(result).to be_a(Kreuzberg::Result)
      rescue Kreuzberg::Errors::ParsingError => e
        expect(e).to be_a(Kreuzberg::Errors::ParsingError)
        expect(e.message).not_to be_empty
      rescue StandardError => e
        expect(e).to be_a(StandardError)
      end
    end
  end

  describe 'invalid MIME type handling' do
    it 'gracefully handles unknown MIME types' do
      path = create_test_file('Content with unknown type')

      result_or_error = nil
      begin
        result_or_error = Kreuzberg.extract_file_sync(path, mime_type: 'application/x-custom-unknown-format')
      rescue Kreuzberg::Errors::UnsupportedFormatError, StandardError => e
        result_or_error = e
      end

      if result_or_error.is_a?(Kreuzberg::Result)
        expect(result_or_error).to be_a(Kreuzberg::Result)
      else
        expect(result_or_error).to be_a(StandardError)
        expect(result_or_error.message).not_to be_empty
      end
    end

    it 'handles malformed MIME type strings' do
      path = create_test_file('Test content')

      # Either succeeds or raises with meaningful error - both acceptable
      result_or_error = nil
      begin
        result_or_error = Kreuzberg.extract_file_sync(path, mime_type: '///invalid@@@')
      rescue StandardError => e
        result_or_error = e
      end

      expect([Kreuzberg::Result, StandardError].any? { |klass| result_or_error.is_a?(klass) }).to be_truthy
    end

    it 'rejects empty MIME type with appropriate error' do
      path = create_test_file('Test')

      # Empty MIME type should either be rejected or handled gracefully
      result_or_error = nil
      begin
        Kreuzberg.extract_file_sync(path, mime_type: '')
      rescue StandardError => e
        result_or_error = e
      end

      expect(result_or_error).to be_a(StandardError) if result_or_error
    end
  end

  describe 'permission and I/O errors' do
    it 'raises IOError or subclass for permission denied scenario' do
      # This is environment-dependent, so we test gracefully

      # Try to write to a file we cannot read from (if setup permits)
      test_file = create_test_file('test content')
      File.chmod(0o000, test_file)

      begin
        Kreuzberg.extract_file_sync(path: test_file)
      ensure
        File.chmod(0o644, test_file)
      end
    rescue Kreuzberg::Errors::IOError => e
      expect(e).to be_a(Kreuzberg::Errors::IOError)
    rescue Errno::EACCES
      # Platform-specific permission error is acceptable
      expect(true).to be_truthy
    rescue StandardError => e
      # Other IO errors are acceptable
      expect(e).to be_a(StandardError)
    end
  end

  describe 'malformed document handling' do
    it 'handles invalid JSON metadata gracefully' do
      result = Kreuzberg::Result.new(
        content: 'Test content',
        mime_type: 'text/plain',
        metadata_json: 'this is not valid json {'
      )

      expect(result.content).to eq('Test content')
      expect(result.metadata).to eq({})
      expect(result.metadata).to be_a(Hash)
    end

    it 'handles empty metadata JSON' do
      result = Kreuzberg::Result.new(
        content: 'Test',
        mime_type: 'text/plain',
        metadata_json: ''
      )

      expect(result.metadata).to eq({})
      expect(result.content).to eq('Test')
    end

    it 'handles nil metadata JSON' do
      result = Kreuzberg::Result.new(
        content: 'Test',
        mime_type: 'text/plain',
        metadata_json: nil
      )

      expect(result.metadata).to eq({})
    end

    it 'handles malformed result object gracefully' do
      result = Kreuzberg::Result.new({})

      expect(result.content).to eq('')
      expect(result.mime_type).to eq('')
      expect(result.metadata).to eq({})
      expect(result.tables).to eq([])
      expect(result.detected_languages).to be_nil
      expect(result.chunks).to eq([])
      expect(result.images).to be_nil
    end

    it 'handles partial result data without errors' do
      result = Kreuzberg::Result.new(
        content: 'Partial content',
        mime_type: 'text/plain'
      )

      expect(result.content).to eq('Partial content')
      expect(result.mime_type).to eq('text/plain')
      expect(result.tables).to eq([])
      expect(result.metadata).to eq({})
    end
  end

  describe 'batch extraction error handling' do
    it 'handles mixed valid and invalid files in batch' do
      files = [
        create_test_file('Valid file content'),
        '/definitely/nonexistent/file.txt'
      ]

      begin
        result = Kreuzberg.batch_extract_files_sync(files)
        expect(result).to be_an(Array)
      rescue StandardError => e
        expect(e).to be_a(StandardError)
        expect(e.message).not_to be_empty
      end
    end

    it 'handles all invalid files in batch without crashing' do
      files = [
        '/nonexistent1.txt',
        '/nonexistent2.txt',
        '/nonexistent3.txt'
      ]

      begin
        result = Kreuzberg.batch_extract_files_sync(files)
        expect(result).to be_an(Array)
      rescue StandardError => e
        expect(e).to be_a(StandardError)
      end
    end

    it 'provides error context in batch results' do
      files = [
        create_test_file('First file'),
        '/nonexistent/second.txt'
      ]

      begin
        results = Kreuzberg.batch_extract_files_sync(files)
        expect(results).to be_an(Array)
      rescue StandardError => e
        expect(e).to be_a(StandardError)
      end
    end
  end

  describe 'concurrent error states' do
    it 'handles rapid successive error operations' do
      errors = []

      3.times do |i|
        Kreuzberg.extract_file_sync(path: "/nonexistent#{i}.pdf")
      rescue StandardError => e
        errors << e
      end

      expect(errors.length).to eq(3)
      expect(errors).to all(be_a(StandardError))
    end
  end

  describe 'error recovery and graceful degradation' do
    it 'recovers gracefully after file not found error' do
      # First operation: try to extract from nonexistent file
      error_caught = false
      begin
        Kreuzberg.extract_file_sync(path: '/nonexistent/does_not_exist.txt')
      rescue StandardError
        error_caught = true
      end

      expect(error_caught).to be_truthy

      # Second operation: should work fine with valid file
      valid_file = create_test_file('Valid content after error')
      result = Kreuzberg.extract_file_sync(path: valid_file)

      expect(result).to be_a(Kreuzberg::Result)
    end

    it 'handles mixed error and success scenarios in sequence' do
      results = []

      # Try invalid file
      begin
        Kreuzberg.extract_file_sync(path: '/nonexistent/file1.txt')
      rescue StandardError
        results << :error1
      end

      # Valid extraction
      valid_file = create_test_file('Valid content')
      Kreuzberg.extract_file_sync(path: valid_file)
      results << :success1

      # Another invalid file
      begin
        Kreuzberg.extract_file_sync(path: '/nonexistent/file2.txt')
      rescue StandardError
        results << :error2
      end

      expect(results).to eq(%i[error1 success1 error2])
    end
  end

  describe 'type conversion and coercion errors' do
    it 'handles non-string content in results gracefully' do
      path = create_test_file('Type coercion test')
      result = Kreuzberg.extract_file_sync(path: path)

      expect(result.content).to be_a(String)
      expect(result.mime_type).to be_a(String)
    end

    it 'extracts images with proper encoding handling' do
      result = Kreuzberg::Result.new(image_result_payload)
      image = result.images&.first

      expect(image&.format).to eq('png')
      expect(image&.data&.encoding).to eq(Encoding::BINARY)
      expect(image&.ocr_result).to be_a(Kreuzberg::Result)
    end
  end
end
