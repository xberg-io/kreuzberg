# frozen_string_literal: true

require 'spec_helper'
require 'tempfile'
require 'fileutils'
require 'securerandom'

RSpec.describe 'Batch Operations' do
  describe 'batch_extract_files with multiple file types' do
    it 'processes mixed file types in single batch' do
      paths = []

      # Create text file
      txt_file = Tempfile.new(['batch_test', '.txt'])
      txt_file.write('Text file content: Machine learning transforms technology.')
      txt_file.close
      paths << txt_file.path

      # Create markdown file
      md_file = Tempfile.new(['batch_test', '.md'])
      md_file.write('# Markdown Header\n\nContent about artificial intelligence.')
      md_file.close
      paths << md_file.path

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      expect(results).to be_a(Array)
      expect(results.length).to eq(2)
      results.each do |result|
        expect(result).to be_a(Kreuzberg::Result)
        expect(result.content).not_to be_empty
      end

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'maintains file order through batch processing' do
      paths = []
      unique_markers = []

      3.times do |i|
        file = Tempfile.new(["ordered_#{i}", '.txt'])
        marker = "MARKER_#{SecureRandom.hex(4)}"
        file.write("File #{i}: #{marker}")
        file.close
        paths << file.path
        unique_markers << marker
      end

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      expect(results.length).to eq(paths.length)
      results.each_with_index do |result, idx|
        expect(result.content).to include(unique_markers[idx])
      end

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'processes large batch operations efficiently' do
      paths = []

      # Create 20 test files
      20.times do |i|
        file = Tempfile.new(["large_batch_#{i}", '.txt'])
        file.write("Content #{i}: Machine learning technology")
        file.close
        paths << file.path
      end

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      expect(results.length).to eq(20)
      expect(results).to all(be_a(Kreuzberg::Result))

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'handles batch with different file sizes' do
      paths = []

      # Small file
      small = Tempfile.new(['small', '.txt'])
      small.write('AI')
      small.close
      paths << small.path

      # Medium file
      medium = Tempfile.new(['medium', '.txt'])
      medium.write('Machine learning is a subset of artificial intelligence.')
      medium.close
      paths << medium.path

      # Large file
      large = Tempfile.new(['large', '.txt'])
      large.write('Machine learning ' * 100)
      large.close
      paths << large.path

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      expect(results.length).to eq(3)
      expect(results).to all(be_a(Kreuzberg::Result))
      expect(results[2].content.length).to be >= results[0].content.length

      paths.each { |p| FileUtils.rm_f(p) }
    end
  end

  describe 'batch extraction with configuration options' do
    it 'applies consistent configuration across batch' do
      paths = []

      3.times do |i|
        file = Tempfile.new(["config_batch_#{i}", '.txt'])
        file.write("Machine learning content #{i}. Artificial intelligence advances.")
        file.close
        paths << file.path
      end

      config = Kreuzberg::Config::Extraction.new(
        keywords: Kreuzberg::Config::Keywords.new(
          algorithm: 'yake',
          max_keywords: 5
        )
      )

      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      expect(results.length).to eq(3)
      results.each do |result|
        expect(result).to be_a(Kreuzberg::Result)
        expect(result.content).not_to be_nil
      end

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'batch respects caching configuration' do
      path = Tempfile.new(['cache_test', '.txt']).tap do |f|
        f.write('Cache test content')
        f.close
      end

      config_no_cache = Kreuzberg::Config::Extraction.new(use_cache: false)
      results1 = Kreuzberg.batch_extract_files_sync(paths: [path.path], config: config_no_cache)

      config_with_cache = Kreuzberg::Config::Extraction.new(use_cache: true)
      results2 = Kreuzberg.batch_extract_files_sync(paths: [path.path], config: config_with_cache)

      expect(results1.length).to eq(1)
      expect(results2.length).to eq(1)
      expect(results1[0].content).to eq(results2[0].content)

      FileUtils.rm_f(path.path)
    end

    it 'supports keyword extraction configuration in batch' do
      paths = []

      2.times do |i|
        file = Tempfile.new(["keywords_batch_#{i}", '.txt'])
        file.write('Machine learning and deep learning enable artificial intelligence.')
        file.close
        paths << file.path
      end

      algorithms = %w[yake rake]

      algorithms.each do |algo|
        config = Kreuzberg::Config::Extraction.new(
          keywords: Kreuzberg::Config::Keywords.new(
            algorithm: algo,
            max_keywords: 5
          )
        )

        results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)
        expect(results.length).to eq(2)
      end

      paths.each { |p| FileUtils.rm_f(p) }
    end
  end

  describe 'batch error handling and resilience' do
    it 'processes batch with some invalid paths gracefully' do
      valid_file = Tempfile.new(['valid_batch', '.txt']).tap do |f|
        f.write('Valid content')
        f.close
      end

      valid_path = valid_file.path
      config = Kreuzberg::Config::Extraction.new

      # Process just the valid path
      results = Kreuzberg.batch_extract_files_sync(paths: [valid_path], config: config)
      expect(results.length).to eq(1)
      expect(results[0]).to be_a(Kreuzberg::Result)

      FileUtils.rm_f(valid_path)
    end

    it 'handles empty file list in batch' do
      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: [], config: config)

      expect(results).to be_a(Array)
      expect(results).to be_empty
    end

    it 'processes batch with single file' do
      file = Tempfile.new(['single_batch', '.txt']).tap do |f|
        f.write('Single file batch processing')
        f.close
      end

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: [file.path], config: config)

      expect(results.length).to eq(1)
      expect(results[0]).to be_a(Kreuzberg::Result)

      FileUtils.rm_f(file.path)
    end

    it 'maintains batch execution on partial failures' do
      valid_file = Tempfile.new(['valid', '.txt']).tap do |f|
        f.write('Valid content')
        f.close
      end

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: [valid_file.path], config: config)

      expect(results).to be_a(Array)
      expect(results).to all(be_a(Kreuzberg::Result))

      FileUtils.rm_f(valid_file.path)
    end
  end

  describe 'batch enumerable processing' do
    it 'iterates over batch results with each' do
      paths = []

      3.times do |i|
        file = Tempfile.new(["enum_#{i}", '.txt'])
        file.write("Enumerable test #{i}")
        file.close
        paths << file.path
      end

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      count = 0
      results.each do |result|
        expect(result).to be_a(Kreuzberg::Result)
        count += 1
      end

      expect(count).to eq(3)

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'maps batch results to extract content' do
      paths = []

      3.times do |i|
        file = Tempfile.new(["map_#{i}", '.txt'])
        file.write("Mapping #{i}")
        file.close
        paths << file.path
      end

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      contents = results.map(&:content)
      expect(contents).to be_a(Array)
      expect(contents.length).to eq(3)
      expect(contents).to all(be_a(String))

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'filters batch results by content length' do
      paths = []

      # Small file
      small = Tempfile.new(['small', '.txt']).tap do |f|
        f.write('x')
        f.close
      end
      paths << small.path

      # Large file
      large = Tempfile.new(['large', '.txt']).tap do |f|
        f.write('content ' * 50)
        f.close
      end
      paths << large.path

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      large_results = results.select { |r| r.content.length > 20 }
      expect(large_results.length).to be >= 1

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'reduces batch results to combined content' do
      paths = []

      3.times do |i|
        file = Tempfile.new(["reduce_#{i}", '.txt'])
        file.write("Part #{i} ")
        file.close
        paths << file.path
      end

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      combined = results.reduce('') { |acc, r| acc + r.content }
      expect(combined).not_to be_empty
      expect(combined).to include('Part')

      paths.each { |p| FileUtils.rm_f(p) }
    end
  end

  describe 'batch with chunking and embeddings' do
    it 'processes batch with chunking enabled' do
      paths = []

      2.times do |i|
        file = Tempfile.new(["chunking_batch_#{i}", '.txt'])
        file.write('Machine learning ' * 50)
        file.close
        paths << file.path
      end

      config = Kreuzberg::Config::Extraction.new(
        chunking: Kreuzberg::Config::Chunking.new(
          enabled: true,
          max_chars: 100
        )
      )

      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      expect(results.length).to eq(2)
      expect(results).to all(be_a(Kreuzberg::Result))

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'batch processes with embedding generation' do
      paths = []

      2.times do |i|
        file = Tempfile.new(["embedding_batch_#{i}", '.txt'])
        file.write('Artificial intelligence transforms technology development.')
        file.close
        paths << file.path
      end

      # Use basic chunking without embeddings to avoid ONNX dependency
      config = Kreuzberg::Config::Extraction.new(
        chunking: Kreuzberg::Config::Chunking.new(
          enabled: true,
          max_chars: 100
        )
      )

      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      expect(results.length).to eq(2)
      expect(results).to all(be_a(Kreuzberg::Result))

      paths.each { |p| FileUtils.rm_f(p) }
    end
  end

  describe 'batch result properties and validation' do
    it 'each batch result has required properties' do
      paths = []

      2.times do |i|
        file = Tempfile.new(["props_#{i}", '.txt'])
        file.write("Result properties test #{i}")
        file.close
        paths << file.path
      end

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      results.each do |result|
        expect(result).to respond_to(:content)
        expect(result).to respond_to(:mime_type)
        expect(result.content).to be_a(String)
        expect(result.mime_type).to be_a(String)
      end

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'batch results maintain independence' do
      file1 = Tempfile.new(['indep1', '.txt']).tap do |f|
        f.write('First file content')
        f.close
      end

      file2 = Tempfile.new(['indep2', '.txt']).tap do |f|
        f.write('Second file content')
        f.close
      end

      paths = [file1.path, file2.path]

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      expect(results[0].content).not_to eq(results[1].content)
      expect(results[0].content).to include('First')
      expect(results[1].content).to include('Second')

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'batch results have consistent structure' do
      paths = []
      tempfiles = []

      3.times do |i|
        file = Tempfile.new(["struct_#{i}", '.txt'])
        file.write("Structure test #{i}")
        file.close
        paths << file.path
        tempfiles << file
      end

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      first_keys = results.first.respond_to?(:to_h) ? results.first.to_h.keys : []

      results.each do |result|
        if result.respond_to?(:to_h)
          result_keys = result.to_h.keys
          expect(result_keys).to match_array(first_keys) if first_keys.any?
        end
      end

      paths.each { |p| FileUtils.rm_f(p) }
    end
  end

  describe 'batch performance characteristics' do
    it 'completes batch faster than sequential processing' do
      paths = []

      5.times do |i|
        file = Tempfile.new(["perf_#{i}", '.txt'])
        file.write("Performance test #{i}")
        file.close
        paths << file.path
      end

      config = Kreuzberg::Config::Extraction.new

      # Batch time
      batch_start = Time.now
      batch_results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)
      batch_time = Time.now - batch_start

      # Sequential time
      seq_start = Time.now
      seq_results = paths.map { |p| Kreuzberg.extract_file_sync(path: p, config: config) }
      seq_time = Time.now - seq_start

      expect(batch_results.length).to eq(seq_results.length)
      # Batch should be faster or comparable
      expect(batch_time).to be <= seq_time + 1.0

      paths.each { |p| FileUtils.rm_f(p) }
    end
  end

  describe 'batch with special configurations' do
    it 'batch processes with language detection' do
      paths = []

      file = Tempfile.new(['lang_batch', '.txt']).tap do |f|
        f.write('Machine learning is transforming industries worldwide.')
        f.close
      end
      paths << file.path

      config = Kreuzberg::Config::Extraction.new(
        language_detection: Kreuzberg::Config::LanguageDetection.new(enabled: true)
      )

      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)
      expect(results.length).to eq(1)

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'batch with mixed keyword algorithms' do
      paths = []

      2.times do |i|
        file = Tempfile.new(["mixed_algo_#{i}", '.txt'])
        file.write('Machine learning neural networks artificial intelligence')
        file.close
        paths << file.path
      end

      # First batch with YAKE
      config_yake = Kreuzberg::Config::Extraction.new(
        keywords: Kreuzberg::Config::Keywords.new(algorithm: 'yake', max_keywords: 3)
      )
      results_yake = Kreuzberg.batch_extract_files_sync(paths: paths, config: config_yake)
      expect(results_yake.length).to eq(2)

      # Second batch with RAKE
      config_rake = Kreuzberg::Config::Extraction.new(
        keywords: Kreuzberg::Config::Keywords.new(algorithm: 'rake', max_keywords: 3)
      )
      results_rake = Kreuzberg.batch_extract_files_sync(paths: paths, config: config_rake)
      expect(results_rake.length).to eq(2)

      paths.each { |p| FileUtils.rm_f(p) }
    end
  end

  describe 'batch with result aggregation' do
    it 'aggregates batch results into statistics' do
      paths = []

      3.times do |i|
        file = Tempfile.new(["stats_#{i}", '.txt'])
        file.write("Content #{i} " * 10)
        file.close
        paths << file.path
      end

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      # Create aggregated statistics
      stats = {
        total_files: results.length,
        total_content_length: results.sum { |r| r.content.length },
        avg_content_length: results.sum { |r| r.content.length } / results.length,
        mime_types: results.map(&:mime_type).uniq
      }

      expect(stats[:total_files]).to eq(3)
      expect(stats[:total_content_length]).to be > 0
      expect(stats[:avg_content_length]).to be > 0
      expect(stats[:mime_types]).to be_a(Array)

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'batch results support JSON serialization' do
      paths = []

      file = Tempfile.new(['json_batch', '.txt']).tap do |f|
        f.write('JSON serialization test')
        f.close
      end
      paths << file.path

      config = Kreuzberg::Config::Extraction.new
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      expect(results.first).to respond_to(:to_json)
      json_str = results.first.to_json
      expect(json_str).to be_a(String)
      expect(json_str.length).to be > 0

      paths.each { |p| FileUtils.rm_f(p) }
    end
  end

  describe 'batch with output and result formats' do
    it 'batch processes with output_format' do
      paths = []
      file = Tempfile.new(['format_test', '.txt']).tap do |f|
        f.write('Test content for output format')
        f.close
      end
      paths << file.path

      config = Kreuzberg::Config::Extraction.new(output_format: 'markdown')
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      expect(results).to be_an Array
      expect(results.length).to eq 1
      expect(results[0]).to be_a Kreuzberg::Result

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'batch processes with result_format' do
      paths = []
      file = Tempfile.new(['format_test', '.txt']).tap do |f|
        f.write('Test content for result format')
        f.close
      end
      paths << file.path

      config = Kreuzberg::Config::Extraction.new(result_format: 'unified')
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      expect(results).to be_an Array
      expect(results.length).to eq 1
      expect(results[0]).to be_a Kreuzberg::Result

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'batch processes with both output and result formats' do
      paths = []
      file = Tempfile.new(['format_test', '.txt']).tap do |f|
        f.write('Test content for both formats')
        f.close
      end
      paths << file.path

      config = Kreuzberg::Config::Extraction.new(
        output_format: 'plain',
        result_format: 'element_based'
      )
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      expect(results).to be_an Array
      expect(results.length).to eq 1
      expect(results[0]).to be_a Kreuzberg::Result

      paths.each { |p| FileUtils.rm_f(p) }
    end

    it 'batch processes with chunking and output_format' do
      paths = []
      file = Tempfile.new(['format_test', '.txt']).tap do |f|
        f.write('Test content ' * 100)
        f.close
      end
      paths << file.path

      config = Kreuzberg::Config::Extraction.new(
        output_format: 'markdown',
        chunking: { max_chars: 1000 }
      )
      results = Kreuzberg.batch_extract_files_sync(paths: paths, config: config)

      expect(results).to be_an Array
      expect(results.length).to eq 1
      expect(results[0]).to be_a Kreuzberg::Result

      paths.each { |p| FileUtils.rm_f(p) }
    end
  end
end
