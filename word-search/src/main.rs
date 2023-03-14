fn main() {
    println!("Hello, world!");
}

pub fn is_in_board(board: [[char; 6]; 4], word: &str) -> bool {

    let first_letter = &word.chars().nth(0).unwrap();

    for (i, row) in board.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col == first_letter {
                // start a breadth first walk
            }
        }
    }

    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let board = [
            ['A', 'B', 'H', 'E', 'J', 'P'],
            ['F', 'T', 'J', 'L', 'L', 'W'],
            ['V', 'Y', 'X', 'R', 'O', 'Q'],
            ['W', 'V', 'U', 'I', 'L', 'W']
        ];

        let word = "HELLO";

        assert!(is_in_board(board, word));
    }
}