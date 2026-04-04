# frozen_string_literal: true

require 'spec_helper'
require 'tempfile'
require 'fileutils'

RSpec.describe 'Async Operations' do
  describe 'Fiber-based async extraction patterns' do
    it 'extracts content asynchronously using Fiber' do
      fiber = Fiber.new do
        config = Kreuzberg::Config::Extraction.new
        text = 'Machine learning transforms technology globally.'
        result = Kreuzberg.extract_bytes_sync(data: text, mime_type: 'text/plain', config: config)

        expect(result).not_to be_nil
        expect(result.content).not_to be_nil
        result
      end

      result = fiber.resume
      expect(result).to be_a(Kreuzberg::Result)
    end

    it 'handles multiple concurrent Fibers with different configs' do
      configs = [
        Kreuzberg::Config::Extraction.new(
          keywords: Kreuzberg::Config::Keywords.new(algorithm: 'yake', max_keywords: 5)
        ),
        Kreuzberg::Config::Extraction.new(
          keywords: Kreuzberg::Config::Keywords.new(algorithm: 'rake', max_keywords: 5)
        )
      ]

      text = 'Artificial intelligence and machine learning drive innovation.'

      fibers = configs.map do |config|
        Fiber.new do
          Kreuzberg.extract_bytes_sync(data: text, mime_type: 'text/plain', config: config)
        end
      end

      results = fibers.map(&:resume)
      expect(results.length).to eq(2)
      expect(results).to all(be_a(Kreuzberg::Result))
    end

    it 'maintains context across Fiber yielding' do
      accumulated_results = []

      texts = [
        'Machine learning enables predictions.',
        'Deep learning powers neural networks.',
        'Data science transforms insights.'
      ]

      fiber = Fiber.new do
        config = Kreuzberg::Config::Extraction.new
        texts.each do |text|
          result = Kreuzberg.extract_bytes_sync(data: text, mime_type: 'text/plain', config: config)
          accumulated_results << result
          Fiber.yield result
        end
        accumulated_results
      end

      expect(fiber.resume).to be_a(Kreuzberg::Result)
      expect(fiber.resume).to be_a(Kreuzberg::Result)
      expect(fiber.resume).to be_a(Kreuzberg::Result)
      final = fiber.resume
      expect(final).to be_a(Array)
      expect(final.length).to eq(3)
    end

    it 'handles Fiber with configuration updates' do
      fiber = Fiber.new do
        text = 'Artificial intelligence transforms technology.'

        # First extraction with YAKE
        config1 = Kreuzberg::Config::Extraction.new(
          keywords: Kreuzberg::Config::Keywords.new(algorithm: 'yake', max_keywords: 5)
        )
        result1 = Kreuzberg.extract_bytes_sync(data: text, mime_type: 'text/plain', config: config1)
        Fiber.yield result1

        # Second extraction with RAKE
        config2 = Kreuzberg::Config::Extraction.new(
          keywords: Kreuzberg::Config::Keywords.new(algorithm: 'rake', max_keywords: 5)
        )
        result2 = Kreuzberg.extract_bytes_sync(data: text, mime_type: 'text/plain', config: config2)
        result2
      end

      result1 = fiber.resume
      expect(result1).to be_a(Kreuzberg::Result)

      result2 = fiber.resume
      expect(result2).to be_a(Kreuzberg::Result)
    end
  end

  describe 'concurrent extraction operations' do
    it 'processes multiple extractions sequentially with Fiber control' do
      paths = []
      3.times do |i|
        file = Tempfile.new(["concurrent_#{i}", '.txt'])
        file.write("Content #{i}: Machine learning and artificial intelligence")
        file.close
        paths << file.path
      end

      results = []
      fiber = Fiber.new do
        config = Kreuzberg::Config::Extraction.new
        paths.each do |path|
          result = Kreuzberg.extract_file_sync(path: path, config: config)
          results << result
          Fiber.yield result
        end
        results
      end

      paths.each do
        expect(fiber.resume).to be_a(Kreuzberg::Result)
      end

      final = fiber.resume
      expect(final.length).to eq(3)

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'extracts files with different configurations in Fiber' do
      file = Tempfile.new(['fiber_test', '.txt'])
      file.write('Machine learning and neural networks enable AI advancement.')
      file.close

      configs = [
        Kreuzberg::Config::Extraction.new,
        Kreuzberg::Config::Extraction.new(
          keywords: Kreuzberg::Config::Keywords.new(algorithm: 'yake', max_keywords: 5)
        ),
        Kreuzberg::Config::Extraction.new(
          keywords: Kreuzberg::Config::Keywords.new(algorithm: 'rake', max_keywords: 10)
        )
      ]

      fiber = Fiber.new do
        results = []
        configs.each do |config|
          result = Kreuzberg.extract_file_sync(path: file.path, config: config)
          results << result
          Fiber.yield result
        end
        results
      end

      configs.each do
        expect(fiber.resume).to be_a(Kreuzberg::Result)
      end

      final = fiber.resume
      expect(final.length).to eq(3)

      FileUtils.rm_f(file.path)
    end

    it 'handles Fiber enumeration over extraction results' do
      texts = [
        'AI transforms industries',
        'Machine learning enables insights',
        'Data science drives decisions'
      ]

      fiber_results = texts.map do |text|
        Fiber.new do
          config = Kreuzberg::Config::Extraction.new
          Kreuzberg.extract_bytes_sync(data: text, mime_type: 'text/plain', config: config)
        end
      end

      results = fiber_results.map(&:resume)
      expect(results.length).to eq(3)
      expect(results).to all(be_a(Kreuzberg::Result))
    end
  end

  describe 'async error handling' do
    it 'catches errors within Fiber context' do
      fiber = Fiber.new do
        # Attempt extraction with invalid config
        Kreuzberg::Config::Extraction.new(chunking: 'invalid')
      rescue ArgumentError => e
        { error: true, message: e.message }
      end

      result = fiber.resume
      expect(result).to be_a(Hash)
      expect(result[:error]).to be true
    end

    it 'maintains Fiber execution on recoverable errors' do
      fiber = Fiber.new do
        results = []

        # First attempt - error
        begin
          Kreuzberg::Config::Extraction.new(ocr: 12_345)
        rescue ArgumentError
          results << 'first_error'
        end
        Fiber.yield results

        # Second attempt - success
        config = Kreuzberg::Config::Extraction.new
        text = 'Machine learning.'
        result = Kreuzberg.extract_bytes_sync(data: text, mime_type: 'text/plain', config: config)
        results << result
        results
      end

      first = fiber.resume
      expect(first).to include('first_error')

      final = fiber.resume
      expect(final).to include('first_error')
      expect(final).to include(a_kind_of(Kreuzberg::Result))
    end

    it 'handles Fiber exception propagation' do
      fiber = Fiber.new do
        raise StandardError, 'Test error in Fiber'
      end

      expect { fiber.resume }.to raise_error(StandardError, /Test error in Fiber/)
    end

    it 'recovers from async operation failures' do
      file = Tempfile.new(['error_recovery', '.txt'])
      file.write('Test content for recovery.')
      file.close

      fiber = Fiber.new do
        config = Kreuzberg::Config::Extraction.new

        # First try
        begin
          result = Kreuzberg.extract_file_sync(path: file.path, config: config)
          Fiber.yield result
        rescue StandardError => e
          error_hash = { error: e.message }
          Fiber.yield error_hash
        end

        # Retry
        retry_config = Kreuzberg::Config::Extraction.new(use_cache: false)
        Kreuzberg.extract_file_sync(path: file.path, config: retry_config)
      end

      first = fiber.resume
      expect([Kreuzberg::Result, Hash]).to include(first.class)

      second = fiber.resume
      expect(second).to be_a(Kreuzberg::Result)

      FileUtils.rm_f(file.path)
    end
  end

  describe 'async batch processing' do
    it 'processes batch with Fiber-based control' do
      paths = []
      3.times do |i|
        file = Tempfile.new(["batch_fiber_#{i}", '.txt'])
        file.write("Batch content #{i}")
        file.close
        paths << file.path
      end

      fiber = Fiber.new do
        config = Kreuzberg::Config::Extraction.new
        results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)
        Fiber.yield results.length
        results
      end

      count = fiber.resume
      expect(count).to eq(3)

      results = fiber.resume
      expect(results.length).to eq(3)
      expect(results).to all(be_a(Kreuzberg::Result))

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'manages batch extraction with progress tracking via Fiber' do
      paths = []
      5.times do |i|
        file = Tempfile.new(["progress_#{i}", '.txt'])
        file.write("Progress tracking #{i}")
        file.close
        paths << file.path
      end

      progress = []

      fiber = Fiber.new do
        config = Kreuzberg::Config::Extraction.new
        results = []

        paths.each_with_index do |path, idx|
          result = Kreuzberg.extract_file_sync(path: path, config: config)
          results << result
          progress << (idx + 1)
          Fiber.yield(idx + 1)
        end

        results
      end

      paths.length.times do
        progress_value = fiber.resume
        expect(progress_value).to be > 0
        expect(progress_value).to be <= paths.length
      end

      final = fiber.resume
      expect(final.length).to eq(paths.length)

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'handles Fiber-based enumerable processing of batch results' do
      paths = []
      4.times do |i|
        file = Tempfile.new(["enumerable_#{i}", '.txt'])
        file.write("Enumerable #{i}")
        file.close
        paths << file.path
      end

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      fiber = Fiber.new do
        processed = []
        results.each do |result|
          processed << result
          Fiber.yield processed.length
        end
        processed
      end

      results.length.times do
        count = fiber.resume
        expect(count).to be > 0
      end

      final = fiber.resume
      expect(final.length).to eq(results.length)

      paths.each { |p| FileUtils.rm_f(p) }
    end
  end

  describe 'async performance and resource management' do
    it 'yields control in Fiber during long extraction' do
      fiber = Fiber.new do
        config = Kreuzberg::Config::Extraction.new
        text = 'Machine learning ' * 100

        start_time = Time.now
        result = Kreuzberg.extract_bytes_sync(data: text, mime_type: 'text/plain', config: config)
        duration = Time.now - start_time

        Fiber.yield duration
        result
      end

      duration = fiber.resume
      expect(duration).to be > 0

      result = fiber.resume
      expect(result).to be_a(Kreuzberg::Result)
    end

    it 'maintains multiple Fiber contexts independently' do
      fiber1 = Fiber.new do
        config = Kreuzberg::Config::Extraction.new(
          keywords: Kreuzberg::Config::Keywords.new(algorithm: 'yake')
        )
        Kreuzberg.extract_bytes_sync(data: 'Fiber 1 content', mime_type: 'text/plain', config: config)
      end

      fiber2 = Fiber.new do
        config = Kreuzberg::Config::Extraction.new(
          keywords: Kreuzberg::Config::Keywords.new(algorithm: 'rake')
        )
        Kreuzberg.extract_bytes_sync(data: 'Fiber 2 content', mime_type: 'text/plain', config: config)
      end

      result1 = fiber1.resume
      result2 = fiber2.resume

      expect(result1).to be_a(Kreuzberg::Result)
      expect(result2).to be_a(Kreuzberg::Result)
      expect(result1.content).to include('Fiber 1')
      expect(result2.content).to include('Fiber 2')
    end

    it 'handles Fiber cleanup and resource management' do
      fiber = Fiber.new do
        paths = []
        3.times do |i|
          file = Tempfile.new(["cleanup_#{i}", '.txt'])
          file.write("Cleanup test #{i}")
          file.close
          paths << file.path
        end

        begin
          config = Kreuzberg::Config::Extraction.new
          results = paths.map { |p| Kreuzberg.extract_file_sync(path: p, config: config) }
          Fiber.yield results.length
          results
        ensure
          paths.each { |p| FileUtils.rm_f(p) }
        end
      end

      count = fiber.resume
      expect(count).to eq(3)

      results = fiber.resume
      expect(results.length).to eq(3)
    end
  end

  describe 'async with configuration variations' do
    it 'applies config changes in Fiber sequence' do
      fiber = Fiber.new do
        text = 'Machine learning transforms technology research and development.'

        # No keywords
        config1 = Kreuzberg::Config::Extraction.new
        result1 = Kreuzberg.extract_bytes_sync(data: text, mime_type: 'text/plain', config: config1)
        Fiber.yield result1.content.length

        # With YAKE
        config2 = Kreuzberg::Config::Extraction.new(
          keywords: Kreuzberg::Config::Keywords.new(algorithm: 'yake', max_keywords: 5)
        )
        result2 = Kreuzberg.extract_bytes_sync(data: text, mime_type: 'text/plain', config: config2)
        Fiber.yield result2.content.length

        # With RAKE
        config3 = Kreuzberg::Config::Extraction.new(
          keywords: Kreuzberg::Config::Keywords.new(algorithm: 'rake', max_keywords: 10)
        )
        result3 = Kreuzberg.extract_bytes_sync(data: text, mime_type: 'text/plain', config: config3)
        result3
      end

      len1 = fiber.resume
      len2 = fiber.resume
      result3 = fiber.resume

      expect(len1).to be > 0
      expect(len2).to be > 0
      expect(result3).to be_a(Kreuzberg::Result)
    end
  end
end
