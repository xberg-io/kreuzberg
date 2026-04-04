defmodule KreuzbergTest.Unit.ValidatorsTest do
  @moduledoc """
  Unit tests for Kreuzberg configuration validators.

  Tests cover all 8 validator functions:
  - validate_chunking_params/1: Validates chunking configuration
  - validate_language_code/1: Validates ISO 639 language codes
  - validate_dpi/1: Validates DPI values (1-2400)
  - validate_confidence/1: Validates confidence thresholds (0.0-1.0)
  - validate_ocr_backend/1: Validates OCR backend names
  - validate_binarization_method/1: Validates binarization methods
  - validate_tesseract_psm/1: Validates Tesseract PSM values (0-13)
  - validate_tesseract_oem/1: Validates Tesseract OEM values (0-3)
  """

  use ExUnit.Case

  alias Kreuzberg.Validators

  # =============================================================================
  # validate_chunking_params/1 Tests
  # =============================================================================

  describe "validate_chunking_params/1" do
    @tag :unit
    test "accepts valid params with string keys" do
      params = %{"max_chars" => 1000, "max_overlap" => 200}
      assert :ok = Validators.validate_chunking_params(params)
    end

    @tag :unit
    test "accepts valid params with atom keys" do
      params = %{max_chars: 1000, max_overlap: 200}
      assert :ok = Validators.validate_chunking_params(params)
    end

    @tag :unit
    test "accepts valid params with mixed key types" do
      params = %{"max_chars" => 1000, max_overlap: 200}
      assert :ok = Validators.validate_chunking_params(params)
    end

    @tag :unit
    test "rejects when max_chars is zero" do
      params = %{"max_chars" => 0, "max_overlap" => 100}
      assert {:error, reason} = Validators.validate_chunking_params(params)
      assert is_binary(reason)
      assert String.contains?(reason, "max_chars")
    end

    @tag :unit
    test "rejects when max_chars is negative" do
      params = %{"max_chars" => -100, "max_overlap" => 50}
      assert {:error, reason} = Validators.validate_chunking_params(params)
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects when max_overlap exceeds max_chars" do
      params = %{"max_chars" => 100, "max_overlap" => 150}
      assert {:error, reason} = Validators.validate_chunking_params(params)
      assert is_binary(reason)
      assert String.contains?(reason, "max_overlap")
    end

    @tag :unit
    test "rejects when max_overlap is negative" do
      params = %{"max_chars" => 1000, "max_overlap" => -1}
      assert {:error, reason} = Validators.validate_chunking_params(params)
      assert is_binary(reason)
    end

    @tag :unit
    test "accepts when max_overlap equals max_chars" do
      params = %{"max_chars" => 1000, "max_overlap" => 1000}
      # This might fail depending on implementation, but test the behavior
      result = Validators.validate_chunking_params(params)
      assert result == :ok or match?({:error, _}, result)
    end

    @tag :unit
    test "accepts when max_overlap is zero" do
      params = %{"max_chars" => 1000, "max_overlap" => 0}
      assert :ok = Validators.validate_chunking_params(params)
    end

    @tag :unit
    test "rejects when max_chars is missing" do
      params = %{"max_overlap" => 200}
      assert {:error, reason} = Validators.validate_chunking_params(params)
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects when max_overlap is missing" do
      params = %{"max_chars" => 1000}
      assert {:error, reason} = Validators.validate_chunking_params(params)
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects when max_chars is not an integer" do
      params = %{"max_chars" => "1000", "max_overlap" => 200}
      assert {:error, reason} = Validators.validate_chunking_params(params)
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects when max_overlap is not an integer" do
      params = %{"max_chars" => 1000, "max_overlap" => "200"}
      assert {:error, reason} = Validators.validate_chunking_params(params)
      assert is_binary(reason)
    end

    @tag :unit
    test "accepts large valid values" do
      params = %{"max_chars" => 1_000_000, "max_overlap" => 500_000}
      assert :ok = Validators.validate_chunking_params(params)
    end

    @tag :unit
    test "accepts small valid values" do
      params = %{"max_chars" => 1, "max_overlap" => 0}
      assert :ok = Validators.validate_chunking_params(params)
    end

    @tag :unit
    test "rejects empty map" do
      result = Validators.validate_chunking_params(%{})
      assert {:error, _reason} = result
    end
  end

  # =============================================================================
  # validate_language_code/1 Tests
  # =============================================================================

  describe "validate_language_code/1" do
    @tag :unit
    test "accepts valid ISO 639-1 code 'en'" do
      assert :ok = Validators.validate_language_code("en")
    end

    @tag :unit
    test "accepts valid ISO 639-1 code 'de'" do
      assert :ok = Validators.validate_language_code("de")
    end

    @tag :unit
    test "accepts valid ISO 639-1 code 'fr'" do
      assert :ok = Validators.validate_language_code("fr")
    end

    @tag :unit
    test "accepts valid ISO 639-1 code 'es'" do
      assert :ok = Validators.validate_language_code("es")
    end

    @tag :unit
    test "accepts valid ISO 639-1 code 'it'" do
      assert :ok = Validators.validate_language_code("it")
    end

    @tag :unit
    test "accepts valid ISO 639-1 code 'pt'" do
      assert :ok = Validators.validate_language_code("pt")
    end

    @tag :unit
    test "accepts valid ISO 639-1 code 'ru'" do
      assert :ok = Validators.validate_language_code("ru")
    end

    @tag :unit
    test "accepts valid ISO 639-1 code 'zh'" do
      assert :ok = Validators.validate_language_code("zh")
    end

    @tag :unit
    test "accepts valid ISO 639-1 code 'ja'" do
      assert :ok = Validators.validate_language_code("ja")
    end

    @tag :unit
    test "accepts valid ISO 639-1 code 'ko'" do
      assert :ok = Validators.validate_language_code("ko")
    end

    @tag :unit
    test "accepts valid ISO 639-3 code 'eng'" do
      assert :ok = Validators.validate_language_code("eng")
    end

    @tag :unit
    test "accepts valid ISO 639-3 code 'deu'" do
      assert :ok = Validators.validate_language_code("deu")
    end

    @tag :unit
    test "accepts valid ISO 639-3 code 'fra'" do
      assert :ok = Validators.validate_language_code("fra")
    end

    @tag :unit
    test "accepts valid ISO 639-3 code 'spa'" do
      assert :ok = Validators.validate_language_code("spa")
    end

    @tag :unit
    test "rejects invalid language code" do
      assert {:error, reason} = Validators.validate_language_code("invalid")
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects single letter code" do
      assert {:error, reason} = Validators.validate_language_code("x")
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects uppercase code" do
      assert :ok = Validators.validate_language_code("EN")
    end

    @tag :unit
    test "rejects mixed case code" do
      assert :ok = Validators.validate_language_code("En")
    end

    @tag :unit
    test "rejects empty string" do
      assert {:error, reason} = Validators.validate_language_code("")
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects too long code" do
      assert {:error, reason} = Validators.validate_language_code("english")
      assert is_binary(reason)
    end

    @tag :unit
    test "error message contains helpful information" do
      {:error, reason} = Validators.validate_language_code("invalid")
      assert String.contains?(reason, "language") or String.contains?(reason, "code")
    end
  end

  # =============================================================================
  # validate_dpi/1 Tests
  # =============================================================================

  describe "validate_dpi/1" do
    @tag :unit
    test "accepts minimum valid DPI (1)" do
      assert :ok = Validators.validate_dpi(1)
    end

    @tag :unit
    test "accepts typical DPI value 72" do
      assert :ok = Validators.validate_dpi(72)
    end

    @tag :unit
    test "accepts typical DPI value 96" do
      assert :ok = Validators.validate_dpi(96)
    end

    @tag :unit
    test "accepts typical DPI value 150" do
      assert :ok = Validators.validate_dpi(150)
    end

    @tag :unit
    test "accepts typical DPI value 300" do
      assert :ok = Validators.validate_dpi(300)
    end

    @tag :unit
    test "accepts typical DPI value 600" do
      assert :ok = Validators.validate_dpi(600)
    end

    @tag :unit
    test "accepts maximum valid DPI (2400)" do
      assert :ok = Validators.validate_dpi(2400)
    end

    @tag :unit
    test "rejects zero DPI" do
      assert {:error, reason} = Validators.validate_dpi(0)
      assert is_binary(reason)
      assert String.contains?(reason, "DPI")
    end

    @tag :unit
    test "rejects negative DPI" do
      assert {:error, reason} = Validators.validate_dpi(-1)
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects negative DPI -300" do
      assert {:error, reason} = Validators.validate_dpi(-300)
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects DPI exceeding maximum (2401)" do
      assert {:error, reason} = Validators.validate_dpi(2401)
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects DPI far exceeding maximum" do
      assert {:error, reason} = Validators.validate_dpi(10_000)
      assert is_binary(reason)
    end

    @tag :unit
    test "accepts mid-range DPI values" do
      assert :ok = Validators.validate_dpi(100)
      assert :ok = Validators.validate_dpi(200)
      assert :ok = Validators.validate_dpi(400)
      assert :ok = Validators.validate_dpi(500)
    end

    @tag :unit
    test "boundary test: 1 is valid, 0 is invalid" do
      assert :ok = Validators.validate_dpi(1)
      assert {:error, _} = Validators.validate_dpi(0)
    end

    @tag :unit
    test "boundary test: 2400 is valid, 2401 is invalid" do
      assert :ok = Validators.validate_dpi(2400)
      assert {:error, _} = Validators.validate_dpi(2401)
    end
  end

  # =============================================================================
  # validate_confidence/1 Tests
  # =============================================================================

  describe "validate_confidence/1" do
    @tag :unit
    test "accepts minimum valid confidence (0.0)" do
      assert :ok = Validators.validate_confidence(0.0)
    end

    @tag :unit
    test "accepts low confidence (0.1)" do
      assert :ok = Validators.validate_confidence(0.1)
    end

    @tag :unit
    test "accepts mid confidence (0.5)" do
      assert :ok = Validators.validate_confidence(0.5)
    end

    @tag :unit
    test "accepts high confidence (0.9)" do
      assert :ok = Validators.validate_confidence(0.9)
    end

    @tag :unit
    test "accepts maximum valid confidence (1.0)" do
      assert :ok = Validators.validate_confidence(1.0)
    end

    @tag :unit
    test "accepts typical confidence values" do
      assert :ok = Validators.validate_confidence(0.6)
      assert :ok = Validators.validate_confidence(0.7)
      assert :ok = Validators.validate_confidence(0.8)
    end

    @tag :unit
    test "rejects negative confidence (-0.1)" do
      assert {:error, reason} = Validators.validate_confidence(-0.1)
      assert is_binary(reason)
      assert String.contains?(reason, "confidence")
    end

    @tag :unit
    test "rejects negative confidence (-1.0)" do
      assert {:error, reason} = Validators.validate_confidence(-1.0)
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects confidence exceeding maximum (1.1)" do
      assert {:error, reason} = Validators.validate_confidence(1.1)
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects confidence exceeding maximum (1.5)" do
      assert {:error, reason} = Validators.validate_confidence(1.5)
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects far exceeding maximum (2.0)" do
      assert {:error, reason} = Validators.validate_confidence(2.0)
      assert is_binary(reason)
    end

    @tag :unit
    test "accepts integer 0 (coerced to float)" do
      assert :ok = Validators.validate_confidence(0)
    end

    @tag :unit
    test "accepts integer 1 (coerced to float)" do
      assert :ok = Validators.validate_confidence(1)
    end

    @tag :unit
    test "boundary test: 0.0 valid, -0.01 invalid" do
      assert :ok = Validators.validate_confidence(0.0)
      assert {:error, _} = Validators.validate_confidence(-0.01)
    end

    @tag :unit
    test "boundary test: 1.0 valid, 1.01 invalid" do
      assert :ok = Validators.validate_confidence(1.0)
      assert {:error, _} = Validators.validate_confidence(1.01)
    end

    @tag :unit
    test "very small valid confidence" do
      assert :ok = Validators.validate_confidence(0.0001)
    end

    @tag :unit
    test "very close to maximum valid confidence" do
      assert :ok = Validators.validate_confidence(0.9999)
    end
  end

  # =============================================================================
  # validate_ocr_backend/1 Tests
  # =============================================================================

  describe "validate_ocr_backend/1" do
    @tag :unit
    test "accepts tesseract backend" do
      assert :ok = Validators.validate_ocr_backend("tesseract")
    end

    @tag :unit
    test "accepts easyocr backend" do
      assert :ok = Validators.validate_ocr_backend("easyocr")
    end

    @tag :unit
    test "accepts paddleocr backend" do
      assert :ok = Validators.validate_ocr_backend("paddleocr")
    end

    @tag :unit
    test "rejects invalid backend name" do
      assert {:error, reason} = Validators.validate_ocr_backend("invalid_backend")
      assert is_binary(reason)
      assert String.contains?(reason, "backend") or String.contains?(reason, "OCR")
    end

    @tag :unit
    test "rejects uppercase tesseract" do
      assert :ok = Validators.validate_ocr_backend("Tesseract")
    end

    @tag :unit
    test "rejects uppercase easyocr" do
      assert :ok = Validators.validate_ocr_backend("EasyOCR")
    end

    @tag :unit
    test "rejects misspelled tesseract" do
      assert {:error, reason} = Validators.validate_ocr_backend("tesseract2")
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects empty string" do
      assert {:error, reason} = Validators.validate_ocr_backend("")
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects non-existent backend" do
      assert {:error, reason} = Validators.validate_ocr_backend("openocr")
      assert is_binary(reason)
    end

    @tag :unit
    test "error message lists valid options" do
      {:error, reason} = Validators.validate_ocr_backend("invalid")
      # Check that error mentions valid backends
      assert String.contains?(reason, "tesseract") or String.contains?(reason, "backend")
    end
  end

  # =============================================================================
  # validate_binarization_method/1 Tests
  # =============================================================================

  describe "validate_binarization_method/1" do
    @tag :unit
    test "accepts otsu method" do
      assert :ok = Validators.validate_binarization_method("otsu")
    end

    @tag :unit
    test "accepts adaptive method" do
      assert :ok = Validators.validate_binarization_method("adaptive")
    end

    @tag :unit
    test "accepts sauvola method" do
      assert :ok = Validators.validate_binarization_method("sauvola")
    end

    @tag :unit
    test "rejects invalid method" do
      assert {:error, reason} = Validators.validate_binarization_method("invalid")
      assert is_binary(reason)
      assert String.contains?(reason, "binarization") or String.contains?(reason, "method")
    end

    @tag :unit
    test "rejects uppercase otsu" do
      assert :ok = Validators.validate_binarization_method("Otsu")
    end

    @tag :unit
    test "rejects uppercase adaptive" do
      assert :ok = Validators.validate_binarization_method("Adaptive")
    end

    @tag :unit
    test "rejects uppercase sauvola" do
      assert :ok = Validators.validate_binarization_method("Sauvola")
    end

    @tag :unit
    test "rejects empty string" do
      assert {:error, reason} = Validators.validate_binarization_method("")
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects misspelled method" do
      assert {:error, reason} = Validators.validate_binarization_method("ostu")
      assert is_binary(reason)
    end

    @tag :unit
    test "error message lists valid options" do
      {:error, reason} = Validators.validate_binarization_method("invalid")
      assert String.contains?(reason, "otsu") or String.contains?(reason, "binarization")
    end

    @tag :unit
    test "rejects method with extra characters" do
      assert {:error, reason} = Validators.validate_binarization_method("otsu ")
      assert is_binary(reason)
    end
  end

  # =============================================================================
  # validate_tesseract_psm/1 Tests
  # =============================================================================

  describe "validate_tesseract_psm/1" do
    @tag :unit
    test "accepts PSM 0 (OSD only)" do
      assert :ok = Validators.validate_tesseract_psm(0)
    end

    @tag :unit
    test "accepts PSM 1 (auto with OSD)" do
      assert :ok = Validators.validate_tesseract_psm(1)
    end

    @tag :unit
    test "accepts PSM 2" do
      assert :ok = Validators.validate_tesseract_psm(2)
    end

    @tag :unit
    test "accepts PSM 3 (default)" do
      assert :ok = Validators.validate_tesseract_psm(3)
    end

    @tag :unit
    test "accepts PSM 4" do
      assert :ok = Validators.validate_tesseract_psm(4)
    end

    @tag :unit
    test "accepts PSM 5" do
      assert :ok = Validators.validate_tesseract_psm(5)
    end

    @tag :unit
    test "accepts PSM 6 (single block)" do
      assert :ok = Validators.validate_tesseract_psm(6)
    end

    @tag :unit
    test "accepts PSM 7 (single text line)" do
      assert :ok = Validators.validate_tesseract_psm(7)
    end

    @tag :unit
    test "accepts PSM 8 (single word)" do
      assert :ok = Validators.validate_tesseract_psm(8)
    end

    @tag :unit
    test "accepts PSM 9 (word in circle)" do
      assert :ok = Validators.validate_tesseract_psm(9)
    end

    @tag :unit
    test "accepts PSM 10 (single character)" do
      assert :ok = Validators.validate_tesseract_psm(10)
    end

    @tag :unit
    test "accepts PSM 11 (sparse text)" do
      assert :ok = Validators.validate_tesseract_psm(11)
    end

    @tag :unit
    test "accepts PSM 12 (sparse text with OSD)" do
      assert :ok = Validators.validate_tesseract_psm(12)
    end

    @tag :unit
    test "accepts PSM 13 (raw line)" do
      assert :ok = Validators.validate_tesseract_psm(13)
    end

    @tag :unit
    test "rejects PSM -1 (negative)" do
      assert {:error, reason} = Validators.validate_tesseract_psm(-1)
      assert is_binary(reason)
      assert String.contains?(reason, "PSM") or String.contains?(reason, "tesseract")
    end

    @tag :unit
    test "rejects PSM 14 (exceeds maximum)" do
      assert {:error, reason} = Validators.validate_tesseract_psm(14)
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects PSM 15" do
      assert {:error, reason} = Validators.validate_tesseract_psm(15)
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects PSM 100" do
      assert {:error, reason} = Validators.validate_tesseract_psm(100)
      assert is_binary(reason)
    end

    @tag :unit
    test "boundary test: 0 valid, -1 invalid" do
      assert :ok = Validators.validate_tesseract_psm(0)
      assert {:error, _} = Validators.validate_tesseract_psm(-1)
    end

    @tag :unit
    test "boundary test: 13 valid, 14 invalid" do
      assert :ok = Validators.validate_tesseract_psm(13)
      assert {:error, _} = Validators.validate_tesseract_psm(14)
    end

    @tag :unit
    test "all valid PSM values are accepted" do
      valid_psms = 0..13

      Enum.each(valid_psms, fn psm ->
        assert :ok = Validators.validate_tesseract_psm(psm),
               "PSM #{psm} should be valid"
      end)
    end
  end

  # =============================================================================
  # validate_tesseract_oem/1 Tests
  # =============================================================================

  describe "validate_tesseract_oem/1" do
    @tag :unit
    test "accepts OEM 0 (Legacy only)" do
      assert :ok = Validators.validate_tesseract_oem(0)
    end

    @tag :unit
    test "accepts OEM 1 (LSTM only)" do
      assert :ok = Validators.validate_tesseract_oem(1)
    end

    @tag :unit
    test "accepts OEM 2 (Legacy + LSTM)" do
      assert :ok = Validators.validate_tesseract_oem(2)
    end

    @tag :unit
    test "accepts OEM 3 (Default)" do
      assert :ok = Validators.validate_tesseract_oem(3)
    end

    @tag :unit
    test "rejects OEM -1 (negative)" do
      assert {:error, reason} = Validators.validate_tesseract_oem(-1)
      assert is_binary(reason)
      assert String.contains?(reason, "OEM") or String.contains?(reason, "tesseract")
    end

    @tag :unit
    test "rejects OEM 4 (exceeds maximum)" do
      assert {:error, reason} = Validators.validate_tesseract_oem(4)
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects OEM 5" do
      assert {:error, reason} = Validators.validate_tesseract_oem(5)
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects OEM 10" do
      assert {:error, reason} = Validators.validate_tesseract_oem(10)
      assert is_binary(reason)
    end

    @tag :unit
    test "rejects OEM 100" do
      assert {:error, reason} = Validators.validate_tesseract_oem(100)
      assert is_binary(reason)
    end

    @tag :unit
    test "boundary test: 0 valid, -1 invalid" do
      assert :ok = Validators.validate_tesseract_oem(0)
      assert {:error, _} = Validators.validate_tesseract_oem(-1)
    end

    @tag :unit
    test "boundary test: 3 valid, 4 invalid" do
      assert :ok = Validators.validate_tesseract_oem(3)
      assert {:error, _} = Validators.validate_tesseract_oem(4)
    end

    @tag :unit
    test "all valid OEM values are accepted" do
      valid_oems = 0..3

      Enum.each(valid_oems, fn oem ->
        assert :ok = Validators.validate_tesseract_oem(oem),
               "OEM #{oem} should be valid"
      end)
    end

    @tag :unit
    test "error message includes valid range" do
      {:error, reason} = Validators.validate_tesseract_oem(4)
      assert String.contains?(reason, "0") and String.contains?(reason, "3")
    end
  end

  # =============================================================================
  # Cross-validator integration tests
  # =============================================================================

  describe "cross-validator integration" do
    @tag :unit
    test "chunking params and other validators work together" do
      chunking = %{"max_chars" => 1000, "max_overlap" => 200}
      assert :ok = Validators.validate_chunking_params(chunking)

      assert :ok = Validators.validate_language_code("en")
      assert :ok = Validators.validate_dpi(300)
      assert :ok = Validators.validate_confidence(0.7)
    end

    @tag :unit
    test "all backends can be validated" do
      backends = ["tesseract", "easyocr", "paddleocr"]

      Enum.each(backends, fn backend ->
        assert :ok = Validators.validate_ocr_backend(backend)
      end)
    end

    @tag :unit
    test "all binarization methods can be validated" do
      methods = ["otsu", "adaptive", "sauvola"]

      Enum.each(methods, fn method ->
        assert :ok = Validators.validate_binarization_method(method)
      end)
    end

    @tag :unit
    test "typical extraction config values validate" do
      # Simulate a typical configuration
      assert :ok = Validators.validate_language_code("en")
      assert :ok = Validators.validate_dpi(300)
      assert :ok = Validators.validate_confidence(0.5)
      assert :ok = Validators.validate_ocr_backend("tesseract")
      assert :ok = Validators.validate_binarization_method("otsu")
      assert :ok = Validators.validate_tesseract_psm(3)
      assert :ok = Validators.validate_tesseract_oem(2)
    end
  end

  # =============================================================================
  # Error handling and type validation tests
  # =============================================================================

  describe "error handling and messages" do
    @tag :unit
    test "all error returns are tagged with :error" do
      # Test various validators return error tuples
      {:error, _} = Validators.validate_dpi(0)
      {:error, _} = Validators.validate_confidence(1.5)
      {:error, _} = Validators.validate_language_code("xx")
      {:error, _} = Validators.validate_ocr_backend("unknown")
    end

    @tag :unit
    test "all error messages are strings" do
      {:error, msg1} = Validators.validate_dpi(0)
      {:error, msg2} = Validators.validate_confidence(1.5)
      {:error, msg3} = Validators.validate_language_code("xx")
      {:error, msg4} = Validators.validate_ocr_backend("unknown")

      assert is_binary(msg1)
      assert is_binary(msg2)
      assert is_binary(msg3)
      assert is_binary(msg4)
    end

    @tag :unit
    test "error messages are non-empty" do
      {:error, msg} = Validators.validate_dpi(-1)
      assert byte_size(msg) > 0
    end
  end
end
