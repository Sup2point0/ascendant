mod examples;
use examples::*;

use ascendant::*;


fn main()
{
    let grid = grid_4x4_easy();

    println!("\n{grid:?}\n");

    let res = Solver::solve(grid);

    println!("\n{res:?}\n");
}
