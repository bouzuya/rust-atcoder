use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [i64; n - 1],
    };
    let mut ans = b[0] + b[n - 1 - 1];
    for (b_i0, b_i1) in b.windows(2).map(|w| match w {
        &[b_i0, b_i1] => (b_i0, b_i1),
        _ => unreachable!(),
    }) {
        ans += std::cmp::min(b_i0, b_i1);
    }
    println!("{}", ans);
}
