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

| size | full easy | full hard | sparse |
| :--- | :-------- | :-------- | :----- |
| 4x4  | solvable  | solvable  | solvable
| 5x5  | solvable  | near      | near
| 6x6  | near      | solvable  | –
| 7x7  | near      |           |
| 8x8  | –         |


<br>


## Algorithm

### Outline
The solving algorithm is iterative – it’ll keep passing over the grid, trying to make logical deductions, until it can no longer find any.

The steps are as follows (they are executed in this order, but the order is irrelevant, really):

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
