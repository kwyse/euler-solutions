defmodule AoC.Y23D1 do
  defmodule P1 do
    defmodule Iterative do
      def p(_data),
        do: nil
    end
  end

  defmodule P2 do
    defmodule Regex do
      def p(data),
        do:
          data
          |> String.split("\n", trim: true)
          |> Enum.map(fn line ->
            values =
              Enum.reduce(0..(String.length(line) - 1), [], fn i, acc ->
                str = String.slice(line, i..(String.length(line) - 1))
                acc ++ [parse(str)]
              end)
              |> Enum.filter(&(&1 != nil))

            Integer.parse(List.first(values) <> List.last(values)) |> elem(0)
          end)
          |> Enum.sum()

      def parse(str) do
        cond do
          String.starts_with?(str, ["one", "1"]) -> "1"
          String.starts_with?(str, ["two", "2"]) -> "2"
          String.starts_with?(str, ["three", "3"]) -> "3"
          String.starts_with?(str, ["four", "4"]) -> "4"
          String.starts_with?(str, ["five", "5"]) -> "5"
          String.starts_with?(str, ["six", "6"]) -> "6"
          String.starts_with?(str, ["seven", "7"]) -> "7"
          String.starts_with?(str, ["eight", "8"]) -> "8"
          String.starts_with?(str, ["nine", "9"]) -> "9"
          true -> nil
        end
      end
    end

    defmodule Trie do
      def new,
        do:
          %{}
          |> insert("one", "1")
          |> insert("two", "2")
          |> insert("three", "3")
          |> insert("four", "4")
          |> insert("five", "5")
          |> insert("six", "6")
          |> insert("seven", "7")
          |> insert("eight", "8")
          |> insert("nine", "9")
          |> insert_literals(["1", "2", "3", "4", "5", "6", "7", "8", "9"])

      def get(trie, char), do: Map.fetch(trie, char) |> handle_fetch(trie)

      defp insert(trie, word, value) do
        insert_char(trie, String.graphemes(word), value)
      end

      defp insert_literals(trie, literals) do
        Enum.reduce(literals, trie, &Map.put(&2, &1, %{end: &1}))
      end

      defp insert_char(trie, [], value), do: Map.put(trie, :end, value)

      defp insert_char(trie, [head | tail], value) do
        sub_trie = Map.get(trie, head, %{}) |> insert_char(tail, value)
        Map.put(trie, head, sub_trie)
      end

      defp handle_fetch({:ok, %{end: value}}, _), do: {:end, value}
      defp handle_fetch({:ok, sub_trie}, _), do: {:continue, sub_trie}
      defp handle_fetch(:error, trie), do: {:not_found, trie}
    end

    defmodule TrieIterative do
      def p(data),
        do:
          data
          |> String.split("\n", trim: true)
          |> Enum.map(&parse_line/1)
          |> Enum.sum()

      def parse_line(line) do
        chars = String.graphemes(line)
        length = String.length(line) - 1
        trie = Trie.new()

        Enum.reduce(0..length, {}, fn i, values ->
          new_val =
            case Trie.get(trie, Enum.at(chars, i)) do
              {:end, value} -> value
              {:continue, sub_trie} -> traverse(sub_trie, Enum.slice(chars, i + 1, length))
              _ -> nil
            end

          if new_val != nil do
            case values do
              {} -> {new_val}
              {first} -> {first, new_val}
              {first, _} -> {first, new_val}
            end
          else
            values
          end
        end)
        |> case do
          {first, second} -> Integer.parse(first <> second) |> elem(0)
          {value} -> Integer.parse(value <> value) |> elem(0)
        end
      end

      defp traverse(trie, [c | rem]) do
        case Trie.get(trie, c) do
          {:end, value} -> value
          {:continue, sub_trie} -> traverse(sub_trie, rem)
          _ -> nil
        end
      end

      defp traverse(_, []), do: nil
    end
  end
end
