import gleam/int
import gleam/io
import gleam/list
import gleam/regexp
import gleam/string
import simplifile

pub fn main() {
  let assert Ok(input) = simplifile.read(from: "input.txt")

  let answer_part1 = part1(input) |> int.to_string
  io.println("Part1: " <> answer_part1)
  let answer_part2 = part2(input) |> int.to_string
  io.println("Part2: " <> answer_part2)
}

pub fn part1(input: String) -> Int {
  // Find mul instructions
  let assert Ok(re) = regexp.from_string("mul\\(\\d+,\\d+\\)")
  let matches = regexp.scan(re, input)

  // Calculate sum of mul instructions
  matches
  |> list.fold(from: 0, with: fn(acc, match) {
    acc + calculate_mul(match.content)
  })
}

pub fn part2(input: String) -> Int {
  // Find don't() instructions
  let dont_splits = string.split(input, "don't()")

  // At the beginning of the program, mul instructions are enabled
  let assert Ok(begin) = list.first(dont_splits)

  // Ignore input portions after don't() until do()
  let rest =
    dont_splits
    |> list.drop(1)
    |> list.fold("", fn(acc, split) { acc <> ignore_until_do(split) })

  let significant_input = begin <> rest

  // Find mul instructions from significant input
  let assert Ok(re) = regexp.from_string("mul\\(\\d+,\\d+\\)")
  let matches = regexp.scan(re, significant_input)

  // Calculate sum of mul instructions
  matches
  |> list.fold(from: 0, with: fn(acc, match) {
    acc + calculate_mul(match.content)
  })
}

fn calculate_mul(instruction: String) -> Int {
  // Example: instruction = "mul(4,2)"
  // left = "mul(4", right = "2)"
  let assert Ok(#(left, right)) = instruction |> string.split_once(",")

  // num1 = "4", num2 = "2"
  let assert Ok(#(_, num1)) = left |> string.split_once("(")
  let assert Ok(#(num2, _)) = right |> string.split_once(")")

  // 4 * 2 = 8
  let assert Ok(num1) = num1 |> int.parse
  let assert Ok(num2) = num2 |> int.parse
  num1 * num2
}

fn ignore_until_do(input: String) -> String {
  case string.split_once(input, "do()") {
    Ok(#(_, right)) -> right
    Error(_) -> ""
  }
}
