use std::usize;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // Assume that every board is a rectangle
    let rows = minefield.len();
    if rows == 0 {
        return vec![];
    }
    let cols = minefield[0].len();
    if cols == 0 {
        return vec![String::from("")];
    }
    let mut board: Vec<Vec<char>> = Vec::new();
    // Iterate every row

    for i in 0..minefield.len() {
        let mut row = Vec::new();
        for j in 0..minefield[0].len() {
            if let Some(c) = minefield[i].chars().nth(j) {
                if c == '*' {
                    row.push(c);
                    println!("Pushing *");
                    continue;
                }
            }
            let min_x: i16 = if j == 0 { 0 } else { -1 };
            let max_x: i16 = if j == cols - 1 { 0 } else { 1 };
            let min_y: i16 = if i == 0 { 0 } else { -1 };
            let max_y: i16 = if i == rows - 1 { 0 } else { 1 };
            
            let mut mines = 0;
            for y in min_y..=max_y {
                let row_num = (i as i16 + y) as usize;
                for x in min_x..=max_x {
                    let col_num = (j as i16 + x) as usize;
                    if x == 0 && y == 0 {
                        continue;
                    }
                    if let Some(c) = minefield[row_num].chars().nth(col_num) {
                        if c == '*' {
                            mines += 1;
                        }
                    }
                }
            }
            println!("Mines in ({}, {}): {}", i, j, mines);
            row.push(if mines == 0 { ' ' } else { (mines as u8 + 48) as char });
        }
        board.push(row);
    }
    println!("{:?}", board);
    
    board.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<String>>()
}

