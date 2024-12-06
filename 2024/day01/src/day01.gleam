import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  let assert Ok(input) = simplifile.read(from: "input.txt")
  let answer = solve(input) |> int.to_string
  io.println(answer)
}

pub fn solve(input: String) -> Int {
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
