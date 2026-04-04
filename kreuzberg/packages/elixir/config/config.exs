import Config

config :rustler_precompiled, :force_build,
  kreuzberg: System.get_env("KREUZBERG_BUILD") in ["1", "true"] || Mix.env() in [:test, :dev]
