defmodule KreuzbergTest.E2E.HTMLExtractionTest do
  @moduledoc """
  End-to-end tests for HTML extraction and markdown conversion workflows.

  Tests complete HTML processing pipelines including:
  - HTML to markdown conversion
  - Metadata extraction from HTML documents
  - Nested element handling
  - Link and image preservation
  - Configuration-based extraction variants
  """

  use ExUnit.Case, async: true

  @tag :e2e
  test "extracts text from valid HTML" do
    html_binary = create_sample_html()

    {:ok, result} = Kreuzberg.extract(html_binary, "text/html")

    assert %Kreuzberg.ExtractionResult{} = result
    assert is_binary(result.content)
    assert result.mime_type == "text/html"
  end

  @tag :e2e
  test "converts HTML to markdown structure" do
    html_binary = create_sample_html()

    {:ok, result} = Kreuzberg.extract(html_binary, "text/html")

    content = result.content
    assert is_binary(content)
    # Verify markdown-like structure (basic check)
    assert content != ""
  end

  @tag :e2e
  test "preserves HTML metadata extraction" do
    html_binary = create_sample_html_with_metadata()

    {:ok, result} = Kreuzberg.extract(html_binary, "text/html")

    assert %Kreuzberg.Metadata{} = result.metadata
    # Metadata structure should be valid with atom keys and proper values
    # Convert struct to map for enumeration
    metadata_map = Map.from_struct(result.metadata)

    Enum.each(metadata_map, fn {key, value} ->
      assert is_atom(key), "Metadata key should be atom"

      assert is_binary(value) or is_atom(value) or is_number(value) or is_nil(value) or
               is_list(value) or is_map(value) or is_struct(value),
             "Metadata value should be serializable or nil"
    end)
  end

  @tag :e2e
  test "extracts from complex nested HTML" do
    html_binary = create_complex_nested_html()

    {:ok, result} = Kreuzberg.extract(html_binary, "text/html")

    assert %Kreuzberg.ExtractionResult{} = result
    assert is_binary(result.content)
    assert byte_size(result.content) > 0
  end

  @tag :e2e
  test "handles malformed HTML gracefully" do
    malformed_html = "<html><body><p>Unclosed paragraph<div>nested</body></html>"

    {:ok, result} = Kreuzberg.extract(malformed_html, "text/html")

    assert %Kreuzberg.ExtractionResult{} = result
    assert is_binary(result.content)
  end

  @tag :e2e
  test "extracts with chunking configuration" do
    html_binary = create_large_html_document()

    config = %Kreuzberg.ExtractionConfig{
      chunking: %{"enabled" => true, "size" => 256}
    }

    {:ok, result} = Kreuzberg.extract(html_binary, "text/html", config)

    assert is_list(result.chunks)
  end

  @tag :e2e
  test "preserves links in HTML conversion" do
    html_with_links = """
    <html>
    <body>
    <a href="https://example.com">Example Link</a>
    <p>Some text with <a href="https://another.com">another link</a></p>
    </body>
    </html>
    """

    {:ok, result} = Kreuzberg.extract(html_with_links, "text/html")

    assert is_binary(result.content)
  end

  @tag :e2e
  test "detects language in HTML content" do
    html_binary = create_sample_html()

    config = %Kreuzberg.ExtractionConfig{
      language_detection: %{"enabled" => true}
    }

    {:ok, result} = Kreuzberg.extract(html_binary, "text/html", config)

    assert is_list(result.detected_languages)
  end

  @tag :e2e
  test "extracts table content from HTML" do
    html_with_table = """
    <html>
    <body>
    <table>
      <tr><th>Header 1</th><th>Header 2</th></tr>
      <tr><td>Cell 1</td><td>Cell 2</td></tr>
    </table>
    </body>
    </html>
    """

    {:ok, result} = Kreuzberg.extract(html_with_table, "text/html")

    assert %Kreuzberg.ExtractionResult{} = result
    assert is_list(result.tables)
  end

  @tag :e2e
  test "handles empty HTML document" do
    empty_html = "<html><body></body></html>"

    result = Kreuzberg.extract(empty_html, "text/html")

    # Empty HTML should either succeed with empty content or fail gracefully
    case result do
      {:ok, extraction_result} ->
        assert %Kreuzberg.ExtractionResult{} = extraction_result
        assert is_binary(extraction_result.content)

      {:error, reason} ->
        assert is_binary(reason)
        assert byte_size(reason) > 0
    end
  end

  # Private helpers

  defp create_sample_html do
    """
    <!DOCTYPE html>
    <html>
    <head>
      <title>Sample HTML Document</title>
    </head>
    <body>
      <h1>Main Heading</h1>
      <p>This is a paragraph with some content.</p>
      <p>Another paragraph with more information.</p>
    </body>
    </html>
    """
  end

  defp create_sample_html_with_metadata do
    """
    <!DOCTYPE html>
    <html>
    <head>
      <title>Document with Metadata</title>
      <meta name="author" content="Test Author">
      <meta name="description" content="A test document">
    </head>
    <body>
      <h1>Content</h1>
      <p>This document has metadata.</p>
    </body>
    </html>
    """
  end

  defp create_complex_nested_html do
    """
    <!DOCTYPE html>
    <html>
    <body>
      <div class="container">
        <section>
          <h2>Section 1</h2>
          <article>
            <p>Article content</p>
            <ul>
              <li>Item 1</li>
              <li>Item 2</li>
              <li>Item 3</li>
            </ul>
          </article>
        </section>
        <section>
          <h2>Section 2</h2>
          <p>More content here.</p>
        </section>
      </div>
    </body>
    </html>
    """
  end

  defp create_large_html_document do
    """
    <!DOCTYPE html>
    <html>
    <body>
      <h1>Large Document</h1>
      #{String.duplicate("<p>This is a paragraph. " <> String.duplicate("Word ", 50) <> "</p>\n", 20)}
    </body>
    </html>
    """
  end
end
