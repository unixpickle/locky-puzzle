# locky-puzzle

This is my attempt at creating a solver for a twisty puzzle. I don't know what this puzzle is called, and I don't have physical access to it. The [photos](photos) show the puzzle with a V-perm on the yellow layer. A face can turn if all the arrows on that face point in the same rotational direction. In the solved state, opposite faces turn in opposite rotational directions (clockwise and counter-clockwise).

# Results

The solver seems to work, although I can't judge how fast it will be on random state scrambles. According to the solver, this is the scramble from the photos:

```
F U2 F' U2 B' U F U' B U' B' U F' U' B
```

You can find a solution to this scramble yourself by running:

```
$ cargo build --release
$ ./target/release/locky-solve --corner-depth 7 --scramble "F U2 F' U2 B' U F U' B U' B' U F' U' B"
Waiting for heuristic...
Trying depth 0...
Trying depth 1...
...
Trying depth 15...
Found solution: B' U F U' B U B' U F' U' B U2 F U2 F'
```

With a corner index depth of 7, my computer finds the above solution in 4s, and half of this time is spent generating the index. With a depth of 6, it takes 10s.
