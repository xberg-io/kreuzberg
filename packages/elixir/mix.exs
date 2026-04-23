defmodule Kreuzberg.MixProject do
  use Mix.Project

  def project do
    [
      app: :kreuzberg,
      version: "4.9.5",
      elixir: "~> 1.14",
      rustler_crates: [kreuzberg_nif: [mode: :release]],
      description: "High-performance document intelligence library",
      package: package(),
      deps: deps()
    ]
  end

  defp package do
    [
      licenses: ["Elastic-2.0"],
      links: %{"GitHub" => "https://github.com/kreuzberg-dev/kreuzberg"},
      files: ~w(lib native .formatter.exs mix.exs README* checksum-*.exs)
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.37.0", optional: true, runtime: false},
      {:rustler_precompiled, "~> 0.9"},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:ex_doc, "~> 0.40", only: :dev, runtime: false}
    ]
  end
end
