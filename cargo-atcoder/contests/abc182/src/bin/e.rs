use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); n],
        cd: [(Usize1, Usize1); m],
    };
    let mut light = vec![vec![false; w]; h];
    // 0: none, 1: light, 2: block
    let mut board = vec![vec![0; w]; h];
    for (a, b) in ab {
        board[a][b] = 1;
    }
    for (c, d) in cd {
        board[c][d] = 2;
    }

    for i in 0..h {
        let mut is_light = false;
        for j in 0..w {
            if board[i][j] == 1 {
                is_light = true;
            } else if board[i][j] == 2 {
                is_light = false;
            }
            if is_light {
                light[i][j] = true;
            }
        }
        let mut is_light = false;
        for j in (0..w).rev() {
            if board[i][j] == 1 {
                is_light = true;
            } else if board[i][j] == 2 {
                is_light = false;
            }
            if is_light {
                light[i][j] = true;
            }
        }
    }

    for j in 0..w {
        let mut is_light = false;
        for i in 0..h {
            if board[i][j] == 1 {
                is_light = true;
            } else if board[i][j] == 2 {
                is_light = false;
            }
            if is_light {
                light[i][j] = true;
            }
        }
        let mut is_light = false;
        for i in (0..h).rev() {
            if board[i][j] == 1 {
                is_light = true;
            } else if board[i][j] == 2 {
                is_light = false;
            }
            if is_light {
                light[i][j] = true;
            }
        }
    }

    let mut count = 0_usize;
    for i in 0..h {
        for j in 0..w {
            if light[i][j] {
                count += 1;
            }
        }
    }

    let ans = count;
    println!("{}", ans);
}
