# frozen_string_literal: true

RSpec.describe Kreuzberg::Config::FontConfig do
  describe 'initialization' do
    it 'creates font config with defaults' do
      config = described_class.new

      expect(config.enabled).to be true
      expect(config.custom_font_dirs).to be_nil
    end

    it 'creates font config with custom values' do
      config = described_class.new(
        enabled: true,
        custom_font_dirs: ['/usr/share/fonts/custom']
      )

      expect(config.enabled).to be true
      expect(config.custom_font_dirs).to eq(['/usr/share/fonts/custom'])
    end

    it 'creates font config with enabled=false' do
      config = described_class.new(enabled: false)

      expect(config.enabled).to be false
      expect(config.custom_font_dirs).to be_nil
    end

    it 'creates font config with custom directories' do
      dirs = ['/usr/share/fonts/custom', '~/my-fonts']
      config = described_class.new(custom_font_dirs: dirs)

      expect(config.enabled).to be true
      expect(config.custom_font_dirs).to eq(dirs)
    end

    it 'creates font config with all parameters' do
      dirs = ['/path/to/fonts']
      config = described_class.new(enabled: true, custom_font_dirs: dirs)

      expect(config.enabled).to be true
      expect(config.custom_font_dirs).to eq(dirs)
    end
  end

  describe 'attribute access' do
    it 'allows setting enabled via attr_accessor' do
      config = described_class.new
      config.enabled = false

      expect(config.enabled).to be false
    end

    it 'allows setting custom_font_dirs via attr_accessor' do
      config = described_class.new
      dirs = ['/new/path']
      config.custom_font_dirs = dirs

      expect(config.custom_font_dirs).to eq(dirs)
    end

    it 'allows clearing custom_font_dirs' do
      config = described_class.new(custom_font_dirs: ['/path1', '/path2'])
      config.custom_font_dirs = nil

      expect(config.custom_font_dirs).to be_nil
    end
  end

  describe 'custom directories' do
    it 'handles empty custom directories array' do
      config = described_class.new(custom_font_dirs: [])

      expect(config.custom_font_dirs).to eq([])
      expect(config.custom_font_dirs.length).to eq(0)
    end

    it 'handles multiple custom directories' do
      dirs = ['/path1', '/path2', '/path3', '~/fonts', './relative-fonts']
      config = described_class.new(custom_font_dirs: dirs)

      expect(config.custom_font_dirs).to eq(dirs)
      expect(config.custom_font_dirs.length).to eq(5)
    end

    it 'preserves directory paths with tilde' do
      dirs = ['~/my-fonts', '~/Documents/fonts']
      config = described_class.new(custom_font_dirs: dirs)

      expect(config.custom_font_dirs).to include('~/my-fonts')
      expect(config.custom_font_dirs).to include('~/Documents/fonts')
    end

    it 'preserves relative paths' do
      dirs = ['./fonts', '../fonts']
      config = described_class.new(custom_font_dirs: dirs)

      expect(config.custom_font_dirs).to include('./fonts')
      expect(config.custom_font_dirs).to include('../fonts')
    end
  end

  describe 'conversion' do
    it 'converts to hash' do
      config = described_class.new(
        enabled: true,
        custom_font_dirs: ['/fonts']
      )
      hash = config.to_h

      expect(hash).to be_a(Hash)
      expect(hash[:enabled]).to be(true)
      expect(hash[:custom_font_dirs]).to eq(['/fonts'])
    end

    it 'converts to hash with nil custom_dirs' do
      config = described_class.new(enabled: false)
      hash = config.to_h

      expect(hash).to be_a(Hash)
      expect(hash[:enabled]).to be(false)
      expect(hash[:custom_font_dirs]).to be_nil
    end

    it 'converts to hash and includes all keys' do
      config = described_class.new(
        enabled: true,
        custom_font_dirs: ['/fonts']
      )
      hash = config.to_h

      expect(hash).to be_a(Hash)
      expect(hash).to include(enabled: true)
      expect(hash).to include(custom_font_dirs: ['/fonts'])
    end
  end

  describe 'integration with PdfConfig' do
    it 'integrates with PdfConfig' do
      font_config = described_class.new(
        enabled: true,
        custom_font_dirs: ['/fonts']
      )
      pdf_config = Kreuzberg::Config::PDF.new(font_config: font_config)

      expect(pdf_config.font_config).not_to be_nil
      expect(pdf_config.font_config.enabled).to be true
      expect(pdf_config.font_config.custom_font_dirs).to eq(['/fonts'])
    end

    it 'integrates with PdfConfig with all parameters' do
      font_config = described_class.new(
        enabled: true,
        custom_font_dirs: ['/custom-fonts']
      )
      pdf_config = Kreuzberg::Config::PDF.new(
        extract_images: true,
        passwords: ['pass1'],
        extract_metadata: true,
        font_config: font_config
      )

      expect(pdf_config.extract_images).to be true
      expect(pdf_config.passwords).to eq(['pass1'])
      expect(pdf_config.extract_metadata).to be true
      expect(pdf_config.font_config.enabled).to be true
    end

    it 'allows setting font_config via setter' do
      pdf_config = Kreuzberg::Config::PDF.new
      font_config = described_class.new(enabled: false)

      pdf_config.font_config = font_config

      expect(pdf_config.font_config).not_to be_nil
      expect(pdf_config.font_config.enabled).to be false
    end

    it 'allows clearing font_config via setter' do
      font_config = described_class.new(custom_font_dirs: ['/fonts'])
      pdf_config = Kreuzberg::Config::PDF.new(font_config: font_config)

      expect(pdf_config.font_config).not_to be_nil

      pdf_config.font_config = nil

      expect(pdf_config.font_config).to be_nil
    end
  end

  describe 'edge cases' do
    it 'handles disabled with custom directories' do
      config = described_class.new(
        enabled: false,
        custom_font_dirs: ['/fonts']
      )

      expect(config.enabled).to be false
      expect(config.custom_font_dirs).to eq(['/fonts'])
    end

    it 'handles multiple changes to enabled flag' do
      config = described_class.new
      config.enabled = false
      config.enabled = true
      config.enabled = false

      expect(config.enabled).to be false
    end

    it 'handles multiple changes to custom directories' do
      config = described_class.new
      config.custom_font_dirs = ['/path1']
      config.custom_font_dirs = ['/path1', '/path2']
      config.custom_font_dirs = []

      expect(config.custom_font_dirs).to eq([])
    end
  end
end
