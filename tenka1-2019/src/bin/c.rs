use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    };

    let mut ans = n;
    let mut w = vec![0_usize; n];
    let mut b = vec![0_usize; n];

    let mut count = 0_usize;
    for i in (0..n).rev() {
        if s[i] == '.' {
            count += 1;
        }
        w[i] = count;
    }
    if count == n {
        ans = 0;
    }

    let mut count = 0_usize;
    for i in 0..n {
        if s[i] == '#' {
            count += 1;
        }
        b[i] = count;
    }
    if count == n {
        ans = 0;
    }

    let ans = ans.min(w[0]).min(b[n - 1]).min(
        (0..n)
            .map(|i| w[i] + if i > 0 { b[i - 1] } else { 0 })
            .min()
            .unwrap(),
    );
    println!("{}", ans);
}
