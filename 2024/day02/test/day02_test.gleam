import day02
import gleeunit
import gleeunit/should
import simplifile

pub fn main() {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn part1_test() {
  let assert Ok(input) = simplifile.read(from: "input_test.txt")
  day02.part1(input) |> should.equal(2)
}

pub fn part2_test() {
  let assert Ok(input) = simplifile.read(from: "input_test.txt")
  day02.part2(input) |> should.equal(4)
}
