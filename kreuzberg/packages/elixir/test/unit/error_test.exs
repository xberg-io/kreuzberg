defmodule KreuzbergTest.Unit.ErrorTest do
  @moduledoc """
  Comprehensive tests for the Kreuzberg.Error module.

  Tests error creation, message formatting, and exception behavior across
  all error types and scenarios.
  """

  use ExUnit.Case

  alias Kreuzberg.Error

  describe "new/2 - error creation with message and reason" do
    test "creates error with message and reason" do
      error = Error.new("File not found", :io_error)

      assert error.message == "File not found"
      assert error.reason == :io_error
      assert error.context == nil
    end

    test "handles all error reason types" do
      reasons = [
        :invalid_format,
        :invalid_config,
        :ocr_error,
        :extraction_error,
        :io_error,
        :nif_error,
        :unknown_error
      ]

      Enum.each(reasons, fn reason ->
        error = Error.new("Test error", reason)
        assert error.reason == reason
      end)
    end

    test "creates error with empty message" do
      error = Error.new("", :unknown_error)

      assert error.message == ""
      assert error.reason == :unknown_error
    end

    test "creates error with very long message" do
      long_message = String.duplicate("x", 10_000)
      error = Error.new(long_message, :extraction_error)

      assert error.message == long_message
      assert String.length(error.message) == 10_000
    end

    test "creates error with unicode message" do
      unicode_message = "Error: 文件未找到 ملف غير موجود файл не найден"
      error = Error.new(unicode_message, :io_error)

      assert error.message == unicode_message
    end

    test "creates error with special characters in message" do
      special_message = "Error: !@#$%^&*(){}[]|\\:;\"'<>,.?/~`"
      error = Error.new(special_message, :extraction_error)

      assert error.message == special_message
    end
  end

  describe "new/3 - error creation with context" do
    test "creates error with context map" do
      context = %{"file" => "document.pdf", "format" => "pdf"}
      error = Error.new("Invalid format", :invalid_format, context)

      assert error.message == "Invalid format"
      assert error.reason == :invalid_format
      assert error.context == context
    end

    test "creates error with nil context" do
      error = Error.new("Error message", :unknown_error, nil)

      assert error.message == "Error message"
      assert error.context == nil
    end

    test "handles complex nested context structures" do
      context = %{
        "file_info" => %{
          "name" => "document.pdf",
          "size" => 1024,
          "nested" => %{"deep" => "value"}
        },
        "extraction_config" => %{"extract_images" => true},
        "errors" => ["error1", "error2"]
      }

      error = Error.new("Extraction failed", :extraction_error, context)

      assert error.context == context
      assert error.context["file_info"]["nested"]["deep"] == "value"
    end

    test "handles context with various value types" do
      context = %{
        "string" => "value",
        "integer" => 42,
        "float" => 3.14,
        "boolean" => true,
        "list" => [1, 2, 3],
        "map" => %{"nested" => "data"},
        "null" => nil
      }

      error = Error.new("Test", :unknown_error, context)

      assert error.context["string"] == "value"
      assert error.context["integer"] == 42
      assert error.context["float"] == 3.14
      assert error.context["boolean"] == true
      assert error.context["list"] == [1, 2, 3]
      assert error.context["map"]["nested"] == "data"
      assert error.context["null"] == nil
    end

    test "handles empty context map" do
      error = Error.new("Message", :extraction_error, %{})

      assert error.context == %{}
    end

    test "preserves original context without modification" do
      original_context = %{"key" => "value"}
      error = Error.new("Message", :extraction_error, original_context)

      # Verify context is preserved
      assert error.context == original_context
      # Verify it's the same reference or equal
      assert error.context["key"] == "value"
    end
  end

  describe "to_string/1 - error string conversion" do
    test "converts error without context to string" do
      error = Error.new("File not found", :io_error)
      string = Error.to_string(error)

      assert string == "File not found (io_error)"
    end

    test "includes reason in string representation" do
      error = Error.new("Error message", :invalid_format)
      string = Error.to_string(error)

      assert String.contains?(string, "invalid_format")
    end

    test "converts error with context to string with details" do
      context = %{"details" => "unsupported"}
      error = Error.new("Invalid format", :invalid_format, context)
      string = Error.to_string(error)

      assert String.contains?(string, "Invalid format")
      assert String.contains?(string, "invalid_format")
      assert String.contains?(string, "context:")
    end

    test "handles errors with complex context in string" do
      context = %{
        "file" => "test.pdf",
        "supported" => ["pdf", "docx"],
        "provided" => "xyz"
      }

      error = Error.new("Unsupported format", :invalid_format, context)
      string = Error.to_string(error)

      assert String.contains?(string, "Unsupported format")
      assert String.contains?(string, "context:")
    end

    test "handles empty message in to_string" do
      error = Error.new("", :unknown_error)
      string = Error.to_string(error)

      assert string == " (unknown_error)"
    end

    test "handles very long message in to_string" do
      long_message = String.duplicate("x", 1000)
      error = Error.new(long_message, :extraction_error)
      string = Error.to_string(error)

      assert String.length(string) > 1000
      assert String.contains?(string, "extraction_error")
    end

    test "to_string output is consistent across calls" do
      error = Error.new("Test error", :io_error, %{"key" => "value"})

      string1 = Error.to_string(error)
      string2 = Error.to_string(error)

      assert string1 == string2
    end
  end

  describe "message/1 - exception message callback" do
    test "generates message for error with all fields" do
      context = %{"details" => "some details"}
      error = Error.new("Error occurred", :extraction_error, context)

      message = Error.message(error)

      assert String.contains?(message, "Error occurred")
      assert String.contains?(message, "extraction_error")
      assert String.contains?(message, "context:")
    end

    test "generates message for error without context" do
      error = Error.new("Simple error", :io_error)

      message = Error.message(error)

      assert message == "Simple error (io_error)"
    end

    test "generates message for error with only message" do
      error = %Error{message: "Only message", reason: nil, context: nil}

      message = Error.message(error)

      assert message == "Only message"
    end

    test "generates message for error with only reason" do
      error = %Error{message: nil, reason: :invalid_config, context: nil}

      message = Error.message(error)

      assert message == "invalid_config"
    end

    test "generates default message for empty error" do
      error = %Error{message: nil, reason: nil, context: nil}

      message = Error.message(error)

      assert message == "Kreuzberg error"
    end

    test "formats complex context in message" do
      context = %{
        "file_info" => %{"name" => "doc.pdf"},
        "errors" => ["err1", "err2"]
      }

      error = Error.new("Validation failed", :extraction_error, context)
      message = Error.message(error)

      assert String.contains?(message, "Validation failed")
      assert String.contains?(message, "extraction_error")
      assert String.contains?(message, "context:")
    end

    test "handles message/1 with nil values safely" do
      # Ensure error with nil values doesn't raise
      error = %Error{message: nil, reason: :unknown_error, context: nil}

      message = Error.message(error)

      assert is_binary(message)
      assert message != ""
    end
  end

  describe "exception behavior" do
    test "can raise Kreuzberg.Error exception" do
      assert_raise Error, fn ->
        raise Error, message: "Test exception", reason: :io_error
      end
    end

    test "exception can be caught and inspected" do
      result =
        try do
          raise Error, message: "Caught error", reason: :extraction_error
        rescue
          e in Error ->
            {e.message, e.reason}
        end

      assert result == {"Caught error", :extraction_error}
    end

    test "exception context is accessible when caught" do
      result =
        try do
          context = %{"file" => "test.pdf"}

          raise Error,
            message: "Error with context",
            reason: :invalid_format,
            context: context
        rescue
          e in Error ->
            {e.message, e.reason, e.context}
        end

      assert result == {"Error with context", :invalid_format, %{"file" => "test.pdf"}}
    end

    test "multiple errors can be raised independently" do
      errors =
        Enum.map(1..5, fn i ->
          try do
            raise Error, message: "Error #{i}", reason: :unknown_error
          rescue
            e in Error -> e
          end
        end)

      assert length(errors) == 5

      Enum.with_index(errors, fn error, i ->
        assert error.message == "Error #{i + 1}"
      end)
    end

    test "error exception preserves all fields through raise/rescue" do
      context = %{"key" => "value", "nested" => %{"deep" => 42}}

      caught_error =
        try do
          raise Error,
            message: "Original error",
            reason: :extraction_error,
            context: context
        rescue
          e in Error -> e
        end

      assert caught_error.message == "Original error"
      assert caught_error.reason == :extraction_error
      assert caught_error.context == context
      assert caught_error.context["nested"]["deep"] == 42
    end
  end

  describe "error classification and categorization" do
    test "classifies IO errors" do
      error = Error.new("File not found", :io_error)

      assert error.reason == :io_error
    end

    test "classifies invalid format errors" do
      error = Error.new("Corrupted PDF", :invalid_format)

      assert error.reason == :invalid_format
    end

    test "classifies configuration errors" do
      error = Error.new("Invalid config parameter", :invalid_config)

      assert error.reason == :invalid_config
    end

    test "classifies OCR errors" do
      error = Error.new("OCR timeout", :ocr_error)

      assert error.reason == :ocr_error
    end

    test "classifies extraction errors" do
      error = Error.new("Extraction failed", :extraction_error)

      assert error.reason == :extraction_error
    end

    test "classifies NIF errors" do
      error = Error.new("Native function error", :nif_error)

      assert error.reason == :nif_error
    end

    test "classifies unknown errors" do
      error = Error.new("Unknown error", :unknown_error)

      assert error.reason == :unknown_error
    end
  end

  describe "edge cases and error handling" do
    test "handles error with all possible fields populated" do
      error =
        Error.new(
          "Comprehensive error message",
          :extraction_error,
          %{
            "file" => "test.pdf",
            "size" => 1024,
            "error_details" => %{"code" => 500}
          }
        )

      assert error.message == "Comprehensive error message"
      assert error.reason == :extraction_error
      assert Map.has_key?(error.context, "file")
    end

    test "handles error struct pattern matching" do
      error = Error.new("Test", :io_error, %{"info" => "data"})

      assert %Error{message: message, reason: reason, context: context} = error
      assert message == "Test"
      assert reason == :io_error
      assert context["info"] == "data"
    end

    test "error is a valid exception struct" do
      error = Error.new("Test", :unknown_error)

      assert is_struct(error, Error)
      assert is_exception(error)
    end

    test "exception message delegates to message/1 callback" do
      error = Error.new("Test message", :io_error)

      # Exception.message/1 should call our custom message/1
      assert Exception.message(error) == Error.message(error)
    end
  end

  describe "multiple error instances" do
    test "creates independent error instances" do
      error1 = Error.new("Error 1", :io_error)
      error2 = Error.new("Error 2", :extraction_error)

      assert error1.message != error2.message
      assert error1.reason != error2.reason
      assert error1 != error2
    end

    test "collects multiple errors" do
      errors = [
        Error.new("Error 1", :io_error),
        Error.new("Error 2", :invalid_format),
        Error.new("Error 3", :extraction_error)
      ]

      assert length(errors) == 3
      assert Enum.at(errors, 0).reason == :io_error
      assert Enum.at(errors, 1).reason == :invalid_format
      assert Enum.at(errors, 2).reason == :extraction_error
    end
  end
end
