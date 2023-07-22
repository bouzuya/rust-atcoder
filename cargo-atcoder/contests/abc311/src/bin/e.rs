use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        ab: [(Usize1, Usize1); n],
    };
    let mut board = vec![vec![0_usize; w]; h];
    for (a, b) in ab {
        board[a][b] = 1;
    }
    let mut s = vec![vec![0_usize; w + 1]; h + 1];
    for i in 1..=h {
        for j in 1..=w {
            s[i][j] = board[i - 1][j - 1] + s[i][j - 1] + s[i - 1][j] - s[i - 1][j - 1];
        }
    }

    let mut ans = 0_usize;
    for i in 0..h {
        for j in 0..w {
            if board[i][j] > 0 {
                continue;
            }
            let mut ok = 1;
            let mut ng = (h - i).min(w - j) + 1;
            while ng - ok > 1 {
                let mid = ok + (ng - ok) / 2;
                let (i_0, j_0) = (i, j);
                let (i_1, j_1) = (i + mid, j + mid);
                if s[i_0][j_0] - s[i_1][j_0] - s[i_0][j_1] + s[i_1][j_1] > 0 {
                    ng = mid;
                } else {
                    ok = mid;
                }
            }
            ans += ok;
        }
    }

    println!("{}", ans);
}
