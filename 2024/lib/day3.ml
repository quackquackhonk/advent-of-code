type instruction =
  Do
| Dont
| Mul of int * int

type parser = {
    muls: (int * int) list;
    enabled: bool;
  }

let sum_muls p =
  let mul_tup (l, r) = l * r in
  List.map mul_tup p.muls
  |> List.fold_left (+) 0


let instruction_regex =
  let mul_regex = Re.Emacs.re {|mul(\([0-9]+\),\([0-9]+\))|} in
  let do_regex = Re.Emacs.re {|do()|} in
  let dont_regex = Re.Emacs.re {|don't()|} in
  Re.alt [mul_regex; do_regex; dont_regex] |> Re.compile

let group_to_instr g =
  let instr_str =  Re.Group.get g 0 in
  let kind_str = List.hd @@ String.split_on_char '(' instr_str in
  (* let _ = print_endline kind_str in *)
  match kind_str with
    "do" -> Do
  | "don't" -> Dont
  | "mul" ->
     let l = int_of_string @@ Re.Group.get g 1 in
     let r = int_of_string @@ Re.Group.get g 2 in
     Mul (l, r)
  | _ -> raise Exit


let process_group par group =
  match group_to_instr group with
    Do -> { muls = par.muls; enabled = true }
  | Dont -> { muls = par.muls; enabled = false}
  | Mul(l, r) -> {
      muls = if par.enabled
             then (l, r) :: par.muls
             else par.muls;
      enabled = par.enabled
    }

let process contents =
  let p = { muls = []; enabled = true } in
  Re.all instruction_regex contents
  |> List.fold_left process_group p
  |> sum_muls


let%test _ =
  let case = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))" in
  process case = 161

let%test _ =
  let case = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))" in
  process case = 48
