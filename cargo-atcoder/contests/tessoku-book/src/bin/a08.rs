use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize; w]; h],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    };
    let mut count = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            count[i + 1][j + 1] += count[i + 1][j] + x[i][j];
        }
    }
    for j in 1..=w {
        for i in 0..h {
            count[i + 1][j] += count[i][j];
        }
    }

    for (a, b, c, d) in abcd {
        let ans = count[c][d] + count[a - 1][b - 1] - count[a - 1][d] - count[c][b - 1];
        println!("{}", ans);
    }
}
