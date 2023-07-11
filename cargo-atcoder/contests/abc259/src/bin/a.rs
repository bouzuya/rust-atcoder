use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
        x: i64,
        t: i64,
        d: i64,
    };

    if (x..=n).contains(&m) {
        println!("{}", t);
        return;
    }

    let i = t - x * d;
    let ans = i + m.min(x) * d;
    println!("{}", ans);
}
