defmodule KreuzbergTest.Unit.FileExtractionTest do
  @moduledoc """
  Unit tests for Kreuzberg file extraction functions.

  Tests cover:
  - extract_file/3: File extraction with explicit and auto-detected MIME types
  - extract_file!/3: Bang variant with success and exception cases
  - Configuration handling with struct and map inputs
  - Error handling for missing files and invalid paths
  - Path type handling (String vs Path.t())
  - Result structure validation
  """

  use ExUnit.Case

  # Helper function to create a temporary test file
  defp create_temp_file(content) do
    unique_id = System.unique_integer()
    path = System.tmp_dir!() <> "/kreuzberg_test_#{unique_id}.txt"
    File.write!(path, content)
    path
  end

  # Helper function to cleanup temporary files
  defp cleanup_temp_file(path) when is_binary(path) do
    if File.exists?(path) do
      File.rm(path)
    end
  end

  describe "extract_file/3 with explicit MIME type" do
    @tag :unit
    test "returns success tuple for text file with explicit MIME type" do
      path = create_temp_file("Hello world")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert %Kreuzberg.ExtractionResult{} = result
        assert result.content == "Hello world"
        assert result.mime_type == "text/plain"
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "accepts String path" do
      path = create_temp_file("Test content")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert %Kreuzberg.ExtractionResult{} = result
        assert result.content == "Test content"
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "accepts Path.t() from Path module" do
      path_string = create_temp_file("Path test")
      path = Path.expand(path_string)

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert %Kreuzberg.ExtractionResult{} = result
        assert result.content == "Path test"
      after
        cleanup_temp_file(path_string)
      end
    end

    @tag :unit
    test "result structure is valid with explicit MIME type" do
      path = create_temp_file("structure test")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert %Kreuzberg.ExtractionResult{
                 content: content,
                 mime_type: mime_type,
                 metadata: metadata,
                 tables: tables
               } = result

        assert is_binary(content)
        assert is_binary(mime_type)
        assert is_map(metadata)
        assert is_list(tables)
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "handles empty file with explicit MIME type" do
      path = create_temp_file("")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert result.content == ""
        assert result.mime_type == "text/plain"
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "handles multiline text files" do
      content = "Line 1\nLine 2\nLine 3"
      path = create_temp_file(content)

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert result.content == content
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "handles special characters in file content" do
      content = "Special chars: @#$%^&*()\nUnicode: 你好世界"
      path = create_temp_file(content)

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert result.content == content
      after
        cleanup_temp_file(path)
      end
    end
  end

  describe "extract_file/3 with nil MIME type (auto-detection)" do
    @tag :unit
    test "returns success tuple with nil MIME type for auto-detection" do
      path = create_temp_file("auto detect")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, nil)

        assert %Kreuzberg.ExtractionResult{} = result
        assert result.content == "auto detect"
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "omitting MIME type defaults to nil (auto-detection)" do
      path = create_temp_file("default mime")

      try do
        {:ok, result} = Kreuzberg.extract_file(path)

        assert %Kreuzberg.ExtractionResult{} = result
        assert result.content == "default mime"
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "auto-detection detects MIME type from file" do
      path = create_temp_file("detected content")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, nil)

        # MIME type should be detected, not nil
        assert is_binary(result.mime_type)
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "result is valid with auto-detected MIME type" do
      path = create_temp_file("auto detect result")

      try do
        {:ok, result} = Kreuzberg.extract_file(path)

        assert %Kreuzberg.ExtractionResult{
                 content: content,
                 mime_type: mime_type,
                 metadata: metadata,
                 tables: tables
               } = result

        assert is_binary(content)
        assert is_binary(mime_type)
        assert is_map(metadata)
        assert is_list(tables)
      after
        cleanup_temp_file(path)
      end
    end
  end

  describe "extract_file/3 with configuration options" do
    @tag :unit
    test "accepts ExtractionConfig struct" do
      path = create_temp_file("config test")

      config = %Kreuzberg.ExtractionConfig{
        ocr: %{"enabled" => true}
      }

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain", config)

        assert %Kreuzberg.ExtractionResult{} = result
        assert result.content == "config test"
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "accepts map config with string keys" do
      path = create_temp_file("map config")

      try do
        {:ok, result} =
          Kreuzberg.extract_file(path, "text/plain", %{
            "ocr" => %{"enabled" => true}
          })

        assert %Kreuzberg.ExtractionResult{} = result
        assert result.content == "map config"
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "accepts map config with atom keys" do
      path = create_temp_file("atom config")

      try do
        {:ok, result} =
          Kreuzberg.extract_file(path, "text/plain", %{
            ocr: %{"enabled" => true},
            chunking: %{"size" => 512}
          })

        assert %Kreuzberg.ExtractionResult{} = result
        assert result.content == "atom config"
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "config struct with multiple options" do
      path = create_temp_file("multi config")

      config = %Kreuzberg.ExtractionConfig{
        ocr: %{"enabled" => true},
        chunking: %{"size" => 512},
        language_detection: %{"enabled" => true}
      }

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain", config)

        assert result.content == "multi config"
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "empty map config works" do
      path = create_temp_file("empty config")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain", %{})

        assert result.content == "empty config"
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "nil config is treated as default" do
      path = create_temp_file("nil config")

      try do
        {:ok, result1} = Kreuzberg.extract_file(path, "text/plain", nil)
        {:ok, result2} = Kreuzberg.extract_file(path, "text/plain")

        # Both should return the same content
        assert result1.content == result2.content
        assert result1.mime_type == result2.mime_type
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "config with nested options" do
      path = create_temp_file("nested config")

      try do
        {:ok, result} =
          Kreuzberg.extract_file(path, "text/plain", %{
            "pdf_options" => %{
              "extract_text" => true,
              "preserve_formatting" => true
            }
          })

        assert result.content == "nested config"
      after
        cleanup_temp_file(path)
      end
    end
  end

  describe "extract_file!/3 success cases" do
    @tag :unit
    test "returns result directly on success with explicit MIME type" do
      path = create_temp_file("bang success")

      try do
        result = Kreuzberg.extract_file!(path, "text/plain")

        assert %Kreuzberg.ExtractionResult{} = result
        assert result.content == "bang success"
        assert result.mime_type == "text/plain"
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "returns result directly on success with auto-detection" do
      path = create_temp_file("auto bang")

      try do
        result = Kreuzberg.extract_file!(path)

        assert %Kreuzberg.ExtractionResult{} = result
        assert result.content == "auto bang"
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "bang variant returns struct, not tuple" do
      path = create_temp_file("struct not tuple")

      try do
        result = Kreuzberg.extract_file!(path, "text/plain")

        # Should be a struct, not a tuple
        assert is_struct(result)
        refute is_tuple(result)
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "bang variant with config struct" do
      path = create_temp_file("bang config")

      config = %Kreuzberg.ExtractionConfig{
        ocr: %{"enabled" => true}
      }

      try do
        result = Kreuzberg.extract_file!(path, "text/plain", config)

        assert %Kreuzberg.ExtractionResult{} = result
        assert result.content == "bang config"
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "bang variant with map config" do
      path = create_temp_file("bang map")

      try do
        result =
          Kreuzberg.extract_file!(path, "text/plain", %{
            "ocr" => %{"enabled" => true}
          })

        assert result.content == "bang map"
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "proper result structure with bang variant" do
      path = create_temp_file("bang structure")

      try do
        result = Kreuzberg.extract_file!(path, "text/plain")

        assert %Kreuzberg.ExtractionResult{
                 content: content,
                 mime_type: mime_type,
                 metadata: metadata,
                 tables: tables
               } = result

        assert is_binary(content)
        assert is_binary(mime_type)
        assert is_map(metadata)
        assert is_list(tables)
      after
        cleanup_temp_file(path)
      end
    end
  end

  describe "extract_file!/3 failure cases" do
    @tag :unit
    test "raises Kreuzberg.Error on missing file" do
      non_existent = "/tmp/non_existent_file_#{System.unique_integer()}.txt"

      assert_raise Kreuzberg.Error, fn ->
        Kreuzberg.extract_file!(non_existent, "text/plain")
      end
    end

    @tag :unit
    test "raised error contains message" do
      non_existent = "/tmp/non_existent_#{System.unique_integer()}.txt"

      assert_raise Kreuzberg.Error, ~r/.+/, fn ->
        Kreuzberg.extract_file!(non_existent, "text/plain")
      end
    end

    @tag :unit
    test "bang variant raises with auto-detection on missing file" do
      non_existent = "/tmp/non_existent_auto_#{System.unique_integer()}.txt"

      assert_raise Kreuzberg.Error, fn ->
        Kreuzberg.extract_file!(non_existent)
      end
    end

    @tag :unit
    test "error is a Kreuzberg.Error exception" do
      non_existent = "/tmp/missing_#{System.unique_integer()}.txt"

      assert_raise Kreuzberg.Error, fn ->
        Kreuzberg.extract_file!(non_existent, "text/plain")
      end
    end
  end

  describe "extract_file/3 error handling" do
    @tag :unit
    test "returns error tuple for missing file" do
      non_existent = "/tmp/not_found_#{System.unique_integer()}.txt"

      {:error, reason} = Kreuzberg.extract_file(non_existent, "text/plain")

      # Error should be a non-empty string
      assert is_binary(reason) and byte_size(reason) > 0
    end

    @tag :unit
    test "returns error, not exception, for missing file" do
      non_existent = "/tmp/missing_#{System.unique_integer()}.txt"

      result = Kreuzberg.extract_file(non_existent, "text/plain")

      assert {:error, _reason} = result
    end

    @tag :unit
    test "returns error for missing file with auto-detection" do
      non_existent = "/tmp/not_found_auto_#{System.unique_integer()}.txt"

      {:error, reason} = Kreuzberg.extract_file(non_existent)

      assert is_binary(reason) and byte_size(reason) > 0
    end

    @tag :unit
    test "returns error for invalid path" do
      # Empty string is an invalid path
      {:error, reason} = Kreuzberg.extract_file("", "text/plain")

      assert is_binary(reason) and byte_size(reason) > 0
    end

    @tag :unit
    test "error message is descriptive" do
      non_existent = "/tmp/error_desc_#{System.unique_integer()}.txt"

      {:error, reason} = Kreuzberg.extract_file(non_existent, "text/plain")

      # Error message should be non-empty
      assert byte_size(reason) > 0
    end
  end

  describe "path type handling" do
    @tag :unit
    test "String path works correctly" do
      path = create_temp_file("string path")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert result.content == "string path"
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "Path.t() from Path module works correctly" do
      path_string = create_temp_file("path module")
      path = Path.expand(path_string)

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert result.content == "path module"
      after
        cleanup_temp_file(path_string)
      end
    end

    @tag :unit
    test "relative path is handled" do
      # Create a file in the current working directory
      path = "test_extraction_relative_#{System.unique_integer()}.txt"
      File.write!(path, "relative path content")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert result.content == "relative path content"
      after
        if File.exists?(path), do: File.rm(path)
      end
    end

    @tag :unit
    test "absolute path is handled" do
      path = create_temp_file("absolute path")

      try do
        abs_path = Path.expand(path)
        {:ok, result} = Kreuzberg.extract_file(abs_path, "text/plain")

        assert result.content == "absolute path"
      after
        cleanup_temp_file(path)
      end
    end
  end

  describe "result structure validation" do
    @tag :unit
    test "result contains all expected fields" do
      path = create_temp_file("full structure")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        # Verify all expected fields are present
        assert %Kreuzberg.ExtractionResult{
                 content: _,
                 mime_type: _,
                 metadata: _,
                 tables: _,
                 detected_languages: _,
                 chunks: _,
                 images: _,
                 pages: _
               } = result
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "result content is always a binary" do
      path = create_temp_file("binary content")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert is_binary(result.content)
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "result mime_type is always a binary" do
      path = create_temp_file("mime binary")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert is_binary(result.mime_type)
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "result metadata is a map" do
      path = create_temp_file("meta map")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert is_map(result.metadata)
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "result tables is a list" do
      path = create_temp_file("table list")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert is_list(result.tables)
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "result detected_languages is a list or nil" do
      path = create_temp_file("lang list")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert is_list(result.detected_languages) or is_nil(result.detected_languages)
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "result chunks is a list or nil" do
      path = create_temp_file("chunk list")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert is_list(result.chunks) or is_nil(result.chunks)
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "result images is a list or nil" do
      path = create_temp_file("image list")

      try do
        {:ok, result} = Kreuzberg.extract_file(path, "text/plain")

        assert is_list(result.images) or is_nil(result.images)
      after
        cleanup_temp_file(path)
      end
    end
  end

  describe "file extraction consistency" do
    @tag :unit
    test "same file produces consistent results" do
      path = create_temp_file("consistent content")

      try do
        {:ok, result1} = Kreuzberg.extract_file(path, "text/plain")
        {:ok, result2} = Kreuzberg.extract_file(path, "text/plain")

        assert result1.content == result2.content
        assert result1.mime_type == result2.mime_type
      after
        cleanup_temp_file(path)
      end
    end

    @tag :unit
    test "extract_file and extract_file! produce same results" do
      path = create_temp_file("same results")

      try do
        {:ok, file_result} = Kreuzberg.extract_file(path, "text/plain")
        bang_result = Kreuzberg.extract_file!(path, "text/plain")

        assert file_result.content == bang_result.content
        assert file_result.mime_type == bang_result.mime_type
      after
        cleanup_temp_file(path)
      end
    end
  end
end
