defmodule Policy do
  @enforce_keys [:n1, :n2, :letter]
  defstruct [:n1, :n2, :letter]

  def parse(s) do
    [numbers, letter] = String.split(s, " ")

    [n1, n2] = String.split(numbers, "-")
    {n1, _} = Integer.parse(n1)
    {n2, _} = Integer.parse(n2)

    %Policy{n1: n1 - 1, n2: n2 - 1, letter: letter}
  end

  def is_valid?(policy, password) do
    c1 = String.at(password, policy.n1)
    c2 = String.at(password, policy.n2)

    case {c1 == policy.letter, c2 == policy.letter} do
      {true, false} -> true
      {false, true} -> true
      _ -> false
    end
  end
end

defmodule D02 do
  def parse_input(s) do
    String.split(s, "\n")
    |> Stream.filter(fn s -> String.length(s) > 0 end)
    |> Stream.map(fn s ->
      [policy, password] = String.split(s, ": ")
      policy = Policy.parse(policy)
      {policy, password}
    end)
  end

  def run(input) do
    Enum.filter(input, fn {policy, password} -> Policy.is_valid?(policy, password) end)
    |> length()
  end
end

input = File.read!("inputs/d02") |> D02.parse_input()
res = D02.run(input)
IO.puts(res)
