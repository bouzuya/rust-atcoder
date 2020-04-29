use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! { s: Bytes };
    let m = 2019;
    // ある桁の数値は 10^i * s[i] と表現できる。
    // 10^i 部分も mod 2019 を取ることで扱える桁数になる。
    // rs[i] 右から i 桁目までの mod ((s[0] + s[1] + ... s[i]) % 2019)
    let rs = std::iter::once(0_usize).chain(s.iter().rev().map(|b| (b - b'0') as usize).scan(
        (1_usize, 0_usize),
        |acc, d| {
            let (b, r) = *acc;
            acc.0 = (b * 10) % m;
            acc.1 = ((d * b) % m + r) % m;
            Some(acc.1)
        },
    ));
    // [i, j] において mod 2019 が 0 になるためには
    // rs[j] - rs[i - 1] = 0 にすれば良い
    // rs[i] = rs[j - 1] になる個数を求めれば良い
    //
    // cs[r] 余り r ごとの rs における出現回数
    let mut cs = vec![0_usize; m];
    for r in rs {
        cs[r] += 1;
    }
    let ans = cs
        .iter()
        .map(|&c| c * c.saturating_sub(1) / 2)
        .sum::<usize>();
    println!("{}", ans);
}
