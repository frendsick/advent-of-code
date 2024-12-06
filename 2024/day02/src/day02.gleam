import gleam/bool
import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

type Direction {
  Increasing
  Decreasing
}

pub fn main() {
  let assert Ok(input) = simplifile.read(from: "input.txt")

  let answer_part1 = part1(input) |> int.to_string
  io.println("Part1: " <> answer_part1)
  // let answer_part2 = part2(input) |> int.to_string
  // io.println("Part2: " <> answer_part2)
}

pub fn part1(input: String) -> Int {
  // Parse reports from input
  let rows = input |> string.trim_end |> string.split("\n")
  let reports = parse_reports(rows)

  // Check how many reports are safe
  reports
  |> list.map(fn(report) { report |> is_report_safe |> bool.to_int })
  |> int.sum
}

pub fn part2(input: String) -> Int {
  todo
}

fn parse_reports(rows: List(String)) -> List(List(Int)) {
  rows
  |> list.map(fn(row) {
    row
    |> string.split(" ")
    |> list.map(fn(x) { x |> int.parse |> result.unwrap(0) })
  })
}

fn safe_step(before: Int, after: Int, direction: Direction) -> Bool {
  let min = 1
  let max = 3
  let diff = before - after
  case direction {
    Decreasing -> diff >= min && diff <= max
    Increasing -> -diff >= min && -diff <= max
  }
}

fn is_report_safe(report: List(Int)) -> Bool {
  let pairs = report |> list.window_by_2

  let safely_decreasing =
    list.all(pairs, fn(pair) { safe_step(pair.0, pair.1, Decreasing) })

  let safely_increasing =
    list.all(pairs, fn(pair) { safe_step(pair.0, pair.1, Increasing) })

  safely_decreasing || safely_increasing
}
