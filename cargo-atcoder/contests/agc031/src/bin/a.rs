use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };

    let mut count = vec![0_usize; 26];
    for s_i in s {
        count[(s_i as u8 - b'a') as usize] += 1;
    }
    let p = 1_000_000_007;
    let mut ans = 1_usize;
    for c in count {
        ans *= c + 1;
        ans %= p;
    }
    ans += p - 1;
    ans %= p;
    println!("{}", ans);
}
