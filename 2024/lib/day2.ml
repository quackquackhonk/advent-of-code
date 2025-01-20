let rec record_diffs record =
  match record with
    x :: y :: rest -> (x - y) :: record_diffs (y :: rest)
  | _ -> []

let same_dir diffs =
  List.for_all (fun x -> x < 0) diffs
  || List.for_all (fun x -> x > 0) diffs

let all_in_range diffs =
  let in_range x = x >= 1 && x <= 3 in
  List.map abs diffs |> List.for_all in_range

let rec list_remove list n =
  if n = 0
  then List.tl list
  else List.hd list :: list_remove (List.tl list) (n - 1)

let is_safe record =
  let diffs = record_diffs record in
  (same_dir diffs && all_in_range diffs)

let dampen record =
  let inner = list_remove record in
  let mapf i _ = inner i in
  let records = List.mapi mapf record in
  List.exists is_safe records

let is_safe_dampened record =
  is_safe record || dampen record

let%test _ = is_safe [7; 6; 4; 2; 1]
let%test _ = is_safe [1; 2; 7; 8; 9] |> not
let%test _ = is_safe [8; 6; 4; 4; 1] |> not
let%test _ = is_safe [1; 3; 6; 7; 9]

let parse_record line =
  String.split_on_char ' ' line
  |> List.map int_of_string

let process contents =
  let lines = String.split_on_char '\n' contents in
  let records = List.map parse_record lines in
  List.filter is_safe_dampened records
  |> List.length

let%test _ =
  let case = {|7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9|} in process case = 4
