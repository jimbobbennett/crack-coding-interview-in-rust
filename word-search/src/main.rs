fn main() {
    println!("Hello, world!");
}

fn is_next_letter_adjacent(remaining_word: &str, board: [[char; 6]; 4], previous_position: (usize, usize)) -> bool {

    // do we have any letters remaining? If not, we've found the word!
    if remaining_word.len() == 0 {
        return true;
    }

    // get the letter to find
    let first_letter = &remaining_word.chars().nth(0).unwrap();

    // if the character above is letter
    // Are we on the top row? If so don't check
    if previous_position.0 > 0 {
        let above =  &board[previous_position.0 - 1][previous_position.1];
        if above == first_letter {
            return is_next_letter_adjacent(&remaining_word[1..], 
                board, (previous_position.0 - 1, previous_position.1))
        }
    }

    // if the character below is letter
    // Are we on the bottom row? If so don't check
    if previous_position.0 < (board.len() - 1) {
        let below =  &board[previous_position.0 + 1][previous_position.1];
        if below == first_letter {
            return is_next_letter_adjacent(&remaining_word[1..], 
                board, (previous_position.0 + 1, previous_position.1))
        }
    }

    // if the character left is letter
    // Are we on the first column? If so don't check
    if previous_position.1 > 0 {
        let left =  &board[previous_position.0][previous_position.1 - 1];
        if left == first_letter {
            return is_next_letter_adjacent(&remaining_word[1..], 
                board, (previous_position.0, previous_position.1 - 1))
        }
    }

    // if the character right is letter
    // Are we on the last column? If so don't check
    if previous_position.1 < (board[0].len() - 1) {
        let right =  &board[previous_position.0][previous_position.1 + 1];
        if right == first_letter {
            return is_next_letter_adjacent(&remaining_word[1..], 
                board, (previous_position.0, previous_position.1 + 1))
        }
    }

    // If we don't find the next letter, drop out
    return false;
}

pub fn is_in_board(board: [[char; 6]; 4], word: &str) -> bool {

    let first_letter = &word.chars().nth(0).unwrap();

    for (i, row) in board.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col == first_letter {
                // start a breadth first walk
                if is_next_letter_adjacent(&word[1..], board, (i, j)) {
                    return true;
                }
            }
        }
    }

    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let board = [
            ['A', 'B', 'H', 'E', 'J', 'P'],
            ['F', 'T', 'J', 'L', 'L', 'W'],
            ['V', 'Y', 'X', 'R', 'O', 'Q'],
            ['W', 'V', 'U', 'I', 'L', 'W']
        ];

        let word = "HELLO";

        assert!(is_in_board(board, word));
    }

    #[test]
    fn test_2() {
        let board = [
            ['A', 'B', 'H', 'J', 'J', 'P'],
            ['F', 'T', 'J', 'N', 'D', 'W'],
            ['V', 'D', 'L', 'R', 'O', 'W'],
            ['W', 'V', 'R', 'I', 'V', 'W']
        ];

        let word = "WORLD";

        assert!(is_in_board(board, word));
    }

    #[test]
    fn test_3() {
        let board = [
            ['A', 'B', 'R', 'U', 'S', 'P'],
            ['F', 'T', 'J', 'N', 'D', 'T'],
            ['V', 'D', 'R', 'R', 'G', 'W'],
            ['W', 'V', 'R', 'I', 'V', 'W']
        ];

        let word = "RUST";

        assert!(!is_in_board(board, word));
    }
}