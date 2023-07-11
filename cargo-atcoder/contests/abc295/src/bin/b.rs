use proconio::{input, marker::Chars};

fn main() {
    input! {
        r: usize,
        c: usize,
        b: [Chars; r],
    };

    let mut board = vec![vec!['#'; c]; r];
    for i in 0..r {
        for j in 0..c {
            if b[i][j] == '.' {
                board[i][j] = '.';
            } else if b[i][j] == '#' {
                // do nothing
            } else {
                let range = b[i][j].to_string().parse::<i64>().unwrap();
                for k in -range..=range {
                    for l in -range..=range {
                        let (r_2, c_2) = (i as i64 + k, j as i64 + l);
                        if !(0..r as i64).contains(&r_2) || !(0..c as i64).contains(&c_2) {
                            continue;
                        }

                        let (r_1, c_1) = (i as i64, j as i64);
                        if (r_1 - r_2).abs() + (c_1 - c_2).abs() > range {
                            continue;
                        }

                        let (r_2, c_2) = (r_2 as usize, c_2 as usize);
                        board[r_2][c_2] = '.';
                    }
                }
            }
        }
    }
    for i in 0..r {
        for j in 0..c {
            print!("{}", board[i][j]);
        }
        println!();
    }
}
