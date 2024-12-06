import day01
import gleeunit
import gleeunit/should
import simplifile

pub fn main() {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn day01_part1_test() {
  let assert Ok(input) = simplifile.read(from: "input_test.txt")
  day01.part1(input) |> should.equal(11)
}

pub fn day01_part2_test() {
  let assert Ok(input) = simplifile.read(from: "input_test.txt")
  day01.part2(input) |> should.equal(31)
}
