<h1 align="center"> <code> ascendant </code> </h1>

An automated solver for [*Skyscrapers*<sup>↗</sup>](https://sup2point0.github.io/skyscraping/walk/primer) puzzles, relying only on pure logic.

For a rundown of the algorithm, jump to [§ Algorithm](#algorithm). For an explanation of how Skyscrapers puzzles work, I have a [quickfire explanation on *Skyscraping*<sup>↗</sup>](https://sup2point0.github.io/skyscraping/walk/primer) you can check out.

### Example

<details>
  <summary> <strong>show</strong> </summary>

```hs
solving puzzle from https://www.brainbashers.com/showskyscrapers.asp?date=1217&size=5&diff=2

pass #0:
   |    4    |    3    |    2    |    2    |    1    |
---------------------------------------------------------
 4 | [12345] | [12345] | [12345] | [12345] | [12345] | 1
 3 | [12345] | [12345] | [12345] | [12345] | [12345] | 2
 2 | [12345] | [12345] | [12345] | [12345] | [12345] | 3
 1 | [12345] | [12345] | [12345] | [12345] | [12345] | 2
 3 | [12345] | [12345] | [12345] | [12345] | [12345] | 2
---------------------------------------------------------
   |    2    |    2    |    2    |    1    |    3    |

pass #1:
   |    4    |    3    |    2    |    2    |    1    |
---------------------------------------------------------
 4 | [   12] | [  123] | [ 1234] |    4    |    5    | 1
 3 | [   23] |    4    |    5    | [   12] | [   23] | 2
 2 |    4    |    5    | [ 1234] | [  123] | [  123] | 3
 1 |    5    | [  123] | [  123] | [  123] |    4    | 2
 3 | [  123] | [ 1234] | [ 1234] |    5    | [  123] | 2
---------------------------------------------------------
   |    2    |    2    |    2    |    1    |    3    |

pass #2:
   |    4    |    3    |    2    |    2    |    1    |
---------------------------------------------------------
 4 |    1    |    3    | [  123] |    4    |    5    | 1
 3 | [   23] |    4    |    5    |    1    | [   23] | 2
 2 |    4    |    5    | [  123] | [  123] | [  123] | 3
 1 |    5    | [   12] | [  123] | [  123] |    4    | 2
 3 | [   23] |    2    |    4    |    5    |    1    | 2
---------------------------------------------------------
   |    2    |    2    |    2    |    1    |    3    |

pass #3:
   |    4    |    3    |    2    |    2    |    1    |
---------------------------------------------------------
 4 |    1    |    3    |    2    |    4    |    5    | 1
 3 |    2    |    4    |    5    |    1    |    3    | 2
 2 |    4    |    5    |    1    |    3    |    2    | 3
 1 |    5    |    1    |    3    |    2    |    4    | 2
 3 |    3    |    2    |    4    |    5    |    1    | 2
---------------------------------------------------------
   |    2    |    2    |    2    |    1    |    3    |
```


<br>


## Run

Clone the repository:

```bash
> git clone https://github.com/Sup2point0/ascendant
```

> [!Important]
> You will need [Rust nightly<sup>↗</sup>](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) since the project uses [`generic_const_exprs`](https://github.com/rust-lang/rust/issues/76560).

See the algorithm in action:

```bash
ascendant> cargo run -- solve-one 7 --diff=2 --random-date
```

See what else you can do:

```bash
ascendant> cargo run -- --help
```

</details>

### Features
Fetch puzzles:

```bash
# fetch 6x6 puzzles of all difficulties
ascendant> cargo run -- fetch --sizes 6 --diffs 1 2 3
```

Solve puzzles in bulk and view stats:

```bash
# solve all 4x4, 5x5, 6x6 puzzles of difficulty 3
ascendant> cargo run -- solve-all --sizes 4 5 6 --diffs 3
```

View the steps in solving one puzzle:

```bash
# solve the 7x7 full hard (difficulty 2) puzzle from April 1, showing all solution substeps
ascendant> cargo run -- solve-one 7 --diff=2 --date=0401 --debug
```


<br>


## Progress

> [!Note]
> Test puzzles sourced from [brainbashers.com<sup>↗</sup>](https://brainbashers.com/skyscrapers.asp), a wonderful puzzles site!

`Full` puzzles have clues along every lane in both directions. `Sparse` puzzles have much fewer clues, and are significantly more difficult. Bear in mind most human-oriented puzzles[^human] are sparse 5x5 or 6x6 ;)

[^human]: After many years solving Skyscrapers (on-off, granted), I can speedrun some sparse 5x5, will need to spend maybe a good hour on sparse 6x6, and have only ever solved 1 sparse 7x7, which took many sittings over many days. Ofc, you may find easier large puzzles with specially designed tricks on sites other than Brainbashers, but this is the only site I’ve really used, so it’s all I have to go off =)

| size | difficulty | solved      | time   | as of |
| :--- | :--------- | :---------- | :----- | :---- |
| 4x4  | full       | **365/365** | ~0.5 s | 2026 January
|      | sparse     |   256/365   | ^      | 2026 January
| 5x5  | full easy  | **365/365** | ~1.5 s | 2026 January
|      | full hard  | **365/365** | ^      | 2026 January
|      | sparse     |    93/365   | ^      | 2026 January
| 6x6  | full easy  |   359/365   | ~4.5 s | 2026 January
|      | full hard  |   355/365   | ^      | 2026 January
|      | sparse     |    23/365   | ^      | 2026 January
| 7x7  | full easy  |   331/365   | ~3.5 s | 2026 January
|      | full hard  |   335/365   | ^      | 2026 January
| 8x8  | full easy  |   321/365   | ~7.5 s | 2026 January
|      | full hard  |   310/365   | ^      | 2026 January

> [!Note]
> 7x7 and 8x8 are taking less time currently since I haven't tried the solver on Sparse puzzles for those sizes yet!

### Notes

<details>
  <summary> <strong>show</strong> </summary>

- The algorithm is not strong at all on sparse puzzles, although it is fairly impressive how far it can get on sparse 5x5 puzzles.
  - Only 21/365 on the sparse 6x6 puzzles... 🤣
  - 4x4 sparse puzzles are quite specially intractable, simply down to how barren information is – some puzzles only have, like, 3 clues!
- Despite the increase in information with larger puzzle sizes, the increase in complexity is much more significant.
  - That being said, the algorithm often reaches an *almost*-solved state.
  - I’m currently figuring out how to improve the algorithm to handle these final near-solved states. Many of them need only a single push before the rules of Sudoku can clear the rest of the puzzle!
- I’m still torn over whether to implement a weak version of lane-isolated backtracking, because humans can do this mentally, so it's arguably not beyond the reach of “logic”. But it’s so ambiguous...

</details>


<br>


## Algorithm

I affectionally call it *peak descent*!

### Outline
The solving algorithm is iterative – it’ll keep passing over the grid, trying to make logical deductions, until it can no longer find any.

These are the steps in each pass-through:[^order]

[^order]: The algorithm steps are executed in this order to optimise deduction rate, but you could theoretically use any order.

- **Ascent**: Use the clues to establish a foundation of candidates for each cell.
  - e.g. A lane $`\text{4 | \_ \_ \_ \_ \_ \_ |}`$ in a 6x6 puzzle could start with any of $`[123]`$, but not $`[456]`$.
- **Peak Descent**: Enforce ascending sequences by descending peaks.
  - If a peak has been found in a lane, step down from the peak towards the clue, calculating how many skyscrapers are currently guaranteed to be visible.
  - Subtract this from the clue to find how many *more* skyscrapers should be visible in front of the first peak.
  - Use this to restrict the candidates of the sequence.
    - e.g. In a lane $`\text{4 | \_ \_ 4 \_ 6 5 |}`$ this deduces $`\text{4 | [12] [23] 4 \_ 6 5 |}`$.
    - e.g. In a lane $`\text{3 | \_ \_ \_ 5 \_ 6 |}`$ this deduces $`\text{3 | [4] [123] [123] 5 \_ 6 |}`$.
- **Sudoku**: Eliminate invalid candidates by the rules of Sudoku.
  - e.g. A lane $`\text{| 3 [36] [123] \_ \_ \_ |}`$ can be eliminated to $`\text{| 3 [6] [12] \_ \_ \_ |}`$.
- **Pinpoint**: Mark cells which are the only place in their lane for a digit to go as solved.
  - e.g. A lane $`\text{| [123] [23] [24] [34] 5 6 |}`$ can be solved to $`\text{| 1 [23] [24] [34] 5 6 |}`$.
- **Isolate**: Eliminate candidates outside of ‘isolated’ groups (couples, triplets, etc.)
  - Two $`[12]`$ cells between them are guaranteed to use both $1$ and $2$.
    - We might not know which way round, but we can still deduce that $1$ and $2$ cannot go in any other cells in the lane.
  - This can be thought of as a ‘pseudo-pinpoint’ followed by sudoku elimination.
  - e.g. A lane $`\text{| [12] [12] [123] [26] \_ \_ |}`$ can be eliminated to $`\text{| [12] [12] [3] [6] \_ \_ |}`$.
- **Solve**: Mark cells with only 1 candidate left as solved.

### Terminology
- **Skyscraper**: One number in a cell. For instance, the “5-skyscraper”.
  - **Candidates**: “Pencil marks” to indicate what skyscrapers *could* go in a cell. 
- **Lane**: A straight line of cells in the $`x`$ or $`y`$ direction.
  - **Row**: A horizontal lane.
  - **Col**: A vertical lane.
  - **Half-Lane**: One side of a lane including cells up to the lane peak.
- **Peak**: A skyscraper guaranteed to be visible.[^peak] (akin to a ‘maximum’ in mathematics)
  - **Lane Peak**: An $`N`$-skyscraper in an $`N`$×$`N`$ puzzle.
- **Sequence**: An ascending sequence of skyscrapers, looking from a clue across the lane towards the peak. Ideally strictly ascending, but not always so.

[^peak]: Named this way because, if you were to look at the skyline of skyscrapers, you would see them taller than other buildings!

### Development

Future strategies to implement include:

- Twin/triplet/... elimination: 2 $`[xy]`$ cells eliminate $x$ and $y$ as candidates from all other cells in the lane.
  - (more advanced) Closed set elimination: Any set of cells which *between them* consume $`[xyz...]`$ eliminate $`[xyz]`$ as candidates from all other cells in the lane.
- Solve cells with 2 candidates where one would be visible and the other obscured.

### Philosophy
- The goal is to get as far as possible with purely logical deductions, i.e. no ‘guesswork’ or ‘backtracking’. Even though guesswork [isn’t well-defined<sup>↗</sup>](https://sup2point0.github.io/skyscraping/thoughts/imagination-vs-guesswork)...
- The strategies I plan on implementing are mostly those covered in [Skyscraping<sup>↗</sup>](https://sup2point0.github.io/skyscraping/cases).
- If we stick firmly to this, then we may be able to also generate new skyscrapers puzzles!


<br>
