include Aoc2024

let usage_msg = "aoc2024 day"
let day = ref 0

let get_day d =
  day := int_of_string d

let () = Arg.parse [] get_day usage_msg

let fname = "data/day" ^ (string_of_int !day)
let contents = Aoc.read_file fname

module IntMap = Map.Make(Int)

let proc_map: (string -> int) IntMap.t =
  IntMap.of_seq
  (* TIL: @@ is the Haskell $ operator!! *)
  @@ List.to_seq [
         (1, Day1.process);
         (2, Day2.process);
         (3, Day3.process);
         (4, Day4.process)
       ]

let proc =
  match IntMap.find_opt !day proc_map with
    None -> print_endline ("day" ^ string_of_int !day); raise Exit
  | Some p -> p

let () =
  let result = proc contents in
  print_endline (string_of_int result)
