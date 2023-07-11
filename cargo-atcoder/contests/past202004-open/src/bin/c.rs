use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    };
    let h = n;
    let w = 2 * n - 1;
    for i in (1..h).rev() {
        for j in 0..w {
            if s[i][j] == 'X' {
                for k in -1_i64..=1 {
                    if !(0..w as i64).contains(&(j as i64 + k)) {
                        continue;
                    }
                    let k = (j as i64 + k) as usize;
                    if s[i - 1][k] == '#' {
                        s[i - 1][k] = 'X';
                    }
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..w {
            print!("{}", s[i][j]);
        }
        println!();
    }
}
