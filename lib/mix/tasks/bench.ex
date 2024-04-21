defmodule Mix.Tasks.Bench do
  use Mix.Task

  @impl Mix.Task
  def run(modules) do
    modules
    |> Enum.map(&configure/1)
    |> Benchee.run()
  end

  defp configure(module) do
    [project, problem] = String.split(module, ".", parts: 2)

    data_dir =
      case project do
        "PE" -> "lib/project_euler/data/"
        "AoC" -> "lib/advent_of_code/data/"
      end

    filename =
      (String.split(problem, ".", parts: 2) |> List.first() |> String.downcase()) <> ".txt"

    fun =
      if File.exists?(path = data_dir <> filename) do
        fn -> String.to_atom("Elixir." <> module).p(File.read!(path)) end
      else
        fn -> String.to_atom("Elixir." <> module).p() end
      end

    {module, fun}
  end
end
