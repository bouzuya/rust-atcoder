use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abcw: [(usize, usize, usize, usize); n],
        abc: [(usize, usize, usize); m],
    };
    let mut dp = vec![vec![vec![0_usize; 100 + 1]; 100 + 1]; 100 + 1];
    abcw.sort_by_key(|&(_, _, _, w)| w);
    for (a, b, c, w) in abcw.into_iter().rev() {
        for i in a..=100 {
            for j in b..=100 {
                for k in c..=100 {
                    if dp[i][j][k] == 0 {
                        dp[i][j][k] = w;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    for (a, b, c) in abc {
        println!("{}", dp[a][b][c]);
    }
}
