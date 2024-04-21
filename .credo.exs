%{
  configs: [
    %{
      name: "default",
      files: %{
        included: ["lib/"],
        excluded: ["lib/mix"]
      },
      plugins: [],
      requires: [],
      strict: true,
      parse_timeout: 5000,
      color: true,
      checks: %{
        disabled: [
          {Credo.Check.Readability.ModuleDoc, []}
        ]
      }
    }
  ]
}
