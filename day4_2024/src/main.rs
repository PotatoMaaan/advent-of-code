const WORD: &str = "XMAS";

fn main() {
    let grid = include_str!("../input_rr")
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = grid.len();

    let mut count = 0;

    // Check horizontally
    for row in 0..rows {
        for col in 0..rows {
            if is_word_found(&grid, row, col, 0, 1) {
                count += 1;
            }
            if is_word_found(&grid, row, col, 0, -1) {
                count += 1;
            }
        }
    }

    // Check vertically
    for col in 0..rows {
        for row in 0..rows {
            if is_word_found(&grid, row, col, 1, 0) {
                count += 1;
            }
            if is_word_found(&grid, row, col, -1, 0) {
                count += 1;
            }
        }
    }

    // Check diagonally
    for row in 0..rows {
        for col in 0..rows {
            if is_word_found(&grid, row, col, 1, 1) {
                count += 1;
            }
            if is_word_found(&grid, row, col, 1, -1) {
                count += 1;
            }
            if is_word_found(&grid, row, col, -1, 1) {
                count += 1;
            }
            if is_word_found(&grid, row, col, -1, -1) {
                count += 1;
            }
        }
    }

    dbg!(&count);
}

fn is_word_found(grid: &[Vec<char>], row: usize, col: usize, row_step: i32, col_step: i32) -> bool {
    if row as i32 + (WORD.len() as i32 - 1) * row_step < 0
        || row as i32 + (WORD.len() as i32 - 1) * row_step >= grid.len() as i32
        || col as i32 + (WORD.len() as i32 - 1) * col_step < 0
        || col as i32 + (WORD.len() as i32 - 1) * col_step >= grid.len() as i32
    {
        return false;
    }

    for (i, ch) in WORD.chars().enumerate() {
        if grid[(row as i32 + i as i32 * row_step) as usize]
            [(col as i32 + i as i32 * col_step) as usize]
            != ch
        {
            return false;
        }
    }

    true
}
