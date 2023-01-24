fn main() {
    println!("Hello, world!");
}

fn should_cell_live(board: &Vec<Vec<usize>>, row: usize, col: usize) -> bool {
    todo!("We should implement this")
}

fn should_cell_be_born(board: &Vec<Vec<usize>>, row: usize, col: usize) -> bool {
    todo!("We should implement this")
}

pub fn calc_next_board_state(board: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut output = Vec::<Vec<usize>>::new();

    for row in 0..board.len() {
        let mut new_row = Vec::<usize>::new();

        for col in 0..board[row].len() {

            let cell = board[row][col];
            if cell == 1 {
                if should_cell_live(&board, row, col) {
                    new_row.push(1);
                }
                else {
                    new_row.push(0);
                }
            }
            else {
                if should_cell_be_born(&board, row, col) {
                    new_row.push(1);
                }
                else {
                    new_row.push(0);
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

    #[test]
    fn test_1() {
        let initial_state = vec![
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![1, 1, 1],
            vec![0, 0, 0],
        ];

        let final_state = vec![
            vec![0, 0, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
            vec![0, 1, 0],
        ];

        let calc_next_state = calc_next_board_state(&initial_state);

        assert_eq!(final_state, calc_next_state);
    }
}