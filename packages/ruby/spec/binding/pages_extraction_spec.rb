# frozen_string_literal: true

RSpec.describe 'Pages Extraction' do
  describe 'Extract Pages' do
    it 'returns pages array when extractPages is true' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: true)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result).not_to be_nil
      expect(result.pages).not_to be_nil
      expect(result.pages).to be_a(Array)
    end

    it 'returns page numbers for each page' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: true)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.pages).not_to be_nil
      result.pages.each do |page|
        expect(page.page_number).to be > 0
      end
    end

    it 'returns page content for each page' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: true)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.pages).not_to be_nil
      result.pages.each do |page|
        expect(page.content).not_to be_nil
      end
    end

    it 'returns nil for pages when extractPages is false' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: false)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result).not_to be_nil
      expect(result.pages).to be_nil
    end

    it 'preserves page order' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: true)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      if result.pages && result.pages.length > 1
        (0...(result.pages.length - 1)).each do |i|
          expect(result.pages[i].page_number).to be < result.pages[i + 1].page_number
        end
      end
    end
  end

  describe 'Insert Page Markers' do
    it 'inserts page markers when insertPageMarkers is true' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(insert_page_markers: true)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result).not_to be_nil
      expect(result.content).not_to be_nil
      expect(result.content).to include('<!-- PAGE')
    end

    it 'does not insert markers when insertPageMarkers is false' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(insert_page_markers: false)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result).not_to be_nil
      # Default marker format should not appear when not enabled
      expect(result.content).not_to include('<!-- PAGE')
    end

    it 'contains page numbers in markers' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(insert_page_markers: true)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.content).not_to be_nil
      # Should contain at least page 1
      expect(result.content).to include('1')
    end

    it 'inserts multiple markers for multi-page documents' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(insert_page_markers: true)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.content).not_to be_nil
      marker_count = result.content.scan('<!-- PAGE').length
      expect(marker_count).to be > 0
    end
  end

  describe 'Custom Marker Format' do
    it 'uses custom marker format when specified' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      custom_format = '=== PAGE {page_num} ==='
      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(
          insert_page_markers: true,
          marker_format: custom_format
        )
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result).not_to be_nil
      expect(result.content).not_to be_nil
      expect(result.content).to include('=== PAGE')
    end

    it 'replaces page_num placeholder in custom format' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      custom_format = '[Page Number: {page_num}]'
      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(
          insert_page_markers: true,
          marker_format: custom_format
        )
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.content).not_to be_nil
      expect(result.content).to include('[Page Number:')
      expect(result.content).not_to include('{page_num}')
    end

    it 'handles simple custom format' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      custom_format = 'PAGE_{page_num}'
      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(
          insert_page_markers: true,
          marker_format: custom_format
        )
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.content).not_to be_nil
      expect(result.content).to include('PAGE_')
    end

    it 'handles custom format with line separators' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      custom_format = "\n---PAGE {page_num}---\n"
      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(
          insert_page_markers: true,
          marker_format: custom_format
        )
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.content).not_to be_nil
      expect(result.content).to include('---PAGE')
    end

    it 'overrides default marker format' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      custom_format = 'CUSTOM_PAGE_{page_num}'
      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(
          insert_page_markers: true,
          marker_format: custom_format
        )
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.content).not_to be_nil
      expect(result.content).to include('CUSTOM_PAGE_')
    end
  end

  describe 'Multi-Page PDF' do
    it 'produces multiple pages from multi-page PDF' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: true)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.pages).not_to be_nil
      expect(result.pages.length).to be > 0
    end

    it 'page numbers are sequential' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: true)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.pages).not_to be_nil
      result.pages.each_with_index do |page, index|
        expect(page.page_number).to eq(index + 1)
      end
    end

    it 'each page has content' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: true)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.pages).not_to be_nil
      result.pages.each do |page|
        expect(page.content).not_to be_nil
        expect(page.content.strip).not_to be_empty
      end
    end

    it 'with markers contains all pages' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(insert_page_markers: true)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.content).not_to be_nil
      marker_count = result.content.scan('<!-- PAGE').length
      expect(marker_count).to be >= 1
    end
  end

  describe 'Page Content Structure Validation' do
    it 'validates page structure' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: true)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.pages).not_to be_nil
      result.pages.each do |page|
        expect(page.content).not_to be_nil
        expect(page.page_number).to be > 0
      end
    end

    it 'page content has required fields' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: true)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.pages).not_to be_nil
      result.pages.each do |page|
        expect(page.page_number).to be > 0
        expect(page.content).not_to be_nil
        # is_blank should be nil or a boolean
        expect(page.is_blank).to be_nil.or be(true).or be(false) if page.respond_to?(:is_blank)
      end
    end

    it 'page content with tables preserves table data' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: true)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.pages).not_to be_nil
      result.pages.each do |page|
        # Tables in page content are optional
        expect(page.tables).to be_an(Array) if page.respond_to?(:tables) && page.tables
      end
    end

    it 'page content with images preserves image data' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: true)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.pages).not_to be_nil
      result.pages.each do |page|
        # Images in page content are optional
        expect(page.images).to be_an(Array) if page.respond_to?(:images) && page.images
      end
    end

    it 'page content is not empty' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: true)
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.pages).not_to be_nil
      page_with_content = result.pages.find { |p| p.content && !p.content.strip.empty? }
      expect(page_with_content).not_to be_nil
    end
  end

  describe 'Combined Features' do
    it 'extract pages and insert markers together' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(
          extract_pages: true,
          insert_page_markers: true
        )
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result).not_to be_nil
      expect(result.pages).not_to be_nil
      expect(result.pages.length).to be > 0
      expect(result.content).to include('<!-- PAGE')
    end

    it 'extract pages with custom marker format' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(
          extract_pages: true,
          insert_page_markers: true,
          marker_format: '[PAGE {page_num}]'
        )
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.pages).not_to be_nil
      expect(result.pages.length).to be > 0
      expect(result.content).to include('[PAGE')
    end

    it 'page extraction consistency between array and markers' do
      pdf_file = test_document_path('pdf/sample_contract.pdf')
      skip "Test PDF not available at #{pdf_file}" unless File.exist?(pdf_file)

      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(
          extract_pages: true,
          insert_page_markers: true
        )
      )

      result = Kreuzberg.extract_file_sync(path: pdf_file, config: config)

      expect(result.pages).not_to be_nil
      expect(result.content).not_to be_nil

      page_array_count = result.pages.length
      marker_count = result.content.scan('<!-- PAGE').length

      expect(page_array_count).to eq(marker_count)
    end
  end

  describe 'PageConfig' do
    it 'creates with default values' do
      config = Kreuzberg::Config::PageConfig.new

      expect(config.extract_pages).to be false
      expect(config.insert_page_markers).to be false
      expect(config.marker_format).to match(/<!-- PAGE/)
    end

    it 'creates with custom values' do
      config = Kreuzberg::Config::PageConfig.new(
        extract_pages: true,
        insert_page_markers: true,
        marker_format: 'CUSTOM_{page_num}'
      )

      expect(config.extract_pages).to be true
      expect(config.insert_page_markers).to be true
      expect(config.marker_format).to eq('CUSTOM_{page_num}')
    end

    it 'converts to hash' do
      config = Kreuzberg::Config::PageConfig.new(
        extract_pages: true,
        insert_page_markers: false,
        marker_format: 'TEST_{page_num}'
      )

      hash = config.to_h

      expect(hash).to be_a(Hash)
      expect(hash[:extract_pages]).to be true
      expect(hash[:insert_page_markers]).to be false
      expect(hash[:marker_format]).to eq('TEST_{page_num}')
    end

    it 'handles boolean conversion' do
      config = Kreuzberg::Config::PageConfig.new(
        extract_pages: 1,
        insert_page_markers: 0
      )

      expect(config.extract_pages).to be true
      expect(config.insert_page_markers).to be false
    end

    it 'preserves marker format default' do
      config = Kreuzberg::Config::PageConfig.new(extract_pages: true)

      expect(config.marker_format).not_to be_nil
      expect(config.marker_format).to match(/<!-- PAGE/)
    end
  end

  describe 'Integration Tests' do
    it 'extraction config includes pages config' do
      extraction_config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: true)
      )

      expect(extraction_config.pages).not_to be_nil
      expect(extraction_config.pages).to be_a(Kreuzberg::Config::PageConfig)
      expect(extraction_config.pages.extract_pages).to be true
    end

    it 'extraction config to_h includes pages' do
      pages_config = Kreuzberg::Config::PageConfig.new(
        extract_pages: true,
        insert_page_markers: true,
        marker_format: 'CUSTOM_{page_num}'
      )
      extraction_config = Kreuzberg::Config::Extraction.new(pages: pages_config)

      hash = extraction_config.to_h

      expect(hash).to include(:pages)
      expect(hash[:pages]).to be_a(Hash)
      expect(hash[:pages][:extract_pages]).to be true
      expect(hash[:pages][:insert_page_markers]).to be true
      expect(hash[:pages][:marker_format]).to eq('CUSTOM_{page_num}')
    end

    it 'accepts pages config as hash in extraction config' do
      extraction_config = Kreuzberg::Config::Extraction.new(
        pages: {
          extract_pages: true,
          insert_page_markers: true,
          marker_format: 'HASH_{page_num}'
        }
      )

      expect(extraction_config.pages).to be_a(Kreuzberg::Config::PageConfig)
      expect(extraction_config.pages.extract_pages).to be true
      expect(extraction_config.pages.insert_page_markers).to be true
      expect(extraction_config.pages.marker_format).to eq('HASH_{page_num}')
    end
  end
end
