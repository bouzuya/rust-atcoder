use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [[usize; w]; h],
        n: usize,
        rc: [(Usize1, Usize1); n],
    };
    for (r, c) in rc {
        if s[r][c] == 0 {
            continue;
        }
        for i in (0..r).rev() {
            s[i + 1][c] = s[i][c];
        }
        s[0][c] = 0;
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}{}", s[i][j], if j == w - 1 { '\n' } else { ' ' });
        }
    }
}
