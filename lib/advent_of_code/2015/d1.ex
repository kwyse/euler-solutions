defmodule AoC.Y15D1 do
  defmodule P1 do
    defmodule Iterative do
      def p(data),
        do:
          String.graphemes(data)
          |> Enum.reduce(0, fn instruction, floor ->
            case instruction do
              "(" -> floor + 1
              ")" -> floor - 1
            end
          end)
    end
  end

  defmodule P2 do
    defmodule Recursive do
      def p(data),
        do:
          String.graphemes(data)
          |> traverse(0, 0)

      defp traverse(_, index, -1), do: index

      defp traverse([instruction | rem], index, floor) do
        floor =
          case instruction do
            "(" -> floor + 1
            ")" -> floor - 1
          end

        traverse(rem, index + 1, floor)
      end
    end
  end
end
