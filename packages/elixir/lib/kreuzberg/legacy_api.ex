defmodule Kreuzberg.LegacyAPI do
  @moduledoc """
  Legacy API functions using deprecated patterns.

  This module contains deprecated functions that used the old configuration approach.
  These functions will be removed in v2.0.0.

  Users should migrate to the new `Kreuzberg` module which uses the modern
  `ExtractionConfig` structure and nested configuration maps.

  ## Migration Guide

  The major change in v2.0.0 is moving from simple boolean flags and flat
  configuration to a structured `ExtractionConfig` with nested configuration maps.

  ### Old Pattern (Deprecated)

      {:ok, result} = Kreuzberg.LegacyAPI.extract_with_ocr(input, "application/pdf", true)
      {:ok, result} = Kreuzberg.LegacyAPI.extract_with_chunking(input, "text/plain", 1024, 100)

  ### New Pattern (Recommended)

      config = %Kreuzberg.ExtractionConfig{
        ocr: %{"enabled" => true, "backend" => "tesseract"},
        chunking: %{"max_chars" => 1024, "max_overlap" => 100}
      }
      {:ok, result} = Kreuzberg.extract(input, "application/pdf", config)

  See: https://docs.kreuzberg.io/v1-to-v2-migration
  """

  alias Kreuzberg.{ExtractionConfig, ExtractionResult}

  @doc """
  Extract content with deprecated boolean OCR parameter.

  This function is deprecated. Use `Kreuzberg.extract/3` with the new
  `ExtractionConfig` structure containing an `ocr` nested configuration map.

  ## Parameters

    * `input` - Binary document data
    * `mime_type` - MIME type of the document
    * `enable_ocr` - Boolean flag to enable OCR (deprecated parameter style)

  ## Returns

    * `{:ok, ExtractionResult.t()}` - Successfully extracted content
    * `{:error, reason}` - Extraction failed

  ## Deprecated

  This function will be removed in v2.0.0. Use:

      config = %ExtractionConfig{ocr: %{"enabled" => true}}
      Kreuzberg.extract(input, mime_type, config)

  ## Examples

      # Deprecated way (old):
      {:ok, result} = Kreuzberg.LegacyAPI.extract_with_ocr(pdf_binary, "application/pdf", true)

      # Recommended way (new):
      config = %ExtractionConfig{ocr: %{"enabled" => true}}
      {:ok, result} = Kreuzberg.extract(pdf_binary, "application/pdf", config)
  """
  @deprecated "Use Kreuzberg.extract/3 with ExtractionConfig.ocr map instead. Removes in v2.0.0."
  @spec extract_with_ocr(binary(), String.t(), boolean()) ::
          {:ok, ExtractionResult.t()} | {:error, String.t()}
  def extract_with_ocr(input, mime_type, enable_ocr)
      when is_binary(input) and is_binary(mime_type) do
    config =
      if enable_ocr do
        %ExtractionConfig{ocr: %{"backend" => "tesseract"}}
      else
        nil
      end

    Kreuzberg.extract(input, mime_type, config)
  end

  @doc """
  Extract content with deprecated chunking parameters.

  This function is deprecated. Use `Kreuzberg.extract/3` with the new
  `ExtractionConfig` structure containing a `chunking` nested configuration.

  ## Parameters

    * `input` - Binary document data
    * `mime_type` - MIME type of the document
    * `chunk_size` - Maximum chunk size (deprecated parameter style)
    * `overlap` - Overlap between chunks (deprecated parameter style)

  ## Returns

    * `{:ok, ExtractionResult.t()}` - Successfully extracted and chunked content
    * `{:error, reason}` - Extraction failed

  ## Deprecated

  This function will be removed in v2.0.0. Use:

      config = %ExtractionConfig{
        chunking: %{"max_chars" => 1024, "max_overlap" => 100}
      }
      Kreuzberg.extract(input, mime_type, config)

  ## Examples

      # Deprecated way (old):
      {:ok, result} = Kreuzberg.LegacyAPI.extract_with_chunking(
        pdf_binary,
        "application/pdf",
        1024,
        100
      )

      # Recommended way (new):
      config = %ExtractionConfig{
        chunking: %{"max_chars" => 1024, "max_overlap" => 100}
      }
      {:ok, result} = Kreuzberg.extract(pdf_binary, "application/pdf", config)
  """
  @deprecated "Use Kreuzberg.extract/3 with ExtractionConfig.chunking map instead. Removes in v2.0.0."
  @spec extract_with_chunking(binary(), String.t(), integer(), integer()) ::
          {:ok, ExtractionResult.t()} | {:error, String.t()}
  def extract_with_chunking(input, mime_type, chunk_size, overlap)
      when is_binary(input) and is_binary(mime_type) and is_integer(chunk_size) and
             is_integer(overlap) do
    config = %ExtractionConfig{
      chunking: %{"max_chars" => chunk_size, "max_overlap" => overlap}
    }

    Kreuzberg.extract(input, mime_type, config)
  end

  @doc """
  Extract file using deprecated simple parameter list.

  This function is deprecated. Use `Kreuzberg.extract_file/3` which accepts
  modern `ExtractionConfig` structures.

  ## Parameters

    * `path` - File path (String or Path.t())
    * `mime_type` - MIME type (optional, string or nil)
    * `opts` - Keyword list of deprecated options:
      * `:ocr` - Boolean to enable OCR
      * `:chunk_size` - Maximum chunk size
      * `:use_cache` - Enable caching

  ## Returns

    * `{:ok, ExtractionResult.t()}` - Successfully extracted content
    * `{:error, reason}` - Extraction failed

  ## Deprecated

  This function will be removed in v2.0.0. Use the modern API:

      config = %ExtractionConfig{
        ocr: %{"enabled" => true},
        chunking: %{"max_chars" => 1024},
        use_cache: true
      }
      Kreuzberg.extract_file(path, mime_type, config)

  ## Examples

      # Deprecated way (old):
      {:ok, result} = Kreuzberg.LegacyAPI.extract_file_legacy(
        "document.pdf",
        "application/pdf",
        ocr: true,
        chunk_size: 1024,
        use_cache: true
      )

      # Recommended way (new):
      config = %ExtractionConfig{
        ocr: %{"enabled" => true},
        chunking: %{"max_chars" => 1024},
        use_cache: true
      }
      {:ok, result} = Kreuzberg.extract_file("document.pdf", "application/pdf", config)
  """
  @deprecated "Use Kreuzberg.extract_file/3 with ExtractionConfig struct. Removes in v2.0.0."
  @spec extract_file_legacy(String.t() | Path.t(), String.t() | nil, keyword()) ::
          {:ok, ExtractionResult.t()} | {:error, String.t()}
  def extract_file_legacy(path, mime_type \\ nil, opts \\ []) do
    config = convert_legacy_opts_to_config(opts)
    Kreuzberg.extract_file(path, mime_type, config)
  end

  @doc """
  Extract with keyword list configuration (deprecated format).

  This function is deprecated. Use `Kreuzberg.extract/3` with modern
  `ExtractionConfig` structure.

  ## Deprecated

  The keyword list configuration format is deprecated in favor of the
  structured `ExtractionConfig` with nested configuration maps.

  ## Examples

      # Deprecated way (old):
      {:ok, result} = Kreuzberg.LegacyAPI.extract_with_options(
        input,
        "application/pdf",
        use_cache: true,
        force_ocr: false,
        output_format: "markdown"
      )

      # Recommended way (new):
      config = %ExtractionConfig{
        use_cache: true,
        force_ocr: false,
        output_format: "markdown"
      }
      {:ok, result} = Kreuzberg.extract(input, "application/pdf", config)
  """
  @deprecated "Use Kreuzberg.extract/3 with ExtractionConfig struct. Removes in v2.0.0."
  @spec extract_with_options(binary(), String.t(), keyword()) ::
          {:ok, ExtractionResult.t()} | {:error, String.t()}
  def extract_with_options(input, mime_type, opts \\ []) do
    config = convert_legacy_opts_to_config(opts)
    Kreuzberg.extract(input, mime_type, config)
  end

  @doc """
  Validate extraction request using deprecated format.

  This function is deprecated. Modern validation is built into the
  extraction functions.

  ## Deprecated

  Explicit validation functions are no longer necessary as validation
  is automatically performed by the extraction functions.
  """
  @deprecated "Validation is now automatic in extraction functions. Removes in v2.0.0."
  @spec validate_extraction_request(binary(), String.t(), keyword()) ::
          :ok | {:error, String.t()}
  def validate_extraction_request(input, mime_type, _opts \\ []) do
    cond do
      not is_binary(input) -> {:error, "Input must be binary"}
      not is_binary(mime_type) -> {:error, "MIME type must be binary"}
      String.length(mime_type) == 0 -> {:error, "MIME type cannot be empty"}
      true -> :ok
    end
  end

  # Private helpers

  @doc false
  defp convert_legacy_opts_to_config(opts) do
    %ExtractionConfig{
      use_cache: Keyword.get(opts, :use_cache, true),
      force_ocr: Keyword.get(opts, :force_ocr, false),
      enable_quality_processing: Keyword.get(opts, :enable_quality_processing, true),
      output_format: Keyword.get(opts, :output_format, "plain"),
      result_format: Keyword.get(opts, :result_format, "unified"),
      ocr: convert_ocr_opts(Keyword.get(opts, :ocr, nil)),
      chunking:
        convert_chunking_opts(
          Keyword.get(opts, :chunk_size, nil),
          Keyword.get(opts, :overlap, nil)
        ),
      language_detection: Keyword.get(opts, :language_detection, nil),
      postprocessor: Keyword.get(opts, :postprocessor, nil),
      images: Keyword.get(opts, :images, nil),
      keywords: Keyword.get(opts, :keywords, nil)
    }
  end

  @doc false
  defp convert_ocr_opts(nil), do: nil
  defp convert_ocr_opts(true), do: %{"backend" => "tesseract"}
  defp convert_ocr_opts(false), do: nil
  defp convert_ocr_opts(opts) when is_map(opts), do: opts

  @doc false
  defp convert_chunking_opts(nil, nil), do: nil

  defp convert_chunking_opts(size, nil) when is_integer(size) do
    %{"max_chars" => size}
  end

  defp convert_chunking_opts(nil, overlap) when is_integer(overlap) do
    %{"max_overlap" => overlap}
  end

  defp convert_chunking_opts(size, overlap) when is_integer(size) and is_integer(overlap) do
    %{"max_chars" => size, "max_overlap" => overlap}
  end
end
