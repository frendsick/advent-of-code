import gleam/dict.{type Dict}
import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile

type Tile {
  Empty
  Obstruction
  Visited
  GuardUp
  GuardDown
  GuardLeft
  GuardRight
}

pub fn main() {
  let assert Ok(input) = simplifile.read(from: "input.txt")

  let answer_part1 = part1(input) |> int.to_string
  io.println("Part1: " <> answer_part1)

  let answer_part2 = part2(input) |> int.to_string
  io.println("Part2: " <> answer_part2)
}

pub fn part1(input: String) -> Int {
  // Parse map from input
  let map = parse_map(input)

  // Simulate guard's route
  let simulated_map = simulate_guard_route(map)

  // Calculate how many tiles were visited
  simulated_map |> dict.filter(fn(_, tile) { tile == Visited }) |> dict.size
}

pub fn part2(input: String) -> Int {
  todo
}

fn parse_map(input: String) -> Dict(#(Int, Int), Tile) {
  let rows = input |> string.trim |> string.split("\n")
  let map =
    list.index_fold(rows, dict.new(), fn(acc, row, y) {
      row
      |> string.to_graphemes
      |> list.index_fold(acc, fn(acc, char, x) {
        let tile = char_to_tile(char)
        dict.insert(acc, #(x, y), tile)
      })
    })
  map
}

fn simulate_guard_route(map: Dict(#(Int, Int), Tile)) -> Dict(#(Int, Int), Tile) {
  // Guard is removed when he moves out of bounds
  case find_guard(map) {
    Ok(#(tile, x, y)) -> guard_step(map, #(x, y), tile) |> simulate_guard_route
    Error(Nil) -> map
  }
}

fn find_guard(map: Dict(#(Int, Int), Tile)) -> Result(#(Tile, Int, Int), Nil) {
  map
  |> dict.filter(fn(_, tile) {
    case tile {
      GuardUp -> True
      GuardDown -> True
      GuardLeft -> True
      GuardRight -> True
      _ -> False
    }
  })
  |> dict.to_list
  |> list.map(fn(position) {
    let tile = position.1
    let x = position.0.0
    let y = position.0.1
    #(tile, x, y)
  })
  |> list.first
}

fn guard_step(
  map: Dict(#(Int, Int), Tile),
  position: #(Int, Int),
  tile: Tile,
) -> Dict(#(Int, Int), Tile) {
  case tile {
    GuardUp -> try_step_up(map, position)
    GuardDown -> try_step_down(map, position)
    GuardLeft -> try_step_left(map, position)
    GuardRight -> try_step_right(map, position)
    _ -> panic
  }
}

fn try_step_up(
  map: Dict(#(Int, Int), Tile),
  position: #(Int, Int),
) -> Dict(#(Int, Int), Tile) {
  let #(x, y) = position
  case dict.get(map, #(x, y - 1)) {
    Ok(Empty) -> step_up(map, position)
    Ok(Visited) -> step_up(map, position)
    Ok(Obstruction) -> step_right(map, position)
    Ok(_) -> panic
    Error(_) -> dict.insert(map, position, Visited)
  }
}

fn try_step_down(
  map: Dict(#(Int, Int), Tile),
  position: #(Int, Int),
) -> Dict(#(Int, Int), Tile) {
  let #(x, y) = position
  case dict.get(map, #(x, y + 1)) {
    Ok(Empty) -> step_down(map, position)
    Ok(Visited) -> step_down(map, position)
    Ok(Obstruction) -> step_left(map, position)
    Ok(_) -> panic
    Error(_) -> dict.insert(map, position, Visited)
  }
}

fn try_step_left(
  map: Dict(#(Int, Int), Tile),
  position: #(Int, Int),
) -> Dict(#(Int, Int), Tile) {
  let #(x, y) = position
  case dict.get(map, #(x - 1, y)) {
    Ok(Empty) -> step_left(map, position)
    Ok(Visited) -> step_left(map, position)
    Ok(Obstruction) -> step_up(map, position)
    Ok(_) -> panic
    Error(_) -> dict.insert(map, position, Visited)
  }
}

fn try_step_right(
  map: Dict(#(Int, Int), Tile),
  position: #(Int, Int),
) -> Dict(#(Int, Int), Tile) {
  let #(x, y) = position
  case dict.get(map, #(x + 1, y)) {
    Ok(Empty) -> step_right(map, position)
    Ok(Visited) -> step_right(map, position)
    Ok(Obstruction) -> step_down(map, position)
    Ok(_) -> panic
    Error(_) -> dict.insert(map, position, Visited)
  }
}

fn step_up(
  map: Dict(#(Int, Int), Tile),
  position: #(Int, Int),
) -> Dict(#(Int, Int), Tile) {
  let #(x, y) = position
  map
  |> dict.insert(position, Visited)
  |> dict.insert(#(x, y - 1), GuardUp)
}

fn step_down(
  map: Dict(#(Int, Int), Tile),
  position: #(Int, Int),
) -> Dict(#(Int, Int), Tile) {
  let #(x, y) = position
  map
  |> dict.insert(position, Visited)
  |> dict.insert(#(x, y + 1), GuardDown)
}

fn step_left(
  map: Dict(#(Int, Int), Tile),
  position: #(Int, Int),
) -> Dict(#(Int, Int), Tile) {
  let #(x, y) = position
  map
  |> dict.insert(position, Visited)
  |> dict.insert(#(x - 1, y), GuardLeft)
}

fn step_right(
  map: Dict(#(Int, Int), Tile),
  position: #(Int, Int),
) -> Dict(#(Int, Int), Tile) {
  let #(x, y) = position
  map
  |> dict.insert(position, Visited)
  |> dict.insert(#(x + 1, y), GuardRight)
}

fn char_to_tile(char: String) -> Tile {
  case char {
    "." -> Empty
    "#" -> Obstruction
    "^" -> GuardUp
    "v" -> GuardDown
    "<" -> GuardLeft
    ">" -> GuardRight
    _ -> panic
  }
}
