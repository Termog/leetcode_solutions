use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        //let mut rows: Vec<HashSet<char>> = vec![HashSet::new();9];
        //let mut cols: Vec<HashSet<char>> = vec![HashSet::new();9];;
        //let mut squares: Vec<HashSet<char>> = vec![HashSet::new();9];
        let mut rows: Vec<HashSet<char>> = vec![HashSet::with_capacity(5);9];
        let mut cols: Vec<HashSet<char>> = vec![HashSet::with_capacity(5);9];;
        let mut squares: Vec<HashSet<char>> = vec![HashSet::with_capacity(5);9];
        for i in 0..9 {
            for j in 0..9 {
                let cha = board[i][j];
                if cha == '.' {
                    continue;
                }
                if !(cha >= '1' && cha <= '9') {
                    return false;
                }
                if !rows[i].insert(cha) {
                    return false
                }
                if !cols[j].insert(cha) {
                    return false
                }
                if !squares[(i/3)+((j/3)*3)].insert(cha) {
                    return false
                }
            }
        }
        true
    }
}
