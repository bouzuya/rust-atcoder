use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut dp = (0, 0);
    for a_i in a {
        let mut next = (0, 0);
        next.0 = next.0.max(dp.0).max(dp.1);
        next.1 = next.1.max(dp.0 + a_i);
        dp = next;
    }
    let ans = dp.0.max(dp.1);
    println!("{}", ans);
}
