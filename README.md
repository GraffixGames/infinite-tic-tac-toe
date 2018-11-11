# Infinite Tic-Tac-Toe

## Usage

build with `cargo build --release` or download from the releases tab

to launch the game, type: `tic-tac-toe [<depth>]` into your terminal.

`depth` is how many levels of tic-tac-toe deep the game goes, default being regular tic-tac-toe. Creation time takes a fair amount of time when using a depth of over 6ish, depending on your system

## examples

### Regular Tic-Tac-Toe

```
$ tic-tac-toe
  1 2 3
1 ? ? ?
2 ? ? ?
3 ? ? ?
X, please type the location of your move (row, column):
```

#### Regular Ultimate Tic-Tac-Toe

```
$ tic-tac-toe 1
  1 2 3
1 ? ? ?
2 ? ? ?
3 ? ? ?
X, please type the location of your move (row, column):
```