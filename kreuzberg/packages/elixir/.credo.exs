%{
  configs: [
    %{
      name: "default",
      files: %{
        included: [
          "lib/",
          "src/",
          "test/",
          "web/",
          "apps/*/lib/",
          "apps/*/src/",
          "apps/*/test/",
          "apps/*/web/"
        ],
        excluded: [~r"/_build/", ~r"/deps/", ~r"/node_modules/"]
      },
      plugins: [],
      requires: [],
      strict: true,
      parse_timeout: 5000,
      color: true,
      checks: %{
        extra: [],
        enabled: [
          #
          # Cyclomatic complexity: raised to 16 to accommodate large case statements
          # in from_map/readable_type functions that have many pattern arms.
          #
          {Credo.Check.Refactor.CyclomaticComplexity, max_complexity: 16}
        ]
      }
    }
  ]
}
