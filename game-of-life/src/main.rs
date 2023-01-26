fn main() {
    println!("Hello, world!");
}

fn should_cell_live(board: &Vec<Vec<CellState>>, row: usize, col: usize) -> bool {
    todo!("We should implement this")
}

fn should_cell_be_born(board: &Vec<Vec<CellState>>, row: usize, col: usize) -> bool {
    todo!("We should implement this")
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellState {
    /// The cell is live
    Alive,
    /// The cell is not live
    Dead,
}

pub fn calc_next_board_state(board: &Vec<Vec<CellState>>) -> Vec<Vec<CellState>> {
    let mut output = Vec::<Vec<CellState>>::new();

    for row in 0..board.len() {
        let mut new_row = Vec::<CellState>::new();

        for col in 0..board[row].len() {
            let cell = board[row][col];
            match cell {
                CellState::Alive => {
                    if should_cell_live(&board, row, col) {
                        new_row.push(CellState::Alive);
                    } else {
                        new_row.push(CellState::Dead);
                    }
                }
                CellState::Dead => {
                    if should_cell_be_born(&board, row, col) {
                        new_row.push(CellState::Alive);
                    } else {
                        new_row.push(CellState::Dead);
                    }
                }
            }
        }

        output.push(new_row);
    }

    output
}

#[cfg(test)]
mod test {
    use super::calc_next_board_state;
    use super::CellState::*;

    #[test]
    fn test_1() {
        #[rustfmt::skip]
        let initial_state = vec![
            vec![Dead,  Alive, Dead], 
            vec![Dead,  Dead,  Alive], 
            vec![Alive, Alive, Alive], 
            vec![Dead,  Dead,  Dead]
        ];

        #[rustfmt::skip]
        let final_state = vec![
            vec![Dead,  Dead,  Dead], 
            vec![Alive, Dead,  Alive], 
            vec![Dead,  Alive, Alive], 
            vec![Dead,  Alive, Dead]
        ];

        let calc_next_state = calc_next_board_state(&initial_state);

        assert_eq!(final_state, calc_next_state);
    }
}
