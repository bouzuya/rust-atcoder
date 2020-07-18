use proconio::input;

fn main() {
    input! {
        n: usize,
        a: i64,
        b: i64,
        x: [i64; n]
    };

    let mut ans = 0;
    for (i, x_i) in x.iter().enumerate().skip(1) {
        let d = x_i - x[i - 1];
        ans += std::cmp::min(d * a, b);
    }
    println!("{}", ans);
}
