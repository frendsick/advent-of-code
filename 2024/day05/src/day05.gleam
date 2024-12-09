import gleam/bool
import gleam/int
import gleam/io
import gleam/list
import gleam/order
import gleam/result
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
  // Split sections
  let assert Ok(#(page_ordering_section, updates_section)) =
    input |> string.trim_end |> string.split_once("\n\n")

  // Parse sections
  let rules = parse_rules(page_ordering_section)
  let updates = parse_updates(updates_section)

  // Filter incorrect updates
  let correct_updates =
    list.filter(updates, fn(update) { is_correct_update(update, rules) })

  // Calculate the sum of middlest numbers
  correct_updates |> list.map(get_middlest_number) |> int.sum
}

pub fn part2(input: String) -> Int {
  // Split sections
  let assert Ok(#(page_ordering_section, updates_section)) =
    input |> string.trim_end |> string.split_once("\n\n")

  // Parse sections
  let rules = parse_rules(page_ordering_section)
  let updates = parse_updates(updates_section)

  // Filter correct updates
  let incorrect_updates =
    list.filter(updates, fn(update) {
      update |> is_correct_update(rules) |> bool.negate
    })

  // Sort the updates according to the rules
  let sorted_updates =
    incorrect_updates
    |> list.map(fn(update) { sort_update(update, rules) })

  // Calculate the sum of middlest numbers
  sorted_updates |> list.map(get_middlest_number) |> int.sum
}

fn parse_rules(section: String) -> List(#(Int, Int)) {
  section
  |> string.split("\n")
  |> list.map(fn(row) {
    let assert Ok(#(left, right)) = row |> string.split_once("|")
    let assert Ok(left) = int.parse(left)
    let assert Ok(right) = int.parse(right)
    #(left, right)
  })
}

fn parse_updates(section: String) -> List(List(Int)) {
  section
  |> string.split("\n")
  |> list.map(parse_update)
}

fn parse_update(row: String) -> List(Int) {
  row
  |> string.split(",")
  |> list.map(fn(number_string) {
    number_string |> int.parse |> result.unwrap(0)
  })
}

fn sort_update(update: List(Int), rules: List(#(Int, Int))) -> List(Int) {
  list.sort(update, fn(a, b) {
    case list.contains(rules, #(b, a)) {
      True -> order.Gt
      False -> order.Lt
    }
  })
}

fn is_correct_update(update: List(Int), rules: List(#(Int, Int))) -> Bool {
  update
  |> list.window_by_2
  |> list.map(fn(page_pair) { is_correct_page_pair(page_pair, rules) })
  |> list.all(fn(x) { x })
}

fn get_middlest_number(update: List(Int)) -> Int {
  let middle_index = list.length(update) / 2
  let assert Ok(middlest) = update |> list.drop(middle_index) |> list.first
  middlest
}

fn is_correct_page_pair(
  page_pair: #(Int, Int),
  rules: List(#(Int, Int)),
) -> Bool {
  let #(left, right) = page_pair
  bool.negate(list.contains(rules, #(right, left)))
}
