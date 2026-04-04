defmodule E2E.MixProject do
  use Mix.Project

  def project do
    [
      app: :e2e_elixir,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: false,
      deps: deps(),
      elixirc_paths: ["test/support"]
    ]
  end

  def application do
    [extra_applications: [:logger]]
  end

  defp deps do
    [
      {:kreuzberg, path: "../../packages/elixir"}
    ]
  end
end
