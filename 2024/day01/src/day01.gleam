import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  let assert Ok(input) = simplifile.read(from: "input.txt")
  let answer_part1 = part1(input) |> int.to_string
  let answer_part2 = part2(input) |> int.to_string
  io.println("Part1: " <> answer_part1)
  io.println("Part2: " <> answer_part2)
}

pub fn part1(input: String) -> Int {
  let rows = input |> string.trim_end |> string.split("\n")
  let #(left_numbers, right_numbers) = parse_left_and_right_numbers(rows)

  // Sort both lists
  let left_sorted = left_numbers |> list.sort(by: int.compare)
  let right_sorted = right_numbers |> list.sort(by: int.compare)

  // Calculate the sum of differences
  let differences =
    list.map2(left_sorted, right_sorted, fn(a, b) { int.absolute_value(a - b) })
  differences |> int.sum
}

pub fn part2(input: String) -> Int {
  let rows = input |> string.trim_end |> string.split("\n")
  let #(left_numbers, right_numbers) = parse_left_and_right_numbers(rows)

  // Calculate similarity score
  left_numbers
  |> list.fold(0, fn(acc, number) {
    acc + number * list.count(right_numbers, fn(x) { x == number })
  })
}

fn parse_left_and_right_numbers(rows: List(String)) -> #(List(Int), List(Int)) {
  rows
  |> list.map(fn(row) {
    let assert Ok(#(left, right)) = row |> string.split_once("   ")
    #(
      left |> int.parse |> result.unwrap(0),
      right |> int.parse |> result.unwrap(0),
    )
  })
  |> list.unzip
}
