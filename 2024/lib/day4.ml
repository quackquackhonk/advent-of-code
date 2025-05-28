module Coord = struct
  type t = { row: int; col: int}

  let make r c = { row = r; col = c }
  let incr_row { row = r; col = c} =
    { row = r + 1; col = c}
  let incr_col { row = r; col = c} =
    { row = r; col = c + 1}
end

module Window = struct
  type t = { start: Coord.t; end': Coord.t }

  let make size =
    let s = Coord.make 0 0 in
    let e = Coord.make size size in
    { start = s; end' = e }
  let shift_left { start = s; end' = e} =
    { start = Coord.incr_col s; end' = Coord.incr_col e }
  let shift_down { start = s; end' = e} =
    { start = Coord.incr_row s; end' = Coord.incr_row e }

  let shift num_rows num_cols window =
    let new_col = window.end'.col + 1 in
    let new_row = window.end'.row + 1 in
    if new_col <= num_cols then Some (shift_left window)
    else if new_row <= num_rows then Some (shift_down window)
    else None

  let extract input window =
    let drop_rows idx _ =
      idx >= window.start.row && idx < window.end'.row
    in
    let drop_cols col_str =
      let s = window.start.col in
      let e = window.end'.col in
      String.sub col_str s (e - s)
    in
    List.filteri drop_rows input
    |> List.map drop_cols
end

let to_find = "XMAS"

let is_word w =
  w = to_find || w = (Common.str_rev to_find)

let find_horz_words input =
  List.length (List.filter is_word input)

let%test _ = find_horz_words ["XMAS"; "...."; "SAMX"; "....";] = 2

let get_char idx str = str.[idx]

let find_vert_words input =
  let vert_slice idx = Common.implode @@ List.map (get_char idx) input in
  let rotated_input = List.init (List.length input) vert_slice in
  find_horz_words rotated_input

let%test _ = find_vert_words ["X.S."; "M.A."; "A.M."; "S.X."] = 2

let find_diag_words input =
  let x = Common.implode @@ List.mapi get_char input in
  let y = Common.implode @@ List.mapi get_char (List.rev input) in
  let words = [x; y] in
  find_horz_words words

let%test _ = find_diag_words ["X..X"; ".MM."; ".AA."; "S..S"] = 2

let find_words input window =
  let input_window = Window.extract input window in
  let r = find_horz_words input_window
          + find_vert_words input_window
          + find_diag_words input_window in
  let _ = print_endline "Window...." in
  let _ = List.iter print_endline input_window in
  let _ = print_endline @@ string_of_int r in
  r


let rec count_words input next win total =
    match win with
      None -> total
    | Some p -> count_words input next (next p) (total + find_words input p)

let count_all_words word input =
  let word_len = String.length word in
  let num_rows = List.length input in
  let num_cols = String.length @@ List.hd input in
  let window = Window.make word_len in
  let next_fun = Window.shift num_rows num_cols in
  count_words input next_fun (Some window) 0

let process contents =
  let lines = String.split_on_char '\n' contents in
  count_all_words to_find lines

let%test _ =
  let r = process {|MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX|} in
  let _ = print_endline @@ string_of_int r in
  r = 18
