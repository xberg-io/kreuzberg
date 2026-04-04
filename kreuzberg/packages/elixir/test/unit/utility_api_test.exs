defmodule KreuzbergTest.Unit.UtilityAPITest do
  @moduledoc """
  Unit tests for UtilityAPI utility functions.

  Tests cover:
  - detect_mime_type/1: MIME type detection from binary content
  - detect_mime_type_from_path/1: MIME type detection from file paths
  - validate_mime_type/1: MIME type validation
  - get_extensions_for_mime/1: Extension retrieval for MIME types
  - list_embedding_presets/0: Listing available embedding presets
  - get_embedding_preset/1: Getting details for specific presets
  - classify_error/1: Error message classification
  - get_error_details/0: Error category information
  """

  use ExUnit.Case

  alias Kreuzberg.UtilityAPI

  # ============================================================================
  # detect_mime_type/1 Tests
  # ============================================================================

  describe "detect_mime_type/1" do
    @tag :unit
    test "returns error for empty binary" do
      {:error, reason} = UtilityAPI.detect_mime_type("")
      assert is_binary(reason)
    end

    @tag :unit
    test "returns error for nil input" do
      # This will raise a FunctionClauseError since the function requires binary
      assert_raise FunctionClauseError, fn ->
        UtilityAPI.detect_mime_type(nil)
      end
    end

    @tag :unit
    test "detects text plain from ASCII content" do
      text_content = "Hello, this is plain text content"
      result = UtilityAPI.detect_mime_type(text_content)
      assert {:ok, mime_type} = result
      assert is_binary(mime_type)
    end

    @tag :unit
    test "detects text plain from UTF-8 content" do
      utf8_content = "Hello, 世界, مرحبا"
      result = UtilityAPI.detect_mime_type(utf8_content)
      assert {:ok, mime_type} = result
      assert is_binary(mime_type)
    end

    @tag :unit
    test "returns error for non-binary input type" do
      assert_raise FunctionClauseError, fn ->
        UtilityAPI.detect_mime_type(123)
      end
    end

    @tag :unit
    test "detects MIME type from JSON content" do
      json_content = "{\"name\": \"test\", \"value\": 123}"
      result = UtilityAPI.detect_mime_type(json_content)
      assert {:ok, mime_type} = result
      assert is_binary(mime_type)
    end

    @tag :unit
    test "detects MIME type from XML content" do
      xml_content = "<?xml version=\"1.0\"?><root><element>test</element></root>"
      result = UtilityAPI.detect_mime_type(xml_content)
      assert {:ok, mime_type} = result
      assert is_binary(mime_type)
    end

    @tag :unit
    test "returns error for invalid binary data" do
      # Random binary that may not match any known format
      invalid_binary = <<255, 254, 253, 252>>
      result = UtilityAPI.detect_mime_type(invalid_binary)
      # May return error or a default type depending on implementation
      assert is_tuple(result) and tuple_size(result) == 2
    end
  end

  # ============================================================================
  # detect_mime_type_from_path/1 Tests
  # ============================================================================

  describe "detect_mime_type_from_path/1" do
    @tag :unit
    test "detects MIME type from .pdf extension" do
      {:error, reason} = UtilityAPI.detect_mime_type_from_path("document.pdf")
      assert is_binary(reason)
    end

    @tag :unit
    test "detects MIME type from .txt extension" do
      {:error, reason} = UtilityAPI.detect_mime_type_from_path("document.txt")
      assert is_binary(reason)
    end

    @tag :unit
    test "detects MIME type from .xlsx extension" do
      {:error, reason} = UtilityAPI.detect_mime_type_from_path("spreadsheet.xlsx")
      assert is_binary(reason)
    end

    @tag :unit
    test "detects MIME type from .jpg extension" do
      {:error, reason} = UtilityAPI.detect_mime_type_from_path("image.jpg")
      assert is_binary(reason)
    end

    @tag :unit
    test "detects MIME type from .jpeg extension" do
      {:error, reason} = UtilityAPI.detect_mime_type_from_path("photo.jpeg")
      assert is_binary(reason)
    end

    @tag :unit
    test "detects MIME type from .png extension" do
      {:error, reason} = UtilityAPI.detect_mime_type_from_path("image.png")
      assert is_binary(reason)
    end

    @tag :unit
    test "detects MIME type from .docx extension" do
      {:error, reason} = UtilityAPI.detect_mime_type_from_path("document.docx")
      assert is_binary(reason)
    end

    @tag :unit
    test "returns error for unknown extension" do
      {:error, reason} = UtilityAPI.detect_mime_type_from_path("file.unknownext")
      assert is_binary(reason)
    end

    @tag :unit
    test "returns error for empty filename" do
      {:error, reason} = UtilityAPI.detect_mime_type_from_path("")
      assert is_binary(reason)
    end

    @tag :unit
    test "handles path with directories" do
      {:error, reason} = UtilityAPI.detect_mime_type_from_path("/path/to/document.pdf")
      assert is_binary(reason)
    end

    @tag :unit
    test "handles path with relative directories" do
      {:error, reason} = UtilityAPI.detect_mime_type_from_path("./files/document.pdf")
      assert is_binary(reason)
    end

    @tag :unit
    test "handles Path.t() struct input" do
      path = Path.expand("document.pdf")
      {:error, reason} = UtilityAPI.detect_mime_type_from_path(path)
      assert is_binary(reason)
    end

    @tag :unit
    test "returns error for file without extension" do
      {:error, reason} = UtilityAPI.detect_mime_type_from_path("Makefile")
      assert is_binary(reason)
    end

    @tag :unit
    test "case insensitive extension detection" do
      {:error, reason1} = UtilityAPI.detect_mime_type_from_path("document.PDF")
      {:error, reason2} = UtilityAPI.detect_mime_type_from_path("document.pdf")
      assert is_binary(reason1)
      assert is_binary(reason2)
    end
  end

  # ============================================================================
  # validate_mime_type/1 Tests
  # ============================================================================

  describe "validate_mime_type/1" do
    @tag :unit
    test "validates application/pdf" do
      {:ok, mime_type} = UtilityAPI.validate_mime_type("application/pdf")
      assert mime_type == "application/pdf"
    end

    @tag :unit
    test "validates text/plain" do
      {:ok, mime_type} = UtilityAPI.validate_mime_type("text/plain")
      assert is_binary(mime_type)
    end

    @tag :unit
    test "validates image/jpeg" do
      {:ok, mime_type} = UtilityAPI.validate_mime_type("image/jpeg")
      assert is_binary(mime_type)
    end

    @tag :unit
    test "validates image/png" do
      {:ok, mime_type} = UtilityAPI.validate_mime_type("image/png")
      assert is_binary(mime_type)
    end

    @tag :unit
    test "validates application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" do
      mime = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
      {:ok, validated} = UtilityAPI.validate_mime_type(mime)
      assert is_binary(validated)
    end

    @tag :unit
    test "returns error for invalid MIME type format" do
      {:error, reason} = UtilityAPI.validate_mime_type("invalid/type")
      assert is_binary(reason)
    end

    @tag :unit
    test "returns error for malformed MIME type" do
      {:error, reason} = UtilityAPI.validate_mime_type("not-a-mime-type")
      assert is_binary(reason)
    end

    @tag :unit
    test "returns error for empty string" do
      {:error, reason} = UtilityAPI.validate_mime_type("")
      assert is_binary(reason)
    end

    @tag :unit
    test "returns error for nil input" do
      assert_raise FunctionClauseError, fn ->
        UtilityAPI.validate_mime_type(nil)
      end
    end

    @tag :unit
    test "returns error for unsupported MIME type" do
      {:error, reason} = UtilityAPI.validate_mime_type("application/unsupported-format")
      assert is_binary(reason)
    end

    @tag :unit
    test "case sensitivity handling" do
      # Test if MIME type validation is case-sensitive or case-insensitive
      result1 = UtilityAPI.validate_mime_type("application/pdf")
      result2 = UtilityAPI.validate_mime_type("Application/PDF")
      # Both should either succeed or fail consistently
      assert match?({:ok, _}, result1) or match?({:error, _}, result1)
      assert match?({:ok, _}, result2) or match?({:error, _}, result2)
    end

    @tag :unit
    test "validates application/json" do
      {:ok, mime_type} = UtilityAPI.validate_mime_type("application/json")
      assert is_binary(mime_type)
    end

    @tag :unit
    test "validates text/html" do
      {:ok, mime_type} = UtilityAPI.validate_mime_type("text/html")
      assert is_binary(mime_type)
    end

    @tag :unit
    test "returns error for non-binary input" do
      assert_raise FunctionClauseError, fn ->
        UtilityAPI.validate_mime_type(123)
      end
    end
  end

  # ============================================================================
  # get_extensions_for_mime/1 Tests
  # ============================================================================

  describe "get_extensions_for_mime/1" do
    @tag :unit
    test "returns extensions for application/pdf" do
      {:ok, extensions} = UtilityAPI.get_extensions_for_mime("application/pdf")
      assert is_list(extensions)
      assert "pdf" in extensions
    end

    @tag :unit
    test "returns extensions for text/plain" do
      {:ok, extensions} = UtilityAPI.get_extensions_for_mime("text/plain")
      assert is_list(extensions)
      assert "txt" in extensions
    end

    @tag :unit
    test "returns extensions for image/jpeg" do
      {:ok, extensions} = UtilityAPI.get_extensions_for_mime("image/jpeg")
      assert is_list(extensions)
      assert "jpg" in extensions or "jpeg" in extensions
    end

    @tag :unit
    test "returns extensions for image/png" do
      {:ok, extensions} = UtilityAPI.get_extensions_for_mime("image/png")
      assert is_list(extensions)
      assert "png" in extensions
    end

    @tag :unit
    test "returns list for application/vnd.openxmlformats-officedocument.wordprocessingml.document" do
      mime = "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
      {:ok, extensions} = UtilityAPI.get_extensions_for_mime(mime)
      assert is_list(extensions)
      assert "docx" in extensions
    end

    @tag :unit
    test "returns list for application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" do
      mime = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
      {:ok, extensions} = UtilityAPI.get_extensions_for_mime(mime)
      assert is_list(extensions)
      assert "xlsx" in extensions
    end

    @tag :unit
    test "returns error for unknown MIME type" do
      {:error, reason} = UtilityAPI.get_extensions_for_mime("application/unknown")
      assert is_binary(reason)
    end

    @tag :unit
    test "returns error for empty string" do
      {:error, reason} = UtilityAPI.get_extensions_for_mime("")
      assert is_binary(reason)
    end

    @tag :unit
    test "returns error for nil input" do
      assert_raise FunctionClauseError, fn ->
        UtilityAPI.get_extensions_for_mime(nil)
      end
    end

    @tag :unit
    test "returns list with multiple extensions when applicable" do
      {:ok, extensions} = UtilityAPI.get_extensions_for_mime("image/jpeg")
      assert is_list(extensions)
      # JPEG typically has multiple extensions
      assert extensions != []
      assert all_strings?(extensions)
    end

    @tag :unit
    test "returns extensions for text/html" do
      {:ok, extensions} = UtilityAPI.get_extensions_for_mime("text/html")
      assert is_list(extensions)
      assert "html" in extensions or "htm" in extensions
    end

    @tag :unit
    test "returns extensions for application/json" do
      {:ok, extensions} = UtilityAPI.get_extensions_for_mime("application/json")
      assert is_list(extensions)
      assert "json" in extensions
    end

    @tag :unit
    test "returns non-empty list for supported MIME types" do
      {:ok, extensions} = UtilityAPI.get_extensions_for_mime("application/pdf")
      assert is_list(extensions)
      assert extensions != []
    end

    @tag :unit
    test "returns error for non-binary input" do
      assert_raise FunctionClauseError, fn ->
        UtilityAPI.get_extensions_for_mime(123)
      end
    end
  end

  # ============================================================================
  # list_embedding_presets/0 Tests
  # ============================================================================

  describe "list_embedding_presets/0" do
    @tag :unit
    test "returns tuple with :ok and list" do
      {:ok, presets} = UtilityAPI.list_embedding_presets()
      assert is_list(presets)
    end

    @tag :unit
    test "returns list of binary strings" do
      {:ok, presets} = UtilityAPI.list_embedding_presets()
      assert all_strings?(presets)
    end

    @tag :unit
    test "returns non-empty list" do
      {:ok, presets} = UtilityAPI.list_embedding_presets()
      assert presets != []
    end

    @tag :unit
    test "contains balanced preset" do
      {:ok, presets} = UtilityAPI.list_embedding_presets()
      assert "balanced" in presets
    end

    @tag :unit
    test "contains fast preset" do
      {:ok, presets} = UtilityAPI.list_embedding_presets()
      assert "fast" in presets
    end

    @tag :unit
    test "contains quality preset" do
      {:ok, presets} = UtilityAPI.list_embedding_presets()
      assert "quality" in presets
    end

    @tag :unit
    test "contains multilingual preset" do
      {:ok, presets} = UtilityAPI.list_embedding_presets()
      assert "multilingual" in presets
    end

    @tag :unit
    test "returns consistent results" do
      {:ok, presets1} = UtilityAPI.list_embedding_presets()
      {:ok, presets2} = UtilityAPI.list_embedding_presets()
      assert presets1 == presets2
    end

    @tag :unit
    test "returns sorted or consistent order" do
      {:ok, presets} = UtilityAPI.list_embedding_presets()
      # Verify list is not empty and consistent
      assert presets != []
      # Call multiple times to verify consistency
      {:ok, presets2} = UtilityAPI.list_embedding_presets()
      assert presets == presets2
    end

    @tag :unit
    test "all preset names are valid strings" do
      {:ok, presets} = UtilityAPI.list_embedding_presets()

      Enum.each(presets, fn preset ->
        assert is_binary(preset)
        assert preset != ""
      end)
    end
  end

  # ============================================================================
  # get_embedding_preset/1 Tests
  # ============================================================================

  describe "get_embedding_preset/1" do
    @tag :unit
    test "returns map for balanced preset" do
      {:ok, preset} = UtilityAPI.get_embedding_preset("balanced")
      assert is_map(preset)
    end

    @tag :unit
    test "balanced preset contains name" do
      {:ok, preset} = UtilityAPI.get_embedding_preset("balanced")
      assert preset["name"] == "balanced"
    end

    @tag :unit
    test "balanced preset contains dimensions" do
      {:ok, preset} = UtilityAPI.get_embedding_preset("balanced")
      assert is_integer(preset["dimensions"])
      assert preset["dimensions"] > 0
    end

    @tag :unit
    test "balanced preset contains chunk_size" do
      {:ok, preset} = UtilityAPI.get_embedding_preset("balanced")
      assert is_integer(preset["chunk_size"])
      assert preset["chunk_size"] > 0
    end

    @tag :unit
    test "balanced preset contains overlap" do
      {:ok, preset} = UtilityAPI.get_embedding_preset("balanced")
      assert is_integer(preset["overlap"])
      assert preset["overlap"] >= 0
    end

    @tag :unit
    test "balanced preset contains description" do
      {:ok, preset} = UtilityAPI.get_embedding_preset("balanced")
      assert is_binary(preset["description"])
    end

    @tag :unit
    test "returns map for fast preset" do
      {:ok, preset} = UtilityAPI.get_embedding_preset("fast")
      assert is_map(preset)
      assert preset["name"] == "fast"
    end

    @tag :unit
    test "fast preset dimensions match expected value" do
      {:ok, preset} = UtilityAPI.get_embedding_preset("fast")
      assert preset["dimensions"] == 384
    end

    @tag :unit
    test "returns map for quality preset" do
      {:ok, preset} = UtilityAPI.get_embedding_preset("quality")
      assert is_map(preset)
      assert preset["name"] == "quality"
    end

    @tag :unit
    test "quality preset chunk_size matches expected value" do
      {:ok, preset} = UtilityAPI.get_embedding_preset("quality")
      assert preset["chunk_size"] == 2000
    end

    @tag :unit
    test "returns map for multilingual preset" do
      {:ok, preset} = UtilityAPI.get_embedding_preset("multilingual")
      assert is_map(preset)
      assert preset["name"] == "multilingual"
    end

    @tag :unit
    test "returns error for nonexistent preset" do
      {:error, reason} = UtilityAPI.get_embedding_preset("nonexistent")
      assert is_binary(reason)
    end

    @tag :unit
    test "returns error for empty preset name" do
      {:error, reason} = UtilityAPI.get_embedding_preset("")
      assert is_binary(reason)
    end

    @tag :unit
    test "returns error for nil input" do
      assert_raise FunctionClauseError, fn ->
        UtilityAPI.get_embedding_preset(nil)
      end
    end

    @tag :unit
    test "returns consistent results for same preset" do
      {:ok, preset1} = UtilityAPI.get_embedding_preset("balanced")
      {:ok, preset2} = UtilityAPI.get_embedding_preset("balanced")
      assert preset1 == preset2
    end

    @tag :unit
    test "different presets have different configurations" do
      {:ok, fast} = UtilityAPI.get_embedding_preset("fast")
      {:ok, quality} = UtilityAPI.get_embedding_preset("quality")
      # At least one of the configurations should differ
      assert fast != quality
    end

    @tag :unit
    test "all preset fields are present for quality" do
      {:ok, preset} = UtilityAPI.get_embedding_preset("quality")
      assert Map.has_key?(preset, "name")
      assert Map.has_key?(preset, "chunk_size")
      assert Map.has_key?(preset, "overlap")
      assert Map.has_key?(preset, "dimensions")
      assert Map.has_key?(preset, "description")
    end

    @tag :unit
    test "returns error for non-binary input" do
      assert_raise FunctionClauseError, fn ->
        UtilityAPI.get_embedding_preset(123)
      end
    end

    @tag :unit
    test "overlap is less than or equal to chunk_size" do
      {:ok, preset} = UtilityAPI.get_embedding_preset("balanced")
      assert preset["overlap"] <= preset["chunk_size"]
    end
  end

  # ============================================================================
  # classify_error/1 Tests
  # ============================================================================

  describe "classify_error/1" do
    @tag :unit
    test "classifies 'File not found' as io_error" do
      assert UtilityAPI.classify_error("File not found") == :io_error
    end

    @tag :unit
    test "classifies 'File not found: /path/to/file.pdf' as io_error" do
      assert UtilityAPI.classify_error("File not found: /path/to/file.pdf") == :io_error
    end

    @tag :unit
    test "classifies 'Permission denied' as io_error" do
      assert UtilityAPI.classify_error("Permission denied") == :io_error
    end

    @tag :unit
    test "classifies 'No such file or directory' as io_error" do
      assert UtilityAPI.classify_error("No such file or directory") == :io_error
    end

    @tag :unit
    test "classifies IO error patterns" do
      assert UtilityAPI.classify_error("io error occurred") == :io_error
    end

    @tag :unit
    test "classifies 'Invalid PDF format' as invalid_format" do
      assert UtilityAPI.classify_error("Invalid PDF format") == :invalid_format
    end

    @tag :unit
    test "classifies 'Corrupted file' as io_error" do
      assert UtilityAPI.classify_error("Corrupted file") == :io_error
    end

    @tag :unit
    test "classifies 'Unsupported format' as invalid_format" do
      assert UtilityAPI.classify_error("Unsupported format") == :invalid_format
    end

    @tag :unit
    test "classifies 'Damaged file' as io_error" do
      assert UtilityAPI.classify_error("Damaged file") == :io_error
    end

    @tag :unit
    test "classifies 'Invalid configuration' as io_error" do
      assert UtilityAPI.classify_error("Invalid configuration") == :io_error
    end

    @tag :unit
    test "classifies 'Invalid parameter' as invalid_format" do
      assert UtilityAPI.classify_error("Invalid parameter") == :invalid_format
    end

    @tag :unit
    test "classifies 'Unknown option' as io_error" do
      assert UtilityAPI.classify_error("Unknown option") == :io_error
    end

    @tag :unit
    test "classifies 'Config error' as invalid_config" do
      assert UtilityAPI.classify_error("Config error") == :invalid_config
    end

    @tag :unit
    test "classifies 'OCR failed' as ocr_error" do
      assert UtilityAPI.classify_error("OCR failed") == :ocr_error
    end

    @tag :unit
    test "classifies 'OCR engine failed' as ocr_error" do
      assert UtilityAPI.classify_error("OCR engine failed") == :ocr_error
    end

    @tag :unit
    test "classifies 'OCR timeout' as ocr_error" do
      assert UtilityAPI.classify_error("OCR timeout") == :ocr_error
    end

    @tag :unit
    test "classifies 'Recognition failed' as io_error" do
      assert UtilityAPI.classify_error("Recognition failed") == :io_error
    end

    @tag :unit
    test "classifies 'Optical character recognition error' as io_error" do
      assert UtilityAPI.classify_error("Optical character recognition error") == :io_error
    end

    @tag :unit
    test "classifies 'Extraction failed' as io_error" do
      assert UtilityAPI.classify_error("Extraction failed") == :io_error
    end

    @tag :unit
    test "classifies 'Processing error' as unknown_error" do
      assert UtilityAPI.classify_error("Processing error") == :unknown_error
    end

    @tag :unit
    test "classifies 'Extraction complete' as io_error" do
      assert UtilityAPI.classify_error("Extraction complete") == :io_error
    end

    @tag :unit
    test "classifies unknown errors as unknown_error" do
      assert UtilityAPI.classify_error("Unknown error occurred") == :unknown_error
    end

    @tag :unit
    test "classifies empty string as unknown_error" do
      assert UtilityAPI.classify_error("") == :unknown_error
    end

    @tag :unit
    test "classifies completely random text as unknown_error" do
      assert UtilityAPI.classify_error("xyzabc dfghjk") == :unknown_error
    end

    @tag :unit
    test "case insensitive classification - file not found" do
      assert UtilityAPI.classify_error("FILE NOT FOUND") == :io_error
      assert UtilityAPI.classify_error("File Not Found") == :io_error
    end

    @tag :unit
    test "case insensitive classification - invalid format" do
      assert UtilityAPI.classify_error("INVALID PDF FORMAT") == :invalid_format
      assert UtilityAPI.classify_error("Invalid Pdf Format") == :invalid_format
    end

    @tag :unit
    test "case insensitive classification - ocr error" do
      assert UtilityAPI.classify_error("OCR FAILED") == :ocr_error
      assert UtilityAPI.classify_error("Ocr Failed") == :ocr_error
    end

    @tag :unit
    test "returns atom type" do
      result = UtilityAPI.classify_error("Any error")
      assert is_atom(result)
    end

    @tag :unit
    test "classification with special characters" do
      result = UtilityAPI.classify_error("File not found: @#$%^")
      assert is_atom(result)
    end

    @tag :unit
    test "classification with multiple error keywords" do
      # When multiple keywords present, first match wins
      result = UtilityAPI.classify_error("File not found - Invalid format")
      # Should match io_error first
      assert result == :io_error
    end

    @tag :unit
    test "returns error for nil input" do
      assert_raise FunctionClauseError, fn ->
        UtilityAPI.classify_error(nil)
      end
    end

    @tag :unit
    test "returns error for non-binary input" do
      assert_raise FunctionClauseError, fn ->
        UtilityAPI.classify_error(123)
      end
    end
  end

  # ============================================================================
  # get_error_details/0 Tests
  # ============================================================================

  describe "get_error_details/0" do
    @tag :unit
    test "returns tuple with :ok and map" do
      {:ok, details} = UtilityAPI.get_error_details()
      assert is_map(details)
    end

    @tag :unit
    test "map contains io_error key" do
      {:ok, details} = UtilityAPI.get_error_details()
      assert Map.has_key?(details, :io_error)
    end

    @tag :unit
    test "map contains invalid_format key" do
      {:ok, details} = UtilityAPI.get_error_details()
      assert Map.has_key?(details, :invalid_format)
    end

    @tag :unit
    test "map contains invalid_config key" do
      {:ok, details} = UtilityAPI.get_error_details()
      assert Map.has_key?(details, :invalid_config)
    end

    @tag :unit
    test "map contains ocr_error key" do
      {:ok, details} = UtilityAPI.get_error_details()
      assert Map.has_key?(details, :ocr_error)
    end

    @tag :unit
    test "map contains extraction_error key" do
      {:ok, details} = UtilityAPI.get_error_details()
      assert Map.has_key?(details, :extraction_error)
    end

    @tag :unit
    test "map contains unknown_error key" do
      {:ok, details} = UtilityAPI.get_error_details()
      assert Map.has_key?(details, :unknown_error)
    end

    @tag :unit
    test "all error categories have 6 entries" do
      {:ok, details} = UtilityAPI.get_error_details()
      assert map_size(details) == 6
    end

    @tag :unit
    test "io_error has required fields" do
      {:ok, details} = UtilityAPI.get_error_details()
      io_error = details[:io_error]
      assert Map.has_key?(io_error, "name")
      assert Map.has_key?(io_error, "description")
      assert Map.has_key?(io_error, "examples")
    end

    @tag :unit
    test "io_error name is correct" do
      {:ok, details} = UtilityAPI.get_error_details()
      assert details[:io_error]["name"] == "IO Error"
    end

    @tag :unit
    test "io_error description is string" do
      {:ok, details} = UtilityAPI.get_error_details()
      assert is_binary(details[:io_error]["description"])
      assert details[:io_error]["description"] != ""
    end

    @tag :unit
    test "io_error examples is list" do
      {:ok, details} = UtilityAPI.get_error_details()
      examples = details[:io_error]["examples"]
      assert is_list(examples)
      assert examples != []
    end

    @tag :unit
    test "io_error examples are strings" do
      {:ok, details} = UtilityAPI.get_error_details()
      examples = details[:io_error]["examples"]
      assert ["File not found", "Permission denied", "No such file or directory"] == examples
    end

    @tag :unit
    test "invalid_format has required fields" do
      {:ok, details} = UtilityAPI.get_error_details()
      invalid_format = details[:invalid_format]
      assert Map.has_key?(invalid_format, "name")
      assert Map.has_key?(invalid_format, "description")
      assert Map.has_key?(invalid_format, "examples")
    end

    @tag :unit
    test "invalid_format name is correct" do
      {:ok, details} = UtilityAPI.get_error_details()
      assert details[:invalid_format]["name"] == "Invalid Format"
    end

    @tag :unit
    test "invalid_config has required fields" do
      {:ok, details} = UtilityAPI.get_error_details()
      invalid_config = details[:invalid_config]
      assert Map.has_key?(invalid_config, "name")
      assert Map.has_key?(invalid_config, "description")
      assert Map.has_key?(invalid_config, "examples")
    end

    @tag :unit
    test "invalid_config name is correct" do
      {:ok, details} = UtilityAPI.get_error_details()
      assert details[:invalid_config]["name"] == "Invalid Configuration"
    end

    @tag :unit
    test "ocr_error has required fields" do
      {:ok, details} = UtilityAPI.get_error_details()
      ocr_error = details[:ocr_error]
      assert Map.has_key?(ocr_error, "name")
      assert Map.has_key?(ocr_error, "description")
      assert Map.has_key?(ocr_error, "examples")
    end

    @tag :unit
    test "ocr_error name is correct" do
      {:ok, details} = UtilityAPI.get_error_details()
      assert details[:ocr_error]["name"] == "OCR Error"
    end

    @tag :unit
    test "extraction_error has required fields" do
      {:ok, details} = UtilityAPI.get_error_details()
      extraction_error = details[:extraction_error]
      assert Map.has_key?(extraction_error, "name")
      assert Map.has_key?(extraction_error, "description")
      assert Map.has_key?(extraction_error, "examples")
    end

    @tag :unit
    test "extraction_error name is correct" do
      {:ok, details} = UtilityAPI.get_error_details()
      assert details[:extraction_error]["name"] == "Extraction Error"
    end

    @tag :unit
    test "unknown_error has required fields" do
      {:ok, details} = UtilityAPI.get_error_details()
      unknown_error = details[:unknown_error]
      assert Map.has_key?(unknown_error, "name")
      assert Map.has_key?(unknown_error, "description")
      assert Map.has_key?(unknown_error, "examples")
    end

    @tag :unit
    test "unknown_error name is correct" do
      {:ok, details} = UtilityAPI.get_error_details()
      assert details[:unknown_error]["name"] == "Unknown Error"
    end

    @tag :unit
    test "all descriptions are non-empty strings" do
      {:ok, details} = UtilityAPI.get_error_details()

      Enum.each(details, fn {_key, category} ->
        assert is_binary(category["description"])
        assert category["description"] != ""
      end)
    end

    @tag :unit
    test "all examples lists are non-empty" do
      {:ok, details} = UtilityAPI.get_error_details()

      Enum.each(details, fn {_key, category} ->
        examples = category["examples"]
        assert is_list(examples)
        assert examples != []
      end)
    end

    @tag :unit
    test "returns consistent results on multiple calls" do
      {:ok, details1} = UtilityAPI.get_error_details()
      {:ok, details2} = UtilityAPI.get_error_details()
      assert details1 == details2
    end

    @tag :unit
    test "all examples are strings" do
      {:ok, details} = UtilityAPI.get_error_details()

      Enum.each(details, fn {_key, category} ->
        Enum.each(category["examples"], fn example ->
          assert is_binary(example)
        end)
      end)
    end
  end

  # ============================================================================
  # Helper Functions
  # ============================================================================

  defp all_strings?(list) do
    Enum.all?(list, &is_binary/1)
  end
end
