# Notes

### Finding edges

h 6
w 5

west edge if cursor % w == 0
east edge if cursor % w == (w - 1)
north edge if cursor - w < 0
south edge if cursor + w > (h \* w)

### Grid indexing

```
00 01 02 03 04
05 06 07 08 09
10 11 12 13 14
15 16 17 18 19
20 21 22 23 24
25 26 27 28 29
```

### Working out how to 'print' mazes for bin debugging

```
+---+---+---+
|       |   |
+---+   +   +
|   |       |
+   +---+   +
|           |
+---+---+---+
```

### Better ways to iterate over grid (supports sidewinder, generalized binary tree impl)

- need a way to iterate over the cell indices with different directionalities (e.g. ne->sw, se -> nw)

```
00 01 02 03 04
05 06 07 08 09
10 11 12 13 14
15 16 17 18 19
20 21 22 23 24
25 26 27 28 29

ne -> sw
[
  [04..01], [09..05],.., [29..25]
]


sw -> nw
[
  []
]

se -> nw
```
