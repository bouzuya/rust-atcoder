use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    };
    let ans = a > b - c;
    println!("{}", if ans { "Takahashi" } else { "Aoki" });
}
