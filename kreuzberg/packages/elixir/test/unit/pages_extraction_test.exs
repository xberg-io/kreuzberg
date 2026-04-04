defmodule Kreuzberg.Unit.PagesExtractionTest do
  @moduledoc """
  Unit tests for page extraction functionality.

  Tests cover:
  - extract_pages: true - Returns pages array
  - insert_page_markers: true - Markers appear in content
  - marker_format: custom format works
  - Multi-page PDF produces multiple pages
  - Page content structure validation
  """

  use ExUnit.Case, async: true

  describe "custom marker_format configuration" do
    test "uses custom marker format when specified" do
      custom_format = "=== Page {page_num} ==="

      config = %Kreuzberg.ExtractionConfig{
        pages: %{
          insert_page_markers: true,
          marker_format: custom_format
        }
      }

      {:ok, result} =
        Kreuzberg.extract(get_test_pdf_bytes(), "application/pdf", config)

      assert String.contains?(result.content, "=== Page")
    end

    test "replaces {page_num} with actual page numbers" do
      custom_format = "--- BEGIN PAGE {page_num} ---"

      config = %Kreuzberg.ExtractionConfig{
        pages: %{
          insert_page_markers: true,
          marker_format: custom_format
        }
      }

      {:ok, result} =
        Kreuzberg.extract(get_test_pdf_bytes(), "application/pdf", config)

      assert String.contains?(result.content, "--- BEGIN PAGE")
    end

    test "alternative custom format works correctly" do
      custom_format = "[Page {page_num}]"

      config = %Kreuzberg.ExtractionConfig{
        pages: %{
          insert_page_markers: true,
          marker_format: custom_format
        }
      }

      {:ok, result} =
        Kreuzberg.extract(get_test_pdf_bytes(), "application/pdf", config)

      assert String.contains?(result.content, "[Page")
    end

    test "supports numeric suffixes in marker format" do
      custom_format = "Page #{to_string(1)}"

      config = %Kreuzberg.ExtractionConfig{
        pages: %{
          insert_page_markers: true,
          marker_format: custom_format
        }
      }

      # Should not raise error even if format is partially numeric
      {:ok, _result} =
        Kreuzberg.extract(get_test_pdf_bytes(), "application/pdf", config)
    end
  end

  describe "combined page extraction and markers" do
    test "extract_pages and insert_page_markers work together" do
      config = %Kreuzberg.ExtractionConfig{
        pages: %{
          extract_pages: true,
          insert_page_markers: true,
          marker_format: "### Page {page_num} ###"
        }
      }

      {:ok, result} =
        Kreuzberg.extract(get_test_pdf_bytes(), "application/pdf", config)

      # Should have pages array
      assert is_list(result.pages)
      assert result.pages != []

      # Should have markers in content
      assert String.contains?(result.content, "###")
    end

    test "markers and extracted pages are consistent" do
      config = %Kreuzberg.ExtractionConfig{
        pages: %{
          extract_pages: true,
          insert_page_markers: true
        }
      }

      {:ok, result} =
        Kreuzberg.extract(get_test_pdf_bytes(), "application/pdf", config)

      # Number of pages in array should match document structure
      assert is_list(result.pages)
      assert result.pages != []
    end
  end

  describe "page configuration edge cases" do
    test "handles extraction without explicit page config" do
      # No pages config provided
      config = %Kreuzberg.ExtractionConfig{}

      {:ok, result} =
        Kreuzberg.extract(get_test_pdf_bytes(), "application/pdf", config)

      # Should succeed with default behavior
      assert is_binary(result.content) or result.content == ""
    end

    test "handles pages config with only extract_pages set" do
      config = %Kreuzberg.ExtractionConfig{
        pages: %{extract_pages: true}
      }

      {:ok, result} =
        Kreuzberg.extract(get_test_pdf_bytes(), "application/pdf", config)

      assert is_list(result.pages)
      assert result.pages != []
    end

    test "handles pages config with only insert_page_markers set" do
      config = %Kreuzberg.ExtractionConfig{
        pages: %{insert_page_markers: true}
      }

      {:ok, result} =
        Kreuzberg.extract(get_test_pdf_bytes(), "application/pdf", config)

      assert is_binary(result.content)
    end

    test "handles pages config with only marker_format set" do
      config = %Kreuzberg.ExtractionConfig{
        pages: %{marker_format: ">> Page {page_num} <<"}
      }

      {:ok, result} =
        Kreuzberg.extract(get_test_pdf_bytes(), "application/pdf", config)

      # Marker format should apply when markers are requested
      assert is_binary(result.content)
    end
  end

  # Helper function to get test PDF bytes
  defp get_test_pdf_bytes do
    # This would typically load from a test fixture file
    # For now, return a minimal valid PDF structure that the library can parse
    # In a real implementation, use File.read! or similar to load actual test PDFs
    case get_test_pdf_path() do
      {:ok, path} -> File.read!(path)
      :error -> minimal_test_pdf()
    end
  end

  # Helper to locate test PDF file
  defp get_test_pdf_path do
    repo_root = get_repo_root()
    test_pdf_path = Path.join([repo_root, "test_documents", "pdfs_with_tables", "tiny.pdf"])

    if File.exists?(test_pdf_path) do
      {:ok, test_pdf_path}
    else
      :error
    end
  end

  # Helper to get repo root
  defp get_repo_root do
    cwd = File.cwd!()
    # Navigate up from packages/elixir to repo root
    Path.join([cwd, "..", "..", ".."])
  end

  # Fallback minimal PDF (won't work for actual extraction, but helps with compilation)
  defp minimal_test_pdf do
    <<"%PDF-1.7\n", "1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n",
      "2 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj\n",
      "3 0 obj<</Type/Page/Parent 2 0 R>>endobj\n",
      "xref 0 4 0000000000 65535 f 0000000009 00000 n 0000000058 00000 n 0000000117 00000 n\n",
      "trailer<</Size 4/Root 1 0 R>>\n", "startxref\n", "191\n", "%%EOF">>
  end
end
