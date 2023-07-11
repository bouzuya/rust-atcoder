use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let mut count = 0;
    for a in 0..n {
        let b = 0;
        let mut t = vec![vec![' '; n]; n];
        for i in 0..n {
            for j in 0..n {
                t[i][j] = s[(i + a) % n][(j + b) % n];
            }
        }
        let mut ok = true;
        for i in 0..n {
            for j in 0..n {
                if t[i][j] != t[j][i] {
                    ok = false;
                }
            }
        }
        if ok {
            count += 1;
        }
    }
    let ans = count * n;
    println!("{}", ans);
}
