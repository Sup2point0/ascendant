use ascendant::*;


fn main()
{
    let grid = examples::grid_5x5_hard_1();

    println!("\n{grid:?}\n");

    let res = Solver::solve(grid);

    println!("\n{res:?}\n");
}
