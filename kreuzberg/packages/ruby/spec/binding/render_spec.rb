# Hand-written binding-specific edge case tests for PDF rendering.
# Happy-path render tests are auto-generated from fixtures in e2e/.
# These tests cover error handling, validation, and lifecycle patterns
# that vary per language and can't be generated uniformly.

# frozen_string_literal: true

require 'spec_helper'

RSpec.describe 'PDF Rendering' do
  it 'exposes rendering methods' do
    expect(Kreuzberg).to respond_to(:render_pdf_page)
    expect(Kreuzberg).to respond_to(:render_pdf_pages_iter)
  end

  describe '.render_pdf_page' do
    it 'raises an error for a nonexistent file' do
      expect do
        Kreuzberg.render_pdf_page('/nonexistent/path/to/document.pdf', 0)
      end.to raise_error(Kreuzberg::Errors::IOError)
    end

    it 'raises an error for an out-of-bounds page index' do
      pdf_path = test_document_path('pdf/tiny.pdf')
      skip 'Test PDF not available' unless File.exist?(pdf_path)

      expect do
        Kreuzberg.render_pdf_page(pdf_path, 9999)
      end.to raise_error(StandardError)
    end
  end

  describe '.render_pdf_page with negative index' do
    it 'raises ArgumentError for a negative page index' do
      pdf_path = test_document_path('pdf/tiny.pdf')
      skip 'Test PDF not available' unless File.exist?(pdf_path)

      expect do
        Kreuzberg.render_pdf_page(pdf_path, -1)
      end.to raise_error(ArgumentError)
    end
  end

  describe '.render_pdf_pages_iter' do
    it 'raises an error for a nonexistent file' do
      expect do
        Kreuzberg.render_pdf_pages_iter('/nonexistent/path/to/document.pdf') { |_, _| nil }
      end.to raise_error(Kreuzberg::Errors::IOError)
    end
  end

  describe '.render_pdf_page with empty path' do
    it 'raises an error for an empty path' do
      expect do
        Kreuzberg.render_pdf_page('', 0)
      end.to raise_error(StandardError)
    end
  end

  describe '.render_pdf_pages_iter cleanup' do
    it 'handles iterator cleanup without fully consuming' do
      pdf_path = test_document_path('pdf/tiny.pdf')
      skip 'Test PDF not available' unless File.exist?(pdf_path)

      # Iterate but stop immediately — no crash
      Kreuzberg.render_pdf_pages_iter(pdf_path) do |_page_index, _png_data|
        break
      end
    end
  end

  describe '.render_pdf_pages_iter early termination' do
    it 'returns valid PNG for the first page then stops' do
      pdf_path = test_document_path('pdf/tiny.pdf')
      skip 'Test PDF not available' unless File.exist?(pdf_path)

      first_png = nil
      Kreuzberg.render_pdf_pages_iter(pdf_path) do |page_index, png_data|
        expect(page_index).to eq(0)
        expect(png_data).to be_a(String)
        expect(png_data.bytesize).to be > 8
        # PNG magic bytes
        expect(png_data.bytes[0..3]).to eq([0x89, 0x50, 0x4E, 0x47])
        first_png = png_data
        break
      end

      expect(first_png).not_to be_nil
    end
  end
end
