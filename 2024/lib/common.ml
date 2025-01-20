(* string -> string *)
let str_rev x =
  let len = String.length x in
  String.init len (fun n -> String.get x (len - n - 1))

(* string -> char list *)
let explode s = List.init (String.length s) (String.get s)

(* char list -> string *)
let implode cs = String.of_seq @@ List.to_seq cs
