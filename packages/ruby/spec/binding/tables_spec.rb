# frozen_string_literal: true

require 'spec_helper'
require 'tempfile'
require 'fileutils'

RSpec.describe 'Table Extraction Quality' do
  describe 'table serialization and conversion' do
    let(:pdf_path) { test_document_path('pdf/table_document.pdf') }

    it 'serializes table to hash correctly' do
      config = Kreuzberg::Config::Extraction.new

      begin
        result = Kreuzberg.extract_file(path: pdf_path, config: config)
      rescue Kreuzberg::Errors::ValidationError
        skip 'Test PDF file not available'
      end

      if result.tables && !result.tables.empty?
        table = result.tables.first
        table_hash = table.to_h

        expect(table_hash).to be_a(Hash)
        expect(table_hash).to have_key(:cells)
        expect(table_hash).to have_key(:markdown)
        expect(table_hash).to have_key(:page_number)
      end
    end

    it 'preserves table data through serialization' do
      config = Kreuzberg::Config::Extraction.new

      begin
        result = Kreuzberg.extract_file(path: pdf_path, config: config)
      rescue Kreuzberg::Errors::ValidationError
        skip 'Test PDF file not available'
      end

      if result.tables && !result.tables.empty?
        table = result.tables.first
        table_hash = table.to_h

        expect(table_hash[:cells]).to eq(table.cells)
        expect(table_hash[:markdown]).to eq(table.markdown)
        expect(table_hash[:page_number]).to eq(table.page_number)
      end
    end

    it 'converts result with tables to JSON' do
      config = Kreuzberg::Config::Extraction.new

      begin
        result = Kreuzberg.extract_file(path: pdf_path, config: config)
      rescue Kreuzberg::Errors::ValidationError
        skip 'Test PDF file not available'
      end

      expect(result).not_to be_nil
      json_str = result.to_json
      expect(json_str).to be_a(String)
      expect(json_str.length).to be > 0
    end
  end

  describe 'table extraction with page context' do
    let(:pdf_path) { test_document_path('pdf/table_document.pdf') }

    it 'associates tables with correct page numbers' do
      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: true)
      )

      begin
        result = Kreuzberg.extract_file(path: pdf_path, config: config)
      rescue Kreuzberg::Errors::ValidationError
        skip 'Test PDF file not available'
      end

      if result.tables && !result.tables.empty?
        result.tables.each do |table|
          expect(table.page_number).to be > 0
          expect(table.page_number).to be <= result.page_count
        end
      end
    end

    it 'extracts tables from specific pages when available' do
      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: true)
      )

      begin
        result = Kreuzberg.extract_file(path: pdf_path, config: config)
      rescue Kreuzberg::Errors::ValidationError
        skip 'Test PDF file not available'
      end

      if result.pages && !result.pages.empty?
        result.pages.each do |page|
          expect(page.page_number).not_to be_nil
          next unless page.tables

          page.tables.each do |table|
            expect(table.page_number).to eq(page.page_number)
          end
        end
      end
    end

    it 'maintains table consistency across page and global results' do
      config = Kreuzberg::Config::Extraction.new(
        pages: Kreuzberg::Config::PageConfig.new(extract_pages: true)
      )

      begin
        result = Kreuzberg.extract_file(path: pdf_path, config: config)
      rescue Kreuzberg::Errors::ValidationError
        skip 'Test PDF file not available'
      end

      if result.tables && !result.tables.empty? && result.pages && !result.pages.empty?
        global_table_count = result.tables.length
        page_table_count = result.pages.sum { |page| page.tables&.length || 0 }

        expect(page_table_count).to eq(global_table_count)
      end
    end
  end

  describe 'Table Struct validation' do
    it 'creates Table struct with all fields' do
      table = Kreuzberg::Result::Table.new(
        cells: [%w[Header1 Header2], %w[Value1 Value2]],
        markdown: '| Header1 | Header2 |\n|---------|--------|\n| Value1 | Value2 |',
        page_number: 1
      )

      expect(table.cells).to eq([%w[Header1 Header2], %w[Value1 Value2]])
      expect(table.markdown).to include('Header1')
      expect(table.page_number).to eq(1)
    end

    it 'converts Table struct to hash' do
      table = Kreuzberg::Result::Table.new(
        cells: [%w[A B], %w[C D]],
        markdown: '| A | B |\n|---|---|\n| C | D |',
        page_number: 2
      )

      table_hash = table.to_h

      expect(table_hash).to be_a(Hash)
      expect(table_hash[:cells]).to eq([%w[A B], %w[C D]])
      expect(table_hash[:markdown]).to include('A')
      expect(table_hash[:page_number]).to eq(2)
    end

    it 'handles Table struct with empty cells' do
      table = Kreuzberg::Result::Table.new(
        cells: [],
        markdown: '',
        page_number: 1
      )

      expect(table.cells).to eq([])
      expect(table.markdown).to eq('')
      expect(table.page_number).to eq(1)
    end

    it 'handles Table struct with nil values' do
      table = Kreuzberg::Result::Table.new(
        cells: nil,
        markdown: nil,
        page_number: 0
      )

      expect(table.cells).to be_nil
      expect(table.markdown).to be_nil
      expect(table.page_number).to eq(0)
    end
  end
end
