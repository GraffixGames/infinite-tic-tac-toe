# Infinite Tic-Tac-Toe

## Usage

build with `cargo build --release`

to launch the game, type: `tic-tac-toe [<depth>]` into your terminal.

`depth` is how many levels of tic-tac-toe deep the game goes, default being regular tic-tac-toe.

## examples

### Regular Tic-Tac-Toe

```
$ tic-tac-toe
Base Board
  1 2 3
1 ? ? ?
2 ? ? ?
3 ? ? ?
X, please type the location of your move (row, column): 11

Base Board
  1 2 3
1 X ? ?
2 ? ? ?
3 ? ? ?
O, please type the location of your move (row, column): 12

Base Board
  1 2 3
1 X O ?
2 ? ? ?
3 ? ? ?
X, please type the location of your move (row, column): 3 1

Base Board
  1 2 3
1 X O ?
2 ? ? ?
3 X ? ?
O, please type the location of your move (row, column): 3, 1
That spot is occupied
O, please type the location of your move (row, column): 2,2

Base Board
  1 2 3
1 X O ?
2 ? O ?
3 X ? ?
X, please type the location of your move (row, column): 2 1
X won the game!
```

#### Regular Ultimate Tic-Tac-Toe

```
$ tic-tac-toe 1
Over Board
  1 2 3
1 ? ? ?
2 ? ? ?
3 ? ? ?
X, please type the location of your move (row, column): 1 1
Base Board
  1 2 3
1 ? ? ?
2 ? ? ?
3 ? ? ?
X, please type the location of your move (row, column): 1 1

Over Board
  1 2 3
1 ? ? ?
2 ? ? ?
3 ? ? ?
O, please type the location of your move (row, column): 11
Base Board
  1 2 3
1 X ? ?
2 ? ? ?
3 ? ? ?
O, please type the location of your move (row, column): 22

Over Board
  1 2 3
1 ? ? ?
2 ? ? ?
3 ? ? ?
X, please type the location of your move (row, column): 11
Base Board
  1 2 3
1 X ? ?
2 ? O ?
3 ? ? ?
X, please type the location of your move (row, column): 21

Over Board
  1 2 3
1 ? ? ?
2 ? ? ?
3 ? ? ?
O, please type the location of your move (row, column): 11
Base Board
  1 2 3
1 X ? ?
2 X O ?
3 ? ? ?
O, please type the location of your move (row, column): 12

Over Board
  1 2 3
1 ? ? ?
2 ? ? ?
3 ? ? ?
X, please type the location of your move (row, column): 11
Base Board
  1 2 3
1 X O ?
2 X O ?
3 ? ? ?
X, please type the location of your move (row, column): 31

Over Board
  1 2 3
1 X ? ?
2 ? ? ?
3 ? ? ?
O, please type the location of your move (row, column): 11
That spot is occupied
O, please type the location of your move (row, column): 11
That spot is occupied
O, please type the location of your move (row, column): 22
Base Board
  1 2 3
1 ? ? ?
2 ? ? ?
3 ? ? ?
O, please type the location of your move (row, column): 22

Over Board
  1 2 3
1 X ? ?
2 ? ? ?
3 ? ? ?
X, please type the location of your move (row, column): 21
Base Board
  1 2 3
1 ? ? ?
2 ? ? ?
3 ? ? ?
X, please type the location of your move (row, column): 11

Over Board
  1 2 3
1 X ? ?
2 ? ? ?
3 ? ? ?
O, please type the location of your move (row, column): 22
Base Board
  1 2 3
1 ? ? ?
2 ? O ?
3 ? ? ?
O, please type the location of your move (row, column): 11

Over Board
  1 2 3
1 X ? ?
2 ? ? ?
3 ? ? ?
X, please type the location of your move (row, column): 21
Base Board
  1 2 3
1 X ? ?
2 ? ? ?
3 ? ? ?
X, please type the location of your move (row, column): 21

Over Board
  1 2 3
1 X ? ?
2 ? ? ?
3 ? ? ?
O, please type the location of your move (row, column): 22
Base Board
  1 2 3
1 O ? ?
2 ? O ?
3 ? ? ?
O, please type the location of your move (row, column): 33

Over Board
  1 2 3
1 X ? ?
2 ? O ?
3 ? ? ?
X, please type the location of your move (row, column): 22
That spot is occupied
X, please type the location of your move (row, column): 21
Base Board
  1 2 3
1 X ? ?
2 X ? ?
3 ? ? ?
X, please type the location of your move (row, column): 31

Over Board
  1 2 3
1 X ? ?
2 X O ?
3 ? ? ?
O, please type the location of your move (row, column): 31
Base Board
  1 2 3
1 ? ? ?
2 ? ? ?
3 ? ? ?
O, please type the location of your move (row, column): 22

Over Board
  1 2 3
1 X ? ?
2 X O ?
3 ? ? ?
X, please type the location of your move (row, column): 31
Base Board
  1 2 3
1 ? ? ?
2 ? O ?
3 ? ? ?
X, please type the location of your move (row, column): 11

Over Board
  1 2 3
1 X ? ?
2 X O ?
3 ? ? ?
O, please type the location of your move (row, column): 33
Base Board
  1 2 3
1 ? ? ?
2 ? ? ?
3 ? ? ?
O, please type the location of your move (row, column): 33

Over Board
  1 2 3
1 X ? ?
2 X O ?
3 ? ? ?
X, please type the location of your move (row, column): 31
Base Board
  1 2 3
1 X ? ?
2 ? O ?
3 ? ? ?
X, please type the location of your move (row, column): 21

Over Board
  1 2 3
1 X ? ?
2 X O ?
3 ? ? ?
O, please type the location of your move (row, column): 33
Base Board
  1 2 3
1 ? ? ?
2 ? ? ?
3 ? ? O
O, please type the location of your move (row, column): 31

Over Board
  1 2 3
1 X ? ?
2 X O ?
3 ? ? ?
X, please type the location of your move (row, column): 31
Base Board
  1 2 3
1 X ? ?
2 X O ?
3 ? ? ?
X, please type the location of your move (row, column): 31

X won the game!
```