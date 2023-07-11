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
        let mut ans = 0_usize;
        let mut cv = vec![0_usize; p];
        cv[0] = 1;
        let mut r = 0_usize;
        let mut b = 1_usize;
        for d in dv.iter().rev() {
            r = (r + ((d * b) % p)) % p;
            b = (b * 10) % p;
            ans += cv[r];
            cv[r] += 1;
        }
        println!("{}", ans);
    }
}
