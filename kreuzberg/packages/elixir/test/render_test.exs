# Hand-written binding-specific edge case tests for PDF rendering.
# Happy-path render tests are auto-generated from fixtures in e2e/.
# These tests cover error handling, validation, and lifecycle patterns
# that vary per language and can't be generated uniformly.

defmodule KreuzbergTest.RenderTest do
  use ExUnit.Case, async: true

  defp get_test_pdf_path do
    repo_root = get_repo_root()

    possible_paths = [
      Path.join([repo_root, "test_documents", "pdf", "tiny.pdf"]),
      Path.join([repo_root, "test_documents", "tiny.pdf"])
    ]

    Enum.find_value(possible_paths, :error, fn path ->
      if File.exists?(path), do: {:ok, path}
    end)
  end

  defp get_repo_root do
    cwd = File.cwd!()
    Path.join([cwd, "..", ".."])
  end

  defp skip_unless_test_pdf! do
    case get_test_pdf_path() do
      {:ok, path} -> path
      :error -> flunk("Test PDF not found — cannot run this test")
    end
  end

  test "rendering functions are exported" do
    assert function_exported?(Kreuzberg, :render_pdf_page, 3)
    assert function_exported?(Kreuzberg, :render_pdf_pages_stream, 2)
  end

  describe "render_pdf_page/3" do
    test "returns error for nonexistent file" do
      result = Kreuzberg.render_pdf_page("/nonexistent/path/to/document.pdf", 0)
      assert {:error, _reason} = result
    end

    test "returns error for out-of-bounds page index" do
      path = skip_unless_test_pdf!()
      result = Kreuzberg.render_pdf_page(path, 9999)
      assert {:error, _reason} = result
    end
  end

  describe "render_pdf_page/3 with negative index" do
    test "raises FunctionClauseError for a negative page index" do
      path = skip_unless_test_pdf!()

      assert_raise FunctionClauseError, fn ->
        Kreuzberg.render_pdf_page(path, -1)
      end
    end
  end

  describe "render_pdf_pages_stream/2" do
    test "stream is lazy and can be halted early" do
      path = skip_unless_test_pdf!()

      first_page =
        Kreuzberg.render_pdf_pages_stream(path)
        |> Enum.take(1)

      assert length(first_page) == 1
      {page_index, png_bytes} = hd(first_page)
      assert is_integer(page_index)
      assert <<0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, _rest::binary>> = png_bytes
    end

    test "returns empty stream for nonexistent file" do
      stream = Kreuzberg.render_pdf_pages_stream("/nonexistent/path/to/document.pdf")
      assert Enum.to_list(stream) == []
    end
  end
end
