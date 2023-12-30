use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    };
    for i in (0..n - 1).rev() {
        for j in 0..2 * n - 1 {
            if s[i][j] != '#' {
                continue;
            }
            let mut ok = false;
            for k in -1..=1 {
                let x = j as i32 + k;
                if !(0..2 * n as i32 - 1).contains(&x) {
                    continue;
                }
                let x = x as usize;
                ok |= s[i + 1][x] == 'X';
            }
            if ok {
                s[i][j] = 'X';
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
