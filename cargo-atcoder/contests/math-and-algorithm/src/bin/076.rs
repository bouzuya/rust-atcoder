use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    a.sort();
    let mut ans = 0_i64;
    for (i, a_i) in a.iter().copied().enumerate() {
        ans += a_i * i as i64 - a_i * (n as i64 - 1 - i as i64);
    }
    println!("{}", ans);
}
