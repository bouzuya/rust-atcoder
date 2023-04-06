use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    }

    for i in (1..n).rev() {
        for j in 0..2 * n - 1 {
            if s[i][j] == 'X' {
                for dj in -1..=1 {
                    let pi = i - 1;
                    let pj = j as i64 + dj;
                    if (0..2 * n as i64 - 1).contains(&pj) {
                        let pj = pj as usize;
                        if s[pi][pj] == '#' {
                            s[pi][pj] = 'X';
                        }
                    }
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..2 * n - 1 {
            print!("{}", s[i][j]);
        }
        println!();
    }
}
