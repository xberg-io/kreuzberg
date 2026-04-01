defmodule Kreuzberg.OutputFormat do
  @moduledoc """
  Enumeration of output content formats.

  Matches the Rust `OutputFormat` enum.

  ## Values

    * `:plain` - Plain text output
    * `:markdown` - Markdown formatted output
    * `:djot` - Djot formatted output
    * `:html` - HTML formatted output
    * `:json` - JSON tree format with heading-driven sections
    * `:structured` - Structured output
  """

  @type t :: :plain | :markdown | :djot | :html | :json | :structured

  @doc """
  Returns all valid OutputFormat values.

  ## Examples

      iex> Kreuzberg.OutputFormat.values()
      [:plain, :markdown, :djot, :html, :json, :structured]
  """
  @spec values() :: list(t())
  def values, do: [:plain, :markdown, :djot, :html, :json, :structured]
end
