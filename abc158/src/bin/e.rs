use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        p: usize,
        s: Chars,
    };
    let mut dv = vec![0_usize; n];
    for i in 0..n {
        dv[i] = s[i] as usize - '0' as usize;
    }

    if 10 % p == 0 {
        let mut ans = 0;
        for i in 0..n {
            if dv[i] % p == 0 {
                ans += i + 1;
            }
        }
        println!("{}", ans);
    } else {
        let mut ans = 0;
        let mut cv = vec![0; p];
        cv[0] = 1;
        let mut r = 0;
        let mut b = 1;
        for d in dv.iter().rev() {
            r = (r + ((d * b) % p)) % p;
            b = (b * 10) % p;
            cv[r] += 1;
        }
        for c in cv {
            if c == 0 {
                continue;
            }
            ans += c * (c - 1) / 2
        }
        println!("{}", ans);
    }
}
