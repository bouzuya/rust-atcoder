use proconio::input;

fn main() {
    input! {
        n: usize,
        mut d: [i64; n],
    };
    d.sort();
    let ans = if d[n / 2 - 1] == d[n / 2] {
        0
    } else {
        d[n / 2] - d[n / 2 - 1]
    };
    println!("{}", ans);
}
