defmodule Kreuzberg.PstMetadata do
  @moduledoc """
  Outlook PST archive metadata.

  Matches the Rust `PstMetadata` struct.

  ## Fields

    * `:message_count` - Number of messages in the archive
  """

  @type t :: %__MODULE__{
          message_count: non_neg_integer()
        }

  defstruct message_count: 0

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      message_count: data["message_count"] || 0
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = meta) do
    %{
      "message_count" => meta.message_count
    }
  end
end
