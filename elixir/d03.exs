defmodule D03 do
  def parse_input(input) do
    String.split(input, "\n")
    |> Stream.filter(fn s -> String.length(s) > 0 end)
    |> Enum.map(fn s ->
      Enum.map(to_charlist(s), fn c ->
        case c do
          ?. -> false
          ?# -> true
          _ -> raise "invalid input: expected '.' or '#', got '#{[c]}'"
        end
      end)
    end)
  end

  def run(input) do
    slopes = [{1, 1}, {3, 1}, {5, 1}, {7, 1}, {1, 2}]
    len = length(List.first(input))

    Enum.reduce(slopes, 1, fn {right, down}, total ->
      rows = Stream.take_every(input, down) |> Stream.drop(1)
      cols = 0..(len - 1) |> Stream.cycle() |> Stream.take_every(right) |> Stream.drop(1)

      total *
        (Stream.zip(rows, cols)
         |> Enum.filter(fn {row, col} -> Enum.at(row, col) end)
         |> length())
    end)
  end
end

input = File.read!("inputs/d03") |> D03.parse_input()
res = D03.run(input)
IO.puts(res)
