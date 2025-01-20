module type DayType = sig
  (* Each day must expost a function that takes the contents of a file
     and returns an int response.*)
  val process : string -> int
end

module Day1 : DayType = Day1
module Day2 : DayType = Day2
module Day3 : DayType = Day3
module Day4 : DayType = Day4


(* Read a file into a string *)
let read_file file =
  In_channel.with_open_bin file In_channel.input_all
