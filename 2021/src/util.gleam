import gleam/io
import gleam/int
import simplifile as file

pub fn get_input(day: Int) -> Result(String, file.FileError) {
  let filename = "input/" <> int.to_string(day)
  file.read(filename)
}
