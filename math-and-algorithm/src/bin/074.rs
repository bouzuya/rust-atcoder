use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    //   1 2 3 4
    // + 0 1 2 3
    // - 3 2 1 0
    //
    // 2 - 1
    // 3 - 1
    // 4 - 1
    // 3 - 2
    // 4 - 2
    // 4 - 3
    let mut ans = 0_i64;
    for (i, a_i) in a.iter().copied().enumerate() {
        let i = i as i64;
        ans += a_i * i - a_i * (n as i64 - 1 - i);
    }
    println!("{}", ans);
}
