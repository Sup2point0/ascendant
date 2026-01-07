mod examples;
use examples::*;

use ascendant::*;


fn main()
{
    let grid = grid1();
    println!("\n{grid:?}\n");

    let res = Solver::solve(grid);
    println!("\n{res:?}\n");
}
