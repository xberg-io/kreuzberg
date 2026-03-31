defmodule Kreuzberg.DbfMetadata do
  @moduledoc """
  dBASE (DBF) file metadata.

  Matches the Rust `DbfMetadata` struct.

  ## Fields

    * `:record_count` - Number of records
    * `:field_count` - Number of fields
    * `:fields` - List of field definitions
  """

  @type t :: %__MODULE__{
          record_count: non_neg_integer(),
          field_count: non_neg_integer(),
          fields: list(Kreuzberg.DbfFieldInfo.t())
        }

  defstruct record_count: 0,
            field_count: 0,
            fields: []

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    fields =
      (data["fields"] || [])
      |> Enum.map(fn
        %Kreuzberg.DbfFieldInfo{} = f -> f
        map when is_map(map) -> Kreuzberg.DbfFieldInfo.from_map(map)
        _ -> nil
      end)
      |> Enum.reject(&is_nil/1)

    %__MODULE__{
      record_count: data["record_count"] || 0,
      field_count: data["field_count"] || 0,
      fields: fields
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = meta) do
    %{
      "record_count" => meta.record_count,
      "field_count" => meta.field_count,
      "fields" => Enum.map(meta.fields, &Kreuzberg.DbfFieldInfo.to_map/1)
    }
  end
end
