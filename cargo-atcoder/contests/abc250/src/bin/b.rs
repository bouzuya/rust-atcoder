use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    };
    let mut board = vec![vec!['.'; b * n]; a * n];
    for i_t in 0..n {
        for j_t in 0..n {
            for i in 0..a {
                for j in 0..b {
                    if (i_t + j_t) % 2 != 0 {
                        board[i_t * a + i][j_t * b + j] = '#';
                    }
                }
            }
        }
    }
    for i in 0..a * n {
        for j in 0..b * n {
            print!("{}", board[i][j]);
        }
        println!();
    }
}
