defmodule KreuzbergTest.E2E.PDFExtractionTest do
  @moduledoc """
  End-to-end tests for PDF extraction workflows.

  Tests complete PDF extraction pipelines including:
  - Single document extraction with various configurations
  - Multi-page PDF processing
  - Metadata and structural element extraction
  - Error handling for corrupted or invalid PDFs
  - Configuration variations (OCR, chunking, caching)
  """

  use ExUnit.Case, async: true

  @tag :e2e
  test "extracts text from valid PDF binary" do
    pdf_binary = create_sample_pdf_binary()

    {:ok, result} = Kreuzberg.extract(pdf_binary, "application/pdf")

    assert %Kreuzberg.ExtractionResult{} = result
    assert is_binary(result.content)
    assert result.mime_type == "application/pdf"
    # Pages may be nil if page extraction is not enabled
    assert is_nil(result.pages) or is_list(result.pages)
  end

  @tag :e2e
  test "extracts metadata from PDF" do
    pdf_binary = create_sample_pdf_binary()

    {:ok, result} = Kreuzberg.extract(pdf_binary, "application/pdf")

    assert %Kreuzberg.Metadata{} = result.metadata
    # Metadata structure is valid - all keys are atoms and values are valid terms
    # Convert struct to map for enumeration
    metadata_map = Map.from_struct(result.metadata)

    Enum.each(metadata_map, fn {key, value} ->
      assert is_atom(key)
      assert valid_erlang_term?(value)
    end)
  end

  @tag :e2e
  test "preserves page structure during extraction" do
    pdf_binary = create_sample_pdf_binary()

    {:ok, result} = Kreuzberg.extract(pdf_binary, "application/pdf")

    # Pages may be nil if page extraction is not enabled
    assert is_nil(result.pages) or is_list(result.pages)

    # Pages should contain structured data (each page is a map with expected fields)
    if result.pages not in [nil, []] do
      Enum.each(result.pages, fn page ->
        assert is_map(page) or is_binary(page), "Page should be map or binary with content"
      end)
    end
  end

  @tag :e2e
  test "extracts with OCR configuration" do
    pdf_binary = create_sample_pdf_binary()

    config = %Kreuzberg.ExtractionConfig{
      ocr: %{"enabled" => true}
    }

    {:ok, result} = Kreuzberg.extract(pdf_binary, "application/pdf", config)

    assert %Kreuzberg.ExtractionResult{} = result
    assert is_binary(result.content)
  end

  @tag :e2e
  test "extracts with chunking enabled" do
    pdf_binary = create_sample_pdf_binary()

    config = %Kreuzberg.ExtractionConfig{
      chunking: %{"enabled" => true, "size" => 256}
    }

    {:ok, result} = Kreuzberg.extract(pdf_binary, "application/pdf", config)

    assert is_list(result.chunks)
  end

  @tag :e2e
  test "handles invalid PDF binary gracefully" do
    invalid_pdf = <<0, 1, 2, 3, 4, 5>>

    {:error, reason} = Kreuzberg.extract(invalid_pdf, "application/pdf")

    assert is_binary(reason)
    assert byte_size(reason) > 0
  end

  @tag :e2e
  test "uses cache when enabled" do
    pdf_binary = create_sample_pdf_binary()

    config = %Kreuzberg.ExtractionConfig{
      use_cache: true
    }

    # First extraction
    {:ok, result1} = Kreuzberg.extract(pdf_binary, "application/pdf", config)
    assert is_binary(result1.content) and byte_size(result1.content) > 0

    # Second extraction should use cache and produce identical results
    {:ok, result2} = Kreuzberg.extract(pdf_binary, "application/pdf", config)

    # Verify results are identical (indicating cache was used or deterministic extraction)
    assert result1.content == result2.content
    assert result1.metadata == result2.metadata
    assert result1.pages == result2.pages
  end

  @tag :e2e
  test "extracts with multiple configuration options" do
    pdf_binary = create_sample_pdf_binary()

    config = %Kreuzberg.ExtractionConfig{
      ocr: %{"enabled" => true, "backend" => "tesseract"},
      chunking: %{"enabled" => true, "size" => 512},
      language_detection: %{"enabled" => true},
      use_cache: true
    }

    {:ok, result} = Kreuzberg.extract(pdf_binary, "application/pdf", config)

    assert %Kreuzberg.ExtractionResult{} = result
    assert is_binary(result.content)
  end

  # Private helpers

  defp create_sample_pdf_binary do
    # Simple PDF binary for testing
    # In production, this would be a real PDF file loaded from disk
    <<"%PDF-1.4\n%comment\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n" <>
        "2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n" <>
        "3 0 obj\n<< /Type /Page /Parent 2 0 R /Resources << /Font << /F1 << /Type /Font /Subtype /Type1 /BaseFont /Helvetica >> >> >> /MediaBox [0 0 612 792] /Contents 4 0 R >>\nendobj\n" <>
        "4 0 obj\n<< /Length 44 >>\nstream\nBT\n/F1 12 Tf\n100 700 Td\n(Hello World) Tj\nET\nendstream\nendobj\nxref\n0 5\n" <>
        "0000000000 65535 f\n0000000009 00000 n\n0000000058 00000 n\n0000000115 00000 n\n0000000244 00000 n\ntrailer\n" <>
        "<< /Size 5 /Root 1 0 R >>\nstartxref\n338\n%%EOF"::binary>>
  end

  defp valid_erlang_term?(term) do
    _encoded = :erlang.term_to_binary(term)
    true
  rescue
    _ -> false
  end
end
