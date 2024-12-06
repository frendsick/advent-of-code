import gleam/dict.{type Dict}
import gleam/int
import gleam/io
import gleam/list
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
  let coordinates = get_char_coordinates(input)

  // Find XMAS matches
  let matches =
    dict.fold(coordinates, 0, fn(acc, pos, char) {
      case char {
        "X" -> acc + xmas_matches(coordinates, pos)
        _ -> acc
      }
    })
  matches
}

pub fn part2(input: String) -> Int {
  let coordinates = get_char_coordinates(input)

  // Find X_MAS matches
  let matches =
    dict.fold(coordinates, 0, fn(acc, pos, char) {
      case char {
        "A" -> acc + x_mas_matches(coordinates, pos)
        _ -> acc
      }
    })
  matches
}

fn get_char_coordinates(input: String) -> Dict(#(Int, Int), String) {
  // Fill out a dictionary with coordinates and respective characters
  let rows = input |> string.trim_end |> string.split("\n")
  let coordinates =
    rows
    |> list.index_fold(dict.new(), fn(coords, row, y) {
      row
      |> string.to_graphemes
      |> list.index_fold(coords, fn(coords, char, x) {
        dict.insert(coords, #(x, y), char)
      })
    })
  coordinates
}

fn xmas_matches(
  coordinates: Dict(#(Int, Int), String),
  position: #(Int, Int),
) -> Int {
  let #(x, y) = position
  let directions = [
    #(0, -1),
    #(-1, 0),
    #(0, 1),
    #(1, 0),
    #(-1, -1),
    #(1, -1),
    #(-1, 1),
    #(1, 1),
  ]

  let matches =
    directions
    |> list.fold(0, fn(acc, direction) {
      let #(dx, dy) = direction
      let m = #(x + dx, y + dy)
      let a = #(x + dx * 2, y + dy * 2)
      let s = #(x + dx * 3, y + dy * 3)
      case
        dict.get(coordinates, m),
        dict.get(coordinates, a),
        dict.get(coordinates, s)
      {
        Ok("M"), Ok("A"), Ok("S") -> acc + 1
        _, _, _ -> acc
      }
    })
  matches
}

fn x_mas_matches(
  coordinates: Dict(#(Int, Int), String),
  position: #(Int, Int),
) -> Int {
  let #(x, y) = position
  let diagonals = [
    #(x - 1, y - 1),
    #(x + 1, y - 1),
    #(x + 1, y + 1),
    #(x - 1, y + 1),
  ]

  case diagonals |> list.map(fn(pos) { dict.get(coordinates, pos) }) {
    [Ok("M"), Ok("M"), Ok("S"), Ok("S")] -> 1
    [Ok("S"), Ok("M"), Ok("M"), Ok("S")] -> 1
    [Ok("S"), Ok("S"), Ok("M"), Ok("M")] -> 1
    [Ok("M"), Ok("S"), Ok("S"), Ok("M")] -> 1
    _ -> 0
  }
}
