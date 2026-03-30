defmodule Kreuzberg.CodeDiagnostic do
  @moduledoc """
  Parse diagnostic (error or warning from tree-sitter).

  ## Fields

    * `:message` - Diagnostic message
    * `:severity` - Severity level (e.g. "error", "warning")
    * `:span` - Source span
  """

  @type t :: %__MODULE__{
          message: String.t(),
          severity: String.t(),
          span: Kreuzberg.CodeSpan.t()
        }

  defstruct message: "",
            severity: "",
            span: %Kreuzberg.CodeSpan{}

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    span =
      case Map.get(data, "span", Map.get(data, :span)) do
        %Kreuzberg.CodeSpan{} = s -> s
        map when is_map(map) -> Kreuzberg.CodeSpan.from_map(map)
        _ -> %Kreuzberg.CodeSpan{}
      end

    %__MODULE__{
      message: Map.get(data, "message", Map.get(data, :message, "")),
      severity: Map.get(data, "severity", Map.get(data, :severity, "")),
      span: span
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = diag) do
    %{
      "message" => diag.message,
      "severity" => diag.severity,
      "span" => Kreuzberg.CodeSpan.to_map(diag.span)
    }
  end
end
