use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        d: usize,
        s: [Chars; n],
    };
    let mut count = 0_usize;
    let mut max = 0_usize;
    for i in 0..d {
        let mut ok = true;
        for j in 0..n {
            if s[j][i] == 'x' {
                count = 0;
                ok = false;
                break;
            }
        }
        if ok {
            count += 1;
        }
        max = max.max(count);
    }
    let ans = max;
    println!("{}", ans);
}
