# frozen_string_literal: true

RSpec.describe 'Pages Extraction' do
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
