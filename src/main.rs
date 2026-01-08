use ascendant::*;


fn main()
{
    let grid = examples::grid_4x4_easy();

    println!("\n{grid:?}\n");

    let res = Solver::solve(grid);

    println!("\n{res:?}\n");
}
