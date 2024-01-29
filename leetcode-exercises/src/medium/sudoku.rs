use std::collections::{HashMap, HashSet};

fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let empty_cell = '.';
    let valid_numbers = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut row_maps = Vec::with_capacity(9);
    let mut column_maps = Vec::with_capacity(9);
    let mut box_maps = Vec::with_capacity(9);

    for _i in 0..9 {
        row_maps.push(HashMap::<char,bool>::new());
        column_maps.push(HashMap::<char,bool>::new());
        box_maps.push(HashMap::<char,bool>::new());
    }

    for (row_index, row) in board.iter().enumerate() {
        for (column_index, item) in row.iter().enumerate() {
            if *item == empty_cell {
                continue;
            }

            if !valid_numbers.contains(item) {
                return false;
            }

            let box_index = (row_index / 3) * 3 + column_index / 3;

            if row_maps[row_index].get(item).is_some() || 
               column_maps[column_index].get(item).is_some() || 
               box_maps[box_index].get(item).is_some() {
                return false;
            }

            row_maps[row_index].insert(*item, true);
            column_maps[column_index].insert(*item, true);
            box_maps[box_index].insert(*item, true);
        }
    }

    return true;
}

// according to the description, each item is either 1-9 or .
// we can test uniqueness by putting the row-number, column-number or box-number pairs to respective hash sets
fn is_valid_sudoku_faster(board: Vec<Vec<char>>) -> bool {
    let valid_numbers = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut rows = HashSet::new();
    let mut columns = HashSet::new();
    let mut boxes = vec![HashSet::new(); 9];

    for row in 0..9 {
        for column in 0..9 {
            let item = board[row][column];
            if item >= '0' && item <= '9' {
                if !rows.insert((row, item)) {
                    return false;
                }

                if !columns.insert((column, item)) {
                    return false;
                }

                if !boxes[3 * (row / 3) + column / 3].insert(item) {
                    return false;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_board() {
        assert_eq!(true, is_valid_sudoku(vec![vec!['5','3','.','.','7','.','.','.','.'],vec!['6','.','.','1','9','5','.','.','.'],vec!['.','9','8','.','.','.','.','6','.'],vec!['8','.','.','.','6','.','.','.','3'],vec!['4','.','.','8','.','3','.','.','1'],vec!['7','.','.','.','2','.','.','.','6'],vec!['.','6','.','.','.','.','2','8','.'],vec!['.','.','.','4','1','9','.','.','5'],vec!['.','.','.','.','8','.','.','7','9']]));
        assert_eq!(true, is_valid_sudoku_faster(vec![vec!['5','3','.','.','7','.','.','.','.'],vec!['6','.','.','1','9','5','.','.','.'],vec!['.','9','8','.','.','.','.','6','.'],vec!['8','.','.','.','6','.','.','.','3'],vec!['4','.','.','8','.','3','.','.','1'],vec!['7','.','.','.','2','.','.','.','6'],vec!['.','6','.','.','.','.','2','8','.'],vec!['.','.','.','4','1','9','.','.','5'],vec!['.','.','.','.','8','.','.','7','9']]));
    }

    #[test]
    fn invalid_board() {
        assert_eq!(false, is_valid_sudoku(vec![vec!['8','3','.','.','7','.','.','.','.'],vec!['6','.','.','1','9','5','.','.','.'],vec!['.','9','8','.','.','.','.','6','.'],vec!['8','.','.','.','6','.','.','.','3'],vec!['4','.','.','8','.','3','.','.','1'],vec!['7','.','.','.','2','.','.','.','6'],vec!['.','6','.','.','.','.','2','8','.'],vec!['.','.','.','4','1','9','.','.','5'],vec!['.','.','.','.','8','.','.','7','9']]));
        assert_eq!(false, is_valid_sudoku_faster(vec![vec!['8','3','.','.','7','.','.','.','.'],vec!['6','.','.','1','9','5','.','.','.'],vec!['.','9','8','.','.','.','.','6','.'],vec!['8','.','.','.','6','.','.','.','3'],vec!['4','.','.','8','.','3','.','.','1'],vec!['7','.','.','.','2','.','.','.','6'],vec!['.','6','.','.','.','.','2','8','.'],vec!['.','.','.','4','1','9','.','.','5'],vec!['.','.','.','.','8','.','.','7','9']]));
    }
}
