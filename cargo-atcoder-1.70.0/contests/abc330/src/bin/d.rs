use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let mut rows = vec![vec![0; n + 1]; n];
    for i in 0..n {
        for j in 0..n {
            rows[i][j + 1] = rows[i][j] + if s[i][j] == 'o' { 1 } else { 0 };
        }
    }

    let mut rrows = vec![vec![0; n + 1]; n];
    for i in 0..n {
        for j in (0..n).rev() {
            rrows[i][j] = rrows[i][j + 1] + if s[i][j] == 'o' { 1 } else { 0 };
        }
    }

    let mut cols = vec![vec![0; n]; n + 1];
    for j in 0..n {
        for i in 0..n {
            cols[i + 1][j] = cols[i][j] + if s[i][j] == 'o' { 1 } else { 0 };
        }
    }

    let mut rcols = vec![vec![0; n]; n + 1];
    for j in 0..n {
        for i in (0..n).rev() {
            rcols[i][j] = rcols[i + 1][j] + if s[i][j] == 'o' { 1 } else { 0 };
        }
    }

    // for i in 0..=n {
    //     for j in 0..n {
    //         print!("{} ", rcols[i][j]);
    //     }
    //     println!();
    // }

    let mut ans = 0_usize;

    // tl
    for i in 0..n {
        for j in 0..n {
            if s[i][j] != 'o' {
                continue;
            }
            ans += (rows[i][n] - rows[i][j + 1]) * (cols[n][j] - cols[i + 1][j]);
        }
    }
    // tr
    for i in 0..n {
        for j in 1..n {
            if s[i][j] != 'o' {
                continue;
            }
            ans += (rrows[i][0] - rrows[i][j]) * (cols[n][j] - cols[i + 1][j]);
        }
    }
    // br
    for i in 1..n {
        for j in 0..n {
            if s[i][j] != 'o' {
                continue;
            }
            ans += (rrows[i][0] - rrows[i][j]) * (rcols[0][j] - rcols[i][j]);
        }
    }
    // bl
    for i in 1..n {
        for j in 0..n {
            if s[i][j] != 'o' {
                continue;
            }
            ans += (rows[i][n] - rows[i][j + 1]) * (rcols[0][j] - rcols[i][j]);
        }
    }

    println!("{}", ans);
}
