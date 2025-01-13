let parse_line line =
  let to_int_tuple tup = (int_of_string (fst tup), int_of_string (snd tup)) in
  let to_tuple list = (List.nth list 0, List.nth list 1) in
  let remove_empty s = s <> "" in
  String.split_on_char ' ' line
  |> List.filter remove_empty
  |> to_tuple
  |> to_int_tuple


let parse_contents contents =
  let lines = String.split_on_char '\n' contents in
  let pairs = List.map parse_line lines in
  List.split pairs

let distance left right =
  let l = List.sort compare left in
  let r = List.sort compare right in
  let sorted = List.combine l r in
  let tup_dist (l, r) = abs (r - l) in
  List.map tup_dist sorted
  |> List.fold_left (+) 0

module IntMap = Map.Make(Int)

let count_occur n l =
  let p x = x = n in
  List.filter p l
  |> List.length

let similarity left right =
  let memo: int IntMap.t ref = ref IntMap.empty in
  let calc_simularity n =
    match IntMap.find_opt n !memo with
      Some n -> n
    | None -> let c = count_occur n right in
              memo := IntMap.add n c !memo;
              c
  in
  let sims = List.map calc_simularity left in
  let tup_prod (l, r) = l * r in
  List.combine left sims
  |> List.map tup_prod
  |> List.fold_left (+) 0

let process contents =
  let lists = parse_contents contents in
  let left = fst lists in
  let right = snd lists in
  similarity left right

(* Example test from the problem *)
let%test _ =
  let case = {|3   4
4   3
2   5
1   3
3   9
3   3|} in process case = 31
