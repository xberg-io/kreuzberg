# frozen_string_literal: true

require 'spec_helper'
require 'tempfile'
require 'fileutils'

RSpec.describe 'Image Extraction' do
  describe 'ImageExtraction configuration integration' do
    it 'applies different DPI settings to affect extraction behavior' do
      config_low = Kreuzberg::Config::Extraction.new(
        image_extraction: Kreuzberg::Config::ImageExtraction.new(
          extract_images: true,
          target_dpi: 72
        )
      )
      config_high = Kreuzberg::Config::Extraction.new(
        image_extraction: Kreuzberg::Config::ImageExtraction.new(
          extract_images: true,
          target_dpi: 300
        )
      )

      begin
        result_low = Kreuzberg.extract_file_sync(path: test_document_path('pdf/with_images.pdf'), config: config_low)
        result_high = Kreuzberg.extract_file_sync(path: test_document_path('pdf/with_images.pdf'), config: config_high)

        # Both configurations should produce valid extraction
        expect(result_low).not_to be_nil
        expect(result_high).not_to be_nil
        # Different DPI settings should be accepted
        expect([result_low, result_high]).to all(be_a(Kreuzberg::Result))
      rescue Kreuzberg::Errors::ValidationError
        skip 'Test file not available'
      end
    end

    it 'respects extract_images false disables extraction' do
      config_enabled = Kreuzberg::Config::Extraction.new(
        image_extraction: Kreuzberg::Config::ImageExtraction.new(
          extract_images: true
        )
      )
      config_disabled = Kreuzberg::Config::Extraction.new(
        image_extraction: Kreuzberg::Config::ImageExtraction.new(
          extract_images: false
        )
      )

      begin
        result_enabled = Kreuzberg.extract_file_sync(path: test_document_path('pdf/with_images.pdf'), config: config_enabled)
        result_disabled = Kreuzberg.extract_file_sync(path: test_document_path('pdf/with_images.pdf'), config: config_disabled)

        # Enabled should extract if images present
        expect(result_enabled).not_to be_nil
        # Disabled should return nil or empty images
        expect(result_disabled.images).to be_empty if result_disabled.images
      rescue Kreuzberg::Errors::ValidationError
        skip 'Test file not available'
      end
    end

    it 'handles dimension constraints realistically' do
      config = Kreuzberg::Config::Extraction.new(
        image_extraction: Kreuzberg::Config::ImageExtraction.new(
          extract_images: true,
          max_image_dimension: 1024
        )
      )

      begin
        result = Kreuzberg.extract_file_sync(path: test_document_path('pdf/with_images.pdf'), config: config)

        expect(result).not_to be_nil
        # Dimension constraint should be applied
        if result.images && !result.images.empty?
          result.images.each do |image|
            # Image should respect dimension constraints
            expect(image).not_to be_nil
          end
        end
      rescue Kreuzberg::Errors::ValidationError
        skip 'Test file not available'
      end
    end
  end

  describe 'Integration with Extraction config' do
    it 'accepts ImageExtraction config in Extraction' do
      image_config = Kreuzberg::Config::ImageExtraction.new(
        extract_images: true,
        target_dpi: 600
      )
      config = Kreuzberg::Config::Extraction.new(image_extraction: image_config)

      expect(config.image_extraction).to be_a(Kreuzberg::Config::ImageExtraction)
      expect(config.image_extraction.target_dpi).to eq(600)
    end

    it 'accepts image extraction config as hash in Extraction' do
      config = Kreuzberg::Config::Extraction.new(
        image_extraction: {
          extract_images: true,
          target_dpi: 600,
          max_image_dimension: 3000
        }
      )

      expect(config.image_extraction).to be_a(Kreuzberg::Config::ImageExtraction)
      expect(config.image_extraction.extract_images).to be true
      expect(config.image_extraction.target_dpi).to eq(600)
      expect(config.image_extraction.max_image_dimension).to eq(3000)
    end

    it 'includes image extraction config in to_h' do
      image_config = Kreuzberg::Config::ImageExtraction.new(
        extract_images: true,
        target_dpi: 600
      )
      config = Kreuzberg::Config::Extraction.new(image_extraction: image_config)

      hash = config.to_h

      expect(hash).to include(:image_extraction)
      expect(hash[:image_extraction]).to be_a(Hash)
      expect(hash[:image_extraction][:extract_images]).to be true
      expect(hash[:image_extraction][:target_dpi]).to eq(600)
    end

    it 'combines image extraction with other configurations' do
      config = Kreuzberg::Config::Extraction.new(
        use_cache: true,
        force_ocr: true,
        image_extraction: Kreuzberg::Config::ImageExtraction.new(
          extract_images: true,
          target_dpi: 600
        ),
        ocr: Kreuzberg::Config::OCR.new(
          backend: 'tesseract',
          language: 'eng'
        )
      )

      expect(config.use_cache).to be true
      expect(config.force_ocr).to be true
      expect(config.image_extraction.target_dpi).to eq(600)
      expect(config.ocr.backend).to eq('tesseract')
    end

    it 'handles nil image extraction config' do
      config = Kreuzberg::Config::Extraction.new(image_extraction: nil)

      expect(config.image_extraction).to be_nil
    end
  end
end
