use ascendant::*;


fn main()
{
    let grid = examples::grid_6x6_full_easy_1();
    let res = Solver::solve(grid);

    println!("\n{res:?}\n");
}
