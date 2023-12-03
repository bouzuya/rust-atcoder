use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [Chars; n],
        abcd: [(usize, usize, usize, usize); q],
    };

    let mut s = vec![vec![0_usize; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            s[i][j] = if p[i - 1][j - 1] == 'B' { 1 } else { 0 } + s[i][j - 1] + s[i - 1][j]
                - s[i - 1][j - 1];
        }
    }

    let f = |i: usize, j: usize| -> usize {
        let tl = s[n][n] * (i / n) * (j / n);
        let tr = s[n][j % n] * (i / n);
        let bl = s[i % n][n] * (j / n);
        let br = s[i % n][j % n];
        tl + tr + bl + br
    };
    for (a, b, c, d) in abcd {
        let (c, d) = (c + 1, d + 1);
        let ans = f(a, b) + f(c, d) - f(c, b) - f(a, d);
        println!("{}", ans);
    }
}
