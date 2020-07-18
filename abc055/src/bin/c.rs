use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
    };

    let s = n;
    let c = m;
    let scc = if s >= c / 2 {
        c / 2
    } else {
        s + (c - 2 * s) / 4
    };
    let ans = scc;
    println!("{}", ans);
}
