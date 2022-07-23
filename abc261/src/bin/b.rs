use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    };
    let mut ok = true;
    for i in 0..n {
        for j in 0..n {
            match a[i][j] {
                'W' => {
                    if a[j][i] != 'L' {
                        ok = false;
                    }
                }
                'L' => {
                    if a[j][i] != 'W' {
                        ok = false;
                    }
                }
                'D' => {
                    if a[j][i] != 'D' {
                        ok = false;
                    }
                }
                '-' => {}
                _ => unreachable!(),
            }
        }
    }
    let ans = ok;
    println!("{}", if ans { "correct" } else { "incorrect" });
}
