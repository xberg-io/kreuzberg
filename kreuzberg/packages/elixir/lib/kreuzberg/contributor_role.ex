defmodule Kreuzberg.ContributorRole do
  @moduledoc """
  JATS contributor with role.

  Matches the Rust `ContributorRole` struct.

  ## Fields

    * `:name` - Contributor name
    * `:role` - Contributor role (e.g., "author", "editor")
  """

  @type t :: %__MODULE__{
          name: String.t(),
          role: String.t() | nil
        }

  defstruct name: "", role: nil

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      name: data["name"] || "",
      role: data["role"]
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = cr) do
    %{
      "name" => cr.name,
      "role" => cr.role
    }
  end
end
