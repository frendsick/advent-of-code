import day01.{solve}
import gleeunit
import gleeunit/should
import simplifile

pub fn main() {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn day01_test() {
  let assert Ok(input) = simplifile.read(from: "input_test.txt")
  solve(input) |> should.equal(11)
}
