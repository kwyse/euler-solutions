defmodule PE.P1 do
  defmodule Iterative do
    def p,
      do:
        3..999
        |> Enum.filter(fn n -> rem(n, 3) == 0 || rem(n, 5) == 0 end)
        |> Enum.sum()
  end
end
