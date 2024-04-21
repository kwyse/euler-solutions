defmodule AoCTestHelper do
  def f(file_part), do: ("lib/advent_of_code/data/" <> file_part <> ".txt") |> File.read!()
end

defmodule AdventOfCodeTest do
  use ExUnit.Case, async: true
  import AoCTestHelper

  test "Y15.D1.P1 iterative", do: assert(AoC.Y15D1.P1.Iterative.p(f("y15d1")) == 74)
  test "Y15.D1.P2 recursive", do: assert(AoC.Y15D1.P2.Recursive.p(f("y15d1")) == 1795)
  test "Y23.D1.P1 iterative", do: assert(AoC.Y23D1.P1.Iterative.p(f("y23d1")) == 54_388)
  test "Y23.D1.P2 regex", do: assert(AoC.Y23D1.P2.Regex.p(f("y23d1")) == 53_515)
  test "Y23.D1.P2 trie iterative", do: assert(AoC.Y23D1.P2.TrieIterative.p(f("y23d1")) == 53_515)
end

defmodule ProjectEulerTest do
  use ExUnit.Case, async: true

  test "P1 iterative", do: assert(PE.P1.Iterative.p() == 233_168)
end
