use proconio::input;

fn main() {
    input! {
        v: i64,
        t: i64,
        s: i64,
        d: i64,
    };
    let ans = !(v * t..=v * s).contains(&d);
    println!("{}", if ans { "Yes" } else { "No" });
}
