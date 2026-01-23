use crate::*;


impl<const N: usize> Solver<N>
{
    /// Find cells in a lane that can be solved and turn them from `Cell::Pencil` to `Cell::Solved`.
    pub fn pinpoint_cells_in_lane(mut lane: [&mut Cell<N>; N]) -> bool
    {
        let mut did_deduce = false;

        for (digit, indices) in Grid::occurrences(&lane) {
            if indices.len() == 1 {
                let idx = indices.into_iter().next().unwrap();

                if let cell@Cell::Pencil(_) = &mut lane[idx] {
                    **cell = Cell::Solved(digit);
                    did_deduce = true;
                }
            }
        }

        for cell in lane {
            if let Cell::Pencil(digits) = cell
            && let Some(d) = digits.single()
            {
                *cell = Cell::Solved(d);
                did_deduce = true;
            }
        }

        did_deduce
    }
}
