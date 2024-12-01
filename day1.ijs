read =: 1!:1 @ <
lines =: ((LF&E.) (,;._2) ]) @ read f.
table =: ". @ lines f.
table =: |: @ table f.
sort =: /:~ @ { f.
part1 =: +/ @ (| @ ((0&sort) - (1&sort))) @ table f.
count =: +/ @ (="0 _) f.
part2 =: +/ @ (0&{ ([ * count) 1&{) @ table f.
