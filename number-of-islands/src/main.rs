use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

/// Converts a row and column index to a position in a 1D vector that represents a 2D array
fn get_vec_index_from_row_col(row: usize, col: usize, cols: usize) -> usize {
    ( row * cols) + col
}

/// Checks if a cell is land.
/// 
/// If a cell is land, this is called recursively to check all the cells around to mark every
/// cell on an island as visited by adding the index for visited cells to a hashset
fn check_is_land(islands: &Vec<bool>, visited: &mut HashSet<usize>, row: usize, col: usize, cols: usize, rows: usize) -> bool {
    let vec_index = get_vec_index_from_row_col(row, col, cols);

    let is_island = islands[vec_index];

    if is_island && !visited.contains(&vec_index) {
        visited.insert(vec_index);

        if col > 0 {
            check_is_land(islands, visited, row, col - 1, cols, rows);
        }

        if col < (cols - 1) {
            check_is_land(islands, visited, row, col + 1, cols, rows);
        }

        if row > 0 {
            check_is_land(islands, visited, row - 1, col, cols, rows);
        }

        if row < (rows - 1) {
            check_is_land(islands, visited, row + 1, col, cols, rows);
        }

        return true
    }

    false
}

/// Counts the number of islands.
/// 
/// The algorithm here is:
/// 
/// Iterate through each row and column in the grid
/// If a cell is land, mark it as visited, count it, then visit every cell above, below, left and right of that cell.
/// Any cells around that are land are also marked as visited. This marking happens recursively so all cells
/// on an island are marked as visited, but we only count the first cell on the island
/// Continue iterating through, ignoring any previously visited cells.
fn num_of_islands(islands: Vec<bool>, cols: usize, rows: usize) -> usize {

    let mut visited = HashSet::<usize>::new();
    let mut count_of_islands:usize = 0;

    for row in 0..rows {
        for col in 0..cols {
            if check_is_land(&islands, &mut visited, row, col, cols, rows) {
                count_of_islands += 1;
            }
        }
    }

    count_of_islands
}

#[cfg(test)]
mod tests {
    use super::num_of_islands;

    #[test]
    fn test_1() {
        let islands = vec![
            true, 	true, 	false, 	true, 	false, 	true, 
            true, 	false, 	false, 	true, 	false, 	true, 
            false, 	false, 	false, 	false, 	false, 	true, 
            true, 	true, 	true, 	false, 	false, 	true, 
            false, 	true, 	false, 	false, 	true, 	true, 
            false, 	true, 	true, 	false, 	false, 	true, 
            false, 	true, 	false, 	false, 	false, 	true, 
            true, 	true, 	false, 	false, 	false, 	false, 
            true, 	true, 	false, 	false, 	false, 	false, 
            false, 	false, 	false, 	true, 	false, 	true, 
            false, 	false, 	false, 	false, 	false, 	false, 
            false, 	false, 	false, 	false, 	false, 	false, 
        ];

        let num = num_of_islands(islands, 6, 12);
        assert_eq!(num, 6);
    }

    #[test]
    fn test_2() {
        let islands = vec![
            false, 	false, 	false, 	false, 
            true, 	false, 	false, 	true, 
            false, 	false, 	false, 	false, 
            false, 	false, 	false, 	false, 
            false, 	false, 	true, 	false, 
            false, 	false, 	false, 	false, 
            false, 	false, 	false, 	false, 
            false, 	false, 	false, 	false, 
            false, 	false, 	false, 	false, 
        ];
    
        let num = num_of_islands(islands, 4, 9);
        assert_eq!(num, 3);
    }

    #[test]
    fn test_3() {
        let islands = vec![
            false, 	false, 	false, 	false, 	false, 	false, 
            false, 	false, 	false, 	false, 	false, 	false, 
            false, 	false, 	false, 	false, 	false, 	false, 
            false, 	false, 	false, 	false, 	false, 	false, 
            false, 	false, 	false, 	false, 	false, 	false, 
            false, 	false, 	false, 	false, 	false, 	false, 
            false, 	false, 	false, 	false, 	false, 	false, 
            false, 	false, 	false, 	false, 	false, 	false, 
        ];
    
        let num = num_of_islands(islands, 6, 8);
        assert_eq!(num, 0);
    }

    #[test]
    fn test_4() {
        let islands = vec![
            false, 	false, 	true, 	false, 
            false, 	false, 	false, 	false, 
        ];
    
        let num = num_of_islands(islands, 4, 2);
        assert_eq!(num, 1);
    }

    #[test]
    fn test_5() {
        let islands = vec![
            true, 	true, 	true, 	true, 	true, 	true, 
            true, 	true, 	true, 	true, 	true, 	true, 
            true, 	true, 	true, 	true, 	true, 	true, 
            true, 	false, 	true, 	true, 	true, 	true, 
            true, 	true, 	true, 	true, 	true, 	true, 
            true, 	true, 	false, 	true, 	false, 	true, 
            true, 	true, 	true, 	true, 	true, 	true, 
            true, 	true, 	true, 	true, 	true, 	true, 
        ];
    
        let num = num_of_islands(islands, 6, 8);
        assert_eq!(num, 1);
    }

    #[test]
    fn test_6() {
        let islands = vec![
            false, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 
            false, 	true, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 
            false, 	true, 	false, 	false, 	false, 	true, 	false, 	false, 	false, 	false, 	false, 	false, 
            false, 	true, 	false, 	false, 	false, 	true, 	false, 	false, 	false, 	false, 	true, 	false, 
            false, 	true, 	false, 	false, 	false, 	true, 	false, 	false, 	false, 	false, 	true, 	false, 
            false, 	true, 	false, 	false, 	false, 	true, 	false, 	false, 	false, 	false, 	true, 	false, 
            false, 	true, 	false, 	false, 	true, 	true, 	true, 	false, 	false, 	false, 	false, 	false, 
            false, 	true, 	false, 	false, 	false, 	true, 	false, 	false, 	false, 	false, 	false, 	false, 
            false, 	false, 	false, 	false, 	false, 	true, 	false, 	false, 	false, 	false, 	false, 	false, 
            false, 	false, 	false, 	false, 	false, 	true, 	false, 	false, 	false, 	false, 	false, 	false, 
            true, 	false, 	false, 	false, 	false, 	true, 	true, 	true, 	true, 	false, 	false, 	false, 
            true, 	true, 	true, 	true, 	false, 	true, 	true, 	true, 	true, 	false, 	false, 	false, 
            false, 	false, 	false, 	true, 	false, 	true, 	true, 	true, 	true, 	false, 	false, 	false, 
            false, 	false, 	false, 	true, 	false, 	true, 	true, 	true, 	true, 	false, 	false, 	false, 
            false, 	false, 	false, 	false, 	false, 	true, 	true, 	true, 	true, 	false, 	true, 	false, 
            false, 	false, 	true, 	false, 	false, 	false, 	false, 	false, 	false, 	true, 	true, 	true, 
            false, 	false, 	true, 	true, 	false, 	false, 	false, 	false, 	false, 	false, 	true, 	false, 
            false, 	false, 	true, 	true, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 
            false, 	false, 	true, 	true, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 
            false, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 	false, 
        ];

        let num = num_of_islands(islands, 12, 20);
        assert_eq!(num, 6);
    }

    #[test]
    fn test_7() {
        let islands = vec![
            true
        ];
    
        let num = num_of_islands(islands, 1, 1);
        assert_eq!(num, 1);
    }

    #[test]
    fn test_8() {
        let islands = vec![
            false
        ];
    
        let num = num_of_islands(islands, 0, 0);
        assert_eq!(num, 0);
    }
}

