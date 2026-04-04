defmodule Kreuzberg.JatsMetadata do
  @moduledoc """
  JATS (Journal Article Tag Suite) metadata.

  Matches the Rust `JatsMetadata` struct.

  ## Fields

    * `:copyright` - Copyright statement
    * `:license` - License information
    * `:history_dates` - Publication history dates (e.g., %{"received" => "2024-01-01"})
    * `:contributor_roles` - List of contributors with roles
  """

  @type t :: %__MODULE__{
          copyright: String.t() | nil,
          license: String.t() | nil,
          history_dates: map(),
          contributor_roles: list(Kreuzberg.ContributorRole.t())
        }

  defstruct copyright: nil,
            license: nil,
            history_dates: %{},
            contributor_roles: []

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    contributor_roles =
      (data["contributor_roles"] || [])
      |> Enum.map(fn
        %Kreuzberg.ContributorRole{} = cr -> cr
        map when is_map(map) -> Kreuzberg.ContributorRole.from_map(map)
        _ -> nil
      end)
      |> Enum.reject(&is_nil/1)

    %__MODULE__{
      copyright: data["copyright"],
      license: data["license"],
      history_dates: data["history_dates"] || %{},
      contributor_roles: contributor_roles
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = meta) do
    %{
      "copyright" => meta.copyright,
      "license" => meta.license,
      "history_dates" => meta.history_dates,
      "contributor_roles" => Enum.map(meta.contributor_roles, &Kreuzberg.ContributorRole.to_map/1)
    }
  end
end
