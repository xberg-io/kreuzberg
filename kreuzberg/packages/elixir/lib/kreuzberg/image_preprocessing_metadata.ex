defmodule Kreuzberg.ImagePreprocessingMetadata do
  @moduledoc """
  Metadata about image preprocessing applied before OCR.

  Matches the Rust `ImagePreprocessingMetadata` struct.

  ## Fields

    * `:original_dimensions` - Original image dimensions as `{width, height}`
    * `:original_dpi` - Original DPI as `{x_dpi, y_dpi}`
    * `:target_dpi` - Target DPI for preprocessing
    * `:scale_factor` - Scale factor applied
    * `:auto_adjusted` - Whether DPI was auto-adjusted
    * `:final_dpi` - Final DPI after adjustment
    * `:new_dimensions` - New dimensions after resize as `{width, height}` or nil
    * `:resample_method` - Resampling method used (e.g., "lanczos3")
    * `:dimension_clamped` - Whether dimensions were clamped to max
    * `:calculated_dpi` - Calculated DPI if auto-detected, or nil
    * `:skipped_resize` - Whether resize was skipped
    * `:resize_error` - Error message if resize failed, or nil
  """

  @type t :: %__MODULE__{
          original_dimensions: {non_neg_integer(), non_neg_integer()},
          original_dpi: {float(), float()},
          target_dpi: integer(),
          scale_factor: float(),
          auto_adjusted: boolean(),
          final_dpi: integer(),
          new_dimensions: {non_neg_integer(), non_neg_integer()} | nil,
          resample_method: String.t(),
          dimension_clamped: boolean(),
          calculated_dpi: integer() | nil,
          skipped_resize: boolean(),
          resize_error: String.t() | nil
        }

  defstruct [
    :new_dimensions,
    :calculated_dpi,
    :resize_error,
    original_dimensions: {0, 0},
    original_dpi: {0.0, 0.0},
    target_dpi: 0,
    scale_factor: 0.0,
    auto_adjusted: false,
    final_dpi: 0,
    resample_method: "",
    dimension_clamped: false,
    skipped_resize: false
  ]

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      original_dimensions: normalize_tuple(data["original_dimensions"], {0, 0}),
      original_dpi: normalize_float_tuple(data["original_dpi"], {0.0, 0.0}),
      target_dpi: data["target_dpi"] || 0,
      scale_factor: (data["scale_factor"] || 0.0) * 1.0,
      auto_adjusted: data["auto_adjusted"] || false,
      final_dpi: data["final_dpi"] || 0,
      new_dimensions: normalize_optional_tuple(data["new_dimensions"]),
      resample_method: data["resample_method"] || "",
      dimension_clamped: data["dimension_clamped"] || false,
      calculated_dpi: data["calculated_dpi"],
      skipped_resize: data["skipped_resize"] || false,
      resize_error: data["resize_error"]
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = meta) do
    %{
      "original_dimensions" => tuple_to_list(meta.original_dimensions),
      "original_dpi" => tuple_to_list(meta.original_dpi),
      "target_dpi" => meta.target_dpi,
      "scale_factor" => meta.scale_factor,
      "auto_adjusted" => meta.auto_adjusted,
      "final_dpi" => meta.final_dpi,
      "new_dimensions" =>
        case meta.new_dimensions do
          nil -> nil
          t -> tuple_to_list(t)
        end,
      "resample_method" => meta.resample_method,
      "dimension_clamped" => meta.dimension_clamped,
      "calculated_dpi" => meta.calculated_dpi,
      "skipped_resize" => meta.skipped_resize,
      "resize_error" => meta.resize_error
    }
  end

  # Serde serializes tuples as arrays: (usize, usize) → [w, h]
  defp normalize_tuple([a, b], _default), do: {a, b}
  defp normalize_tuple({a, b}, _default), do: {a, b}
  defp normalize_tuple(_, default), do: default

  defp normalize_float_tuple([a, b], _default), do: {a * 1.0, b * 1.0}
  defp normalize_float_tuple({a, b}, _default), do: {a * 1.0, b * 1.0}
  defp normalize_float_tuple(_, default), do: default

  defp normalize_optional_tuple(nil), do: nil
  defp normalize_optional_tuple([a, b]), do: {a, b}
  defp normalize_optional_tuple({a, b}), do: {a, b}
  defp normalize_optional_tuple(_), do: nil

  defp tuple_to_list({a, b}), do: [a, b]
end
