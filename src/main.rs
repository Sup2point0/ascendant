use ascendant::*;


fn main()
{
    let grid = examples::grid_5x5_full_hard_1();
    let res = Solver::solve(grid);

    println!("\n{res:?}\n");
}
