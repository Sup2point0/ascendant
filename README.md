<h1 align="center"> <code> ascendant </code> </h1>

An automated solver for [*Skyscrapers*<sup>↗</sup>](https://sup2point0.github.io/skyscraping/walk/primer) puzzles.


<br>


## Run

Clone the repository:

```bash
> git clone https://github.com/Sup2point0/ascendant
```

You will need [Rust nightly<sup>↗</sup>](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) since the project uses [`generic_const_exprs`](https://github.com/rust-lang/rust/issues/76560).

```rust
ascendant> cargo run
```

Some example puzzles are provided in `src/examples.rs`.


<br>


## Progress

> [!Note]
> Test puzzles sourced from [brainbashers.com<sup>↗</sup>](https://brainbashers.com/skyscrapers.asp), a wonderful puzzles site!

Bear in mind most human-oriented puzzles are 5x5 or 6x6 ;)

`Full` puzzles have clues along every lane in both directions. `Sparse` puzzles have much fewer clues, and are significantly more difficult.

| size | full easy | full hard | sparse |
| :--- | :-------- | :-------- | :----- |
| 4x4  | solvable  | solvable  | solvable
| 5x5  | solvable  | solvable  | solvable
| 6x6  | solvable  | solvable  | –
| 7x7  | solvable  | solvable  |
| 8x8  | –         |
| 9x9  | –         |

### Metrics

| size | difficulty | solved  | time  | as of |
| :--- | :--------- | :------ | :---- | :---- |
| 5x5  | full hard  | 478/487 |       | 2026 January
|      | sparse     | 48/365  | 1.5 s | 2026 January
| 6x6  | full hard  | 305/365 | 0.9 s | 2026 January


<br>


## Algorithm

### Outline
The solving algorithm is iterative – it’ll keep passing over the grid, trying to make logical deductions, until it can no longer find any.

The steps in each pass-through are as follows (they are executed in this order to optimise deduction rate, but the order is irrelevant, really):

- Use the clues to establish a foundation of what digits each cell might take on.
  - For instance, a lane with a clue of $4$ in a $6 \times 6$ puzzle could start with any of $\{ 1, 2, 3 \}$, but not $\{ 4, 5, 6 \}$.
- If a peak has been found, further use the clues to establish ascending sequences of candidates.
- Use the rules of Sudoku to eliminate invalid candidates.
- Find cells which are the only place in their lane for a digit to go.
  - For instance, if the middle cell of a row is the only one with $2$ as a candidate, then we know $2$ must go in that middle cell.
- Find cells with only 1 candidate left – these cells have been solved.

### Philosophy
- The goal is to get as far as possible with purely logical deductions, i.e. no ‘guesswork’ or ‘backtracking’. Even though guesswork [isn’t well-defined<sup>↗</sup>](https://sup2point0.github.io/skyscraping/thoughts/imagination-vs-guesswork)...
- The strategies I plan on implementing are mostly those covered in [Skyscraping<sup>↗</sup>](https://sup2point0.github.io/skyscraping/cases).
- If we stick firmly to this, then we may be able to also generate new skyscrapers puzzles!

### Terminology
- *Skyscraper*: One value in a cell. For instance, the “5-skyscraper”.
- *Lane*: A straight line of cells in the $x$ or $y$ direction.
- *Row*: A horizontal lane, determined by a $y$ index.
- *Col*: A vertical lane, determined by an $x$ index.
- *Peak*: An $N$-skyscraper in an $N \times N$ puzzle. Akin to a ‘maximum’ in mathematics.
- *Sequence*: An ascending sequence of skyscrapers, looking from a clue across the lane towards the peak. Ideally strictly ascending, but not always so.
