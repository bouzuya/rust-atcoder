use proconio::input;

fn main() {
    input! {
        a: i64,
        _: i64,
        _: i64,
        _: i64,
        e: i64,
        k: i64,
    };
    let ans = e - a <= k;
    println!("{}", if ans { "Yay!" } else { ":(" });
}
