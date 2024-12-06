import gleam/int
import gleam/io
import simplifile

pub fn main() {
  let assert Ok(input) = simplifile.read(from: "input.txt")

  let answer_part1 = part1(input) |> int.to_string
  io.println("Part1: " <> answer_part1)

  // let answer_part2 = part2(input) |> int.to_string
  // io.println("Part2: " <> answer_part2)
}

pub fn part1(input: String) -> Int {
  todo
}

pub fn part2(input: String) -> Int {
  todo
}
