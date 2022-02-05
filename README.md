# TextCuber

Rubik's cube simulator on terminal.

References:
- https://qiita.com/7y2n/items/a840e44dba77b1859352

## Requirements

Rust 2021 Edition

## How to run

```
git https://github.com/misebox/textcuber.git

cargo run
```

## Views

### Perspective View 

![PerspectiveView](images/perspective_view_01.png?raw=true "Text Cuber Perspective View")

### Net View
![NetView](images/net_view_01.png?raw=true "Text Cuber Net View")

## Moves
```
U key -> up
D key -> down
F key -> front
B key -> back
L key -> left
R key -> right

SHIFT + U key -> anti-up
SHIFT + D key -> anti-down
SHIFT + F key -> anti-front
SHIFT + B key -> anti-back
SHIFT + L key -> anti-left
SHIFT + R key -> anti-right
```
Refs about these moves:
- https://solvethecube.com/notation
- https://ruwix.com/the-rubiks-cube/notation/

## Features to be implemented
- X move
- Y move
- Z move
- M move
- E move
- S move
- Parsing the notation like "R U R' U' ..."
- Adding more intuitive view


