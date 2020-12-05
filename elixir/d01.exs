defmodule D01 do
  def parse_input(input) do
    String.split(input, "\n")
    |> Stream.filter(fn s -> String.length(s) > 0 end)
    |> Stream.map(&Integer.parse/1)
    |> Stream.map(fn {n, _} -> n end)
  end

  def run(input) do
    input = Enum.with_index(input)

    sp =
      for {n1, i1} <- input, {n2, i2} <- input, {n3, i3} <- input, i1 < i2 && i2 < i3 do
        {n1 + n2 + n3, n1 * n2 * n3}
      end

    {_, p} = Enum.find(sp, fn {s, _} -> s == 2020 end)
    p
  end
end

input = File.read!(List.first(System.argv())) |> D01.parse_input()
res = D01.run(input)
IO.puts(res)
