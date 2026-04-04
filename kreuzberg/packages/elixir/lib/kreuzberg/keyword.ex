defmodule Kreuzberg.Keyword do
  @moduledoc """
  Structure representing an extracted keyword with score and algorithm info.

  ## Fields

    * `:text` - The keyword text
    * `:score` - Relevance score (algorithm-dependent)
    * `:algorithm` - Algorithm used for extraction (e.g., "yake", "rake")
    * `:positions` - Optional list of positions where keyword appears
  """

  @type t :: %__MODULE__{
          text: String.t(),
          score: float(),
          algorithm: String.t(),
          positions: list(non_neg_integer()) | nil
        }

  defstruct [
    :positions,
    text: "",
    score: 0.0,
    algorithm: "yake"
  ]

  @doc """
  Create a Keyword struct from a map.

  ## Examples

      iex> Kreuzberg.Keyword.from_map(%{
      ...>   "text" => "elixir",
      ...>   "score" => 0.95,
      ...>   "algorithm" => "yake"
      ...> })
      %Kreuzberg.Keyword{
        text: "elixir",
        score: 0.95,
        algorithm: "yake",
        positions: nil
      }
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      text: data["text"] || "",
      score: (data["score"] || 0.0) * 1.0,
      algorithm: data["algorithm"] || "yake",
      positions: data["positions"]
    }
  end

  @doc """
  Convert a Keyword struct to a map.

  ## Examples

      iex> kw = %Kreuzberg.Keyword{text: "elixir", score: 0.95, algorithm: "yake"}
      iex> Kreuzberg.Keyword.to_map(kw)
      %{
        "text" => "elixir",
        "score" => 0.95,
        "algorithm" => "yake",
        "positions" => nil
      }
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = kw) do
    %{
      "text" => kw.text,
      "score" => kw.score,
      "algorithm" => kw.algorithm,
      "positions" => kw.positions
    }
  end
end
