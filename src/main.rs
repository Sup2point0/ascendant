use ascendant::*;


fn main()
{
    let grids = [
        // examples::grid_5x5_full_easy_1(),
        // examples::grid_5x5_full_hard_1(),
        // examples::grid_5x5_sparse_1(),

        // examples::grid_6x6_full_easy_1(),
        // examples::grid_6x6_full_hard_1(),
        // examples::grid_6x6_sparse_1(),

        // examples::grid_7x7_full_easy_1(),

        examples::grid_8x8_full_easy_1(),
    ];

    for (i, grid) in grids.into_iter().enumerate() {
        println!("solving grid #{}", i+1);
        let res = Solver::solve(grid);
        println!("\n{res:?}\n");
    }
}
