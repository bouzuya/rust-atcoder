use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    };
    let ans = (a..=b).contains(&c);
    println!("{}", if ans { "Yes" } else { "No" });
}
