defmodule Kreuzberg.DbfFieldInfo do
  @moduledoc """
  dBASE field information.

  Matches the Rust `DbfFieldInfo` struct.

  ## Fields

    * `:name` - Field name
    * `:field_type` - Field type (e.g., "Character", "Numeric")
  """

  @type t :: %__MODULE__{
          name: String.t(),
          field_type: String.t()
        }

  defstruct name: "", field_type: ""

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      name: data["name"] || "",
      field_type: data["field_type"] || ""
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = info) do
    %{
      "name" => info.name,
      "field_type" => info.field_type
    }
  end
end
