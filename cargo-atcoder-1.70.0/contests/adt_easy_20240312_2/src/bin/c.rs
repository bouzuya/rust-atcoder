use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        d: usize,
        s: [Chars; n],
    };
    let mut max = 0_usize;
    let mut count = 0_usize;
    for j in 0..d {
        let mut all = true;
        for i in 0..n {
            if s[i][j] == 'x' {
                all = false;
                break;
            }
        }
        if all {
            count += 1;
            max = max.max(count);
        } else {
            count = 0;
        }
    }
    let ans = max;
    println!("{}", ans);
}
