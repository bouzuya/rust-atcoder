use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut ans = 0_usize;
    let mut dp = (0_usize, 0_usize);
    for s_i in s {
        let (z, o) = dp;
        dp = match s_i {
            '0' => (1, z + o),
            '1' => (o, z + 1),
            _ => unreachable!(),
        };
        ans += dp.1;
    }
    println!("{}", ans);
}
