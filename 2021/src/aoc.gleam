import argv
import gleam/io
import gleam/int
import day1

pub fn main() -> Nil {
  case argv.load().arguments {
    [day] -> run_day(day)
    _ -> io.println("Usage: vars get <name>")
  }
}

fn run_day(day: String) -> Nil {
  // parse the day
  let day = case int.parse(day) {
    Ok(d) -> d
    Error(_) -> {
      io.println("Invalid day  \"" <> day <> "\", defaulting to day 1.")
      1
    }
  }

  case day {
    1 -> day1.main()
    _ -> io.println("Unsupported day!")
  }
}
